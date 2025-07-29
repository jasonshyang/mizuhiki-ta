use crate::core::{
    series::Series,
    traits::{Indexable, Numeric},
};

/// RSI configuration with generic alpha type
#[derive(Debug, Clone, Copy)]
pub struct RsiConfig<T> {
    pub period: usize,
    pub alpha: T,
}

impl<T> RsiConfig<T> {
    pub fn new(period: usize, alpha: T) -> Self {
        Self { period, alpha }
    }
}

impl RsiConfig<f32> {
    /// Creates a config with the standard EMA smoothing factor
    pub fn from_period(period: usize) -> RsiConfig<f32> {
        let alpha = 2.0 / (period as f32 + 1.0);
        RsiConfig::new(period, alpha)
    }

    /// Creates a config with Wilder's original smoothing factor
    pub fn from_period_wilder(period: usize) -> RsiConfig<f32> {
        let alpha = 1.0 / period as f32;
        RsiConfig::new(period, alpha)
    }
}

impl RsiConfig<f64> {
    /// Creates a config with the standard EMA smoothing factor
    pub fn from_period(period: usize) -> RsiConfig<f64> {
        let alpha = 2.0 / (period as f64 + 1.0);
        RsiConfig::new(period, alpha)
    }

    /// Creates a config with Wilder's original smoothing factor
    pub fn from_period_wilder(period: usize) -> RsiConfig<f64> {
        let alpha = 1.0 / period as f64;
        RsiConfig::new(period, alpha)
    }
}

impl Default for RsiConfig<f32> {
    fn default() -> Self {
        Self::from_period(14)
    }
}

impl Default for RsiConfig<f64> {
    fn default() -> Self {
        Self::from_period(14)
    }
}

/// RSI calculation result
#[derive(Debug)]
pub struct RsiResult<T, I> {
    pub rsi: Series<T, I>,
    pub avg_gain: Series<T, I>,
    pub avg_loss: Series<T, I>,
}

/// Calculate RSI (Relative Strength Index) for a price series
///
/// RSI is a momentum oscillator that measures the speed and magnitude of price changes.
/// Values range from 0 to 100, with readings above 70 typically considered overbought
/// and readings below 30 considered oversold.
///
/// # Arguments
/// * `prices` - Price series (typically closing prices)
/// * `config` - RSI configuration
///
/// # Example
/// ```rust
/// use mizuhiki_ta::core::series::Series;
/// use mizuhiki_ta::indicators::rsi::{rsi, RsiConfig};
///
/// let prices = Series::from_vec("price".to_string(), vec![
///     100.0, 102.0, 101.0, 103.0, 104.0, 102.0, 105.0, 106.0, 103.0, 107.0
/// ]);
///
/// let config = RsiConfig::<f64>::default();
/// let result = rsi(&prices, config);
/// ```
pub fn rsi<T, I>(prices: &Series<T, I>, config: RsiConfig<T>) -> RsiResult<T, I>
where
    T: Numeric,
    I: Indexable,
{
    // FIXME: for the initial calculation with missing values,
    // ideally we should let user knows (pandas-ta will return NaN)

    // Calculate price changes (diff)
    let changes = prices.diff();

    // Separate gains and losses
    let gains = changes.map(|&change| {
        if change > T::zero() {
            change
        } else {
            T::zero()
        }
    });

    let losses = changes.map(|&change| {
        if change < T::zero() {
            T::zero() - change
        } else {
            T::zero()
        }
    });

    // Calculate exponential moving averages
    let avg_gain = gains.ewm_mean(config.alpha);
    let avg_loss = losses.ewm_mean(config.alpha);

    // Calculate RSI from average gains and losses
    let rsi = calculate_rsi_from_averages(&avg_gain, &avg_loss);

    RsiResult {
        rsi,
        avg_gain,
        avg_loss,
    }
}

/// Helper function to calculate RSI from average gains and losses
fn calculate_rsi_from_averages<T, I>(
    avg_gain: &Series<T, I>,
    avg_loss: &Series<T, I>,
) -> Series<T, I>
where
    T: Numeric,
    I: Indexable,
{
    let gain_values = avg_gain.values();
    let loss_values = avg_loss.values();

    let hundred = T::hundred();
    let fifty = T::fifty();
    let zero = T::zero();

    let rsi_data: Vec<T> = gain_values
        .iter()
        .zip(loss_values.iter())
        .map(|(&gain, &loss)| {
            if gain == zero && loss == zero {
                fifty
            } else {
                hundred * (gain / (gain + loss))
            }
        })
        .collect();

    Series::new(
        format!("{}_rsi", avg_gain.name()),
        rsi_data,
        avg_gain.index().to_vec(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::series::Series;

    fn assert_within_tolerance<T: Numeric>(value: T, expected: T, tolerance: T) {
        assert!(
            (value - expected).abs() < tolerance,
            "Value {:?} not within tolerance of {:?}, expected {:?}",
            value,
            tolerance,
            expected
        );
    }

    #[test]
    fn test_rsi_basic() {
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.21, 46.25, 45.71, 46.45,
            45.78, 45.35, 44.03, 44.18, 44.22, 44.57, 43.42, 42.66, 43.13,
        ];

        // Ran from pandas-ta, the initial 14 values are NaN
        let expected_rsi = vec![
            71.80, 65.19, 65.55, 69.88, 65.45, 54.18, 61.24, 61.69, 52.84, 61.08, 52.19, 47.42,
            36.42, 38.17, 38.66, 42.89, 34.47, 30.25, 35.51,
        ];

        let series = Series::from_vec("close".to_string(), prices);
        let config = RsiConfig::<f64>::new(14, 1.0 / 14.0);
        let result = rsi(&series, config);

        assert_eq!(result.rsi.len(), expected_rsi.len() + 14); // 14 initial NaN values

        for i in 0..expected_rsi.len() {
            let rsi_value = result.rsi.get(i + 14).unwrap();
            assert_within_tolerance(*rsi_value, expected_rsi[i], 0.005);
        }
    }
}
