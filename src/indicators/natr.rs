use crate::core::{
    series::Series,
    traits::{Indexable, Numeric},
};

/// NATR configuration with generic alpha type
#[derive(Debug, Clone, Copy)]
pub struct NatrConfig<T> {
    pub period: usize,
    pub alpha: T,
}

impl<T> NatrConfig<T> {
    pub fn new(period: usize, alpha: T) -> Self {
        Self { period, alpha }
    }
}

impl NatrConfig<f32> {
    /// Creates a config with the standard EMA smoothing factor
    pub fn from_period(period: usize) -> NatrConfig<f32> {
        let alpha = 2.0 / (period as f32 + 1.0);
        NatrConfig::new(period, alpha)
    }

    /// Creates a config with Wilder's original smoothing factor
    pub fn from_period_wilder(period: usize) -> NatrConfig<f32> {
        let alpha = 1.0 / period as f32;
        NatrConfig::new(period, alpha)
    }
}

impl NatrConfig<f64> {
    /// Creates a config with the standard EMA smoothing factor
    pub fn from_period(period: usize) -> NatrConfig<f64> {
        let alpha = 2.0 / (period as f64 + 1.0);
        NatrConfig::new(period, alpha)
    }

    /// Creates a config with Wilder's original smoothing factor
    pub fn from_period_wilder(period: usize) -> NatrConfig<f64> {
        let alpha = 1.0 / period as f64;
        NatrConfig::new(period, alpha)
    }
}

impl Default for NatrConfig<f32> {
    fn default() -> Self {
        Self::from_period(14)
    }
}

impl Default for NatrConfig<f64> {
    fn default() -> Self {
        Self::from_period(14)
    }
}

/// NATR calculation result
#[derive(Debug)]
pub struct NatrResult<T, I> {
    pub natr: Series<T, I>,
    pub true_range: Series<T, I>,
    pub atr: Series<T, I>,
}

/// Calculate NATR (Normalized Average True Range) for OHLC price series
///
/// NATR is a volatility indicator that normalizes ATR as a percentage of the closing price.
/// This makes it comparable across different price levels and instruments. Values typically
/// range from 0% to 10%, with higher values indicating greater volatility.
///
/// NATR = (ATR / Close) * 100
///
/// # Arguments
/// * `high` - High prices  
/// * `low` - Low prices
/// * `close` - Closing prices
/// * `config` - NATR configuration including period and smoothing parameters
///
/// # Example
/// ```
/// use mizuhiki_ta::core::series::Series;
/// use mizuhiki_ta::indicators::natr::{natr, NatrConfig};
///
/// let high = Series::from_vec("high".to_string(), vec![102.0, 104.0, 103.0, 105.0]);
/// let low = Series::from_vec("low".to_string(), vec![99.0, 101.0, 100.0, 102.0]);
/// let close = Series::from_vec("close".to_string(), vec![101.0, 103.0, 102.0, 104.0]);
///
/// let config = NatrConfig::<f64>::default();
/// let result = natr(&high, &low, &close, config);
///
/// // Check volatility level
/// let latest_natr = result.natr.values().last().unwrap();
/// if *latest_natr > 5.0 {
///     println!("High volatility: {:.2}%", latest_natr);
/// }
/// ```
pub fn natr<T, I>(
    high: &Series<T, I>,
    low: &Series<T, I>,
    close: &Series<T, I>,
    config: NatrConfig<T>,
) -> NatrResult<T, I>
where
    T: Numeric,
    I: Indexable,
{
    // Calculate True Range
    let true_range = calculate_true_range(high, low, close);

    // Calculate ATR using exponential moving average
    let atr = true_range.ewm_mean(config.alpha);

    // Calculate NATR = (ATR / Close) * 100
    let natr = calculate_natr_from_atr(&atr, close);

    NatrResult {
        natr,
        true_range,
        atr,
    }
}

/// Helper function to calculate True Range
fn calculate_true_range<T, I>(
    high: &Series<T, I>,
    low: &Series<T, I>,
    close: &Series<T, I>,
) -> Series<T, I>
where
    T: Numeric + PartialOrd,
    I: Indexable,
{
    let high_values = high.values();
    let low_values = low.values();
    let close_values = close.values();

    let mut true_range_data = Vec::new();

    for i in 0..high_values.len() {
        let current_high = high_values[i];
        let current_low = low_values[i];

        let tr = if i == 0 {
            // For the first period, use high - low
            current_high - current_low
        } else {
            let prev_close = close_values[i - 1];

            // True Range = max(high - low, |high - prev_close|, |low - prev_close|)
            let hl = current_high - current_low;
            let hc = if current_high > prev_close {
                current_high - prev_close
            } else {
                prev_close - current_high
            };
            let lc = if current_low > prev_close {
                current_low - prev_close
            } else {
                prev_close - current_low
            };

            // Find maximum of the three values
            let max_hc_lc = if hc > lc { hc } else { lc };
            if hl > max_hc_lc { hl } else { max_hc_lc }
        };

        true_range_data.push(tr);
    }

    Series::new(
        format!("{}_tr", high.name()),
        true_range_data,
        high.index().to_vec(),
    )
}

/// Helper function to calculate NATR from ATR and closing prices
fn calculate_natr_from_atr<T, I>(atr: &Series<T, I>, close: &Series<T, I>) -> Series<T, I>
where
    T: Numeric,
    I: Indexable,
{
    let atr_values = atr.values();
    let close_values = close.values();

    let hundred = T::hundred();
    let zero = T::zero();

    let natr_data: Vec<T> = atr_values
        .iter()
        .zip(close_values.iter())
        .map(|(&atr_val, &close_val)| {
            if close_val != zero {
                (atr_val / close_val) * hundred
            } else {
                zero // Handle division by zero
            }
        })
        .collect();

    Series::new(
        format!("{}_natr", atr.name()),
        natr_data,
        atr.index().to_vec(),
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

    fn get_test_data() -> (Series<f64, usize>, Series<f64, usize>, Series<f64, usize>) {
        let highs = vec![
            48.70, 48.72, 48.90, 48.87, 48.82, 49.05, 49.20, 49.35, 49.92, 50.19, 50.12, 49.66,
            49.88, 50.19, 50.36, 50.57, 50.65, 50.90, 51.12, 51.22, 51.30, 51.18, 50.92, 50.74,
            50.56, 50.67, 50.73, 50.81, 50.94, 51.12, 51.25, 51.36, 51.18, 50.92, 50.65, 50.54,
            50.33, 50.10, 49.91,
        ];

        let lows = vec![
            48.12, 48.14, 48.39, 48.37, 48.24, 48.64, 48.94, 48.86, 49.50, 49.87, 49.20, 48.90,
            49.43, 49.73, 49.26, 49.31, 49.50, 49.72, 50.43, 50.55, 50.68, 50.44, 50.20, 49.93,
            49.84, 49.90, 50.05, 50.18, 50.29, 50.33, 50.48, 50.57, 50.21, 49.89, 49.71, 49.66,
            49.55, 49.38, 49.10,
        ];

        let closes = vec![
            48.16, 48.61, 48.75, 48.63, 48.74, 49.03, 49.07, 49.32, 49.91, 50.13, 49.53, 49.50,
            49.75, 50.03, 49.61, 49.80, 50.20, 50.73, 50.94, 51.08, 50.97, 50.55, 50.42, 50.14,
            50.23, 50.41, 50.53, 50.60, 50.81, 50.95, 51.01, 51.07, 50.65, 50.16, 49.85, 49.77,
            49.66, 49.49, 49.21,
        ];

        let high = Series::from_vec("high".to_string(), highs);
        let low = Series::from_vec("low".to_string(), lows);
        let close = Series::from_vec("close".to_string(), closes);
        (high, low, close)
    }

    #[test]
    fn test_calc_tr() {
        let (high, low, close) = get_test_data();
        let true_range = calculate_true_range(&high, &low, &close);
        let expected_tr = vec![
            0.58, 0.58, 0.51, 0.50, 0.58, 0.41, 0.26, 0.49, 0.60, 0.32, 0.93, 0.76, 0.45, 0.46,
            1.10, 1.26, 1.15, 1.18, 0.69, 0.67, 0.62, 0.74, 0.72, 0.81, 0.72, 0.77, 0.68, 0.63,
            0.65, 0.79, 0.77, 0.79, 0.97, 1.03, 0.94, 0.88, 0.78, 0.72, 0.81,
        ];

        for i in 0..true_range.len() {
            let tr_value = true_range.get(i).unwrap();
            assert_within_tolerance(*tr_value, expected_tr[i], 0.001);
        }
    }

    #[test]
    fn test_calc_atr() {
        let (high, low, close) = get_test_data();
        let true_range = calculate_true_range(&high, &low, &close);
        let atr = true_range.ewm_mean(1.0 / 14.0);

        // From pandas-ta, the first value is NaN
        let expected_atr = vec![
            0.5800, 0.5750, 0.5696, 0.5704, 0.5589, 0.5376, 0.5342, 0.5389, 0.5232, 0.5523, 0.5671,
            0.5588, 0.5517, 0.5909, 0.6387, 0.6752, 0.7113, 0.7097, 0.7069, 0.7007, 0.7035, 0.7047,
            0.7122, 0.7128, 0.7168, 0.7142, 0.7082, 0.7040, 0.7102, 0.7145, 0.7198, 0.7377, 0.7586,
            0.7716, 0.7793, 0.7793, 0.7751, 0.7776,
        ];

        assert_eq!(atr.len(), expected_atr.len() + 1);

        for i in 0..expected_atr.len() {
            let atr_value = atr.get(i + 1).unwrap();
            assert_within_tolerance(*atr_value, expected_atr[i], 0.001);
        }
    }

    #[test]
    fn test_natr_basic() {
        let (high, low, close) = get_test_data();

        // This is not exactly same as pandas-ta because pandas-ta use pandas' `adjust=True` mode
        // (https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.ewm.html)
        // (https://github.com/Data-Analisis/Technical-Analysis-Indicators---Pandas/blob/master/pandas_ta/overlap/rma.py#L16)
        let expected_natr = vec![
            1.1932, 1.1795, 1.1714, 1.1703, 1.1400, 1.0955, 1.0831, 1.0797, 1.0438, 1.1151, 1.1457,
            1.1232, 1.1028, 1.1910, 1.2825, 1.3450, 1.4020, 1.3933, 1.3839, 1.3747, 1.3917, 1.3976,
            1.4204, 1.4190, 1.4220, 1.4134, 1.3996, 1.3856, 1.3939, 1.4006, 1.4095, 1.4565, 1.5124,
            1.5477, 1.5658, 1.5694, 1.5662, 1.5802,
        ];

        let config = NatrConfig::<f64>::new(14, 1.0 / 14.0);
        let result = natr(&high, &low, &close, config);
        assert_eq!(result.natr.len(), expected_natr.len() + 1); // 1 initial NaN value

        for i in 0..expected_natr.len() {
            let natr_value = result.natr.get(i + 1).unwrap(); // Skip first 14 NaN values
            assert_within_tolerance(*natr_value, expected_natr[i], 0.001);
        }
    }
}
