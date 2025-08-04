use crate::{
    core::{CandleSeries, Column, Error, Numeric},
    indicators::Config,
};

/// Calculate Relative Strength Index (RSI) for a candle series.
///
/// The Relative Strength Index is a popular momentum oscillator used to measure the
/// velocity as well as the magnitude of directional price movements. RSI oscillates
/// between 0 and 100, with values above 70 typically indicating overbought conditions
/// and values below 30 indicating oversold conditions.
///
/// # Algorithm
///
/// ```text
/// positive = close[i] - close[i-1] if positive, else 0
/// negative = |close[i] - close[i-1]| if negative, else 0
///
/// avg_gain = EMA(positive, alpha)
/// avg_loss = EMA(negative, alpha)
///
/// RSI = 100 * avg_gain / (avg_gain + avg_loss)
/// ```
///
/// # Arguments
/// * `candles` - Series of OHLC candles
/// * `config` - Configuration with period and smoothing parameters
///
/// # Returns
/// A column of RSI values (0-100 range)
///
/// # Errors
/// Returns `Error::NotEnoughData` if insufficient candles for calculation.
pub fn rsi_series<T: Numeric>(
    candles: &CandleSeries<T>,
    config: &Config<T>,
) -> Result<Column<T>, Error> {
    // We need at least `period + 1` candles to calculate RSI
    // Because we lose the first candle when calculating gains and losses
    if candles.len() < config.period + 1 {
        return Err(Error::NotEnoughData);
    }

    let closes = candles.closes();
    let (gains, losses) = closes.gains_losses(Some(config.max_history));

    let ema_gains = gains.into_ewm_mean(config.alpha);
    let ema_losses = losses.into_ewm_mean(config.alpha);

    let fifty = T::fifty();
    let hundred = T::hundred();

    Ok(ema_gains
        .iter()
        .zip(ema_losses.iter())
        .map(|(gain, loss)| {
            if gain.is_zero() && loss.is_zero() {
                fifty
            } else {
                hundred * (*gain / (*gain + *loss))
            }
        })
        .collect())
}

/// Calculate the latest RSI value for a candle series.
/// This is more efficient than `rsi_series` when only the most recent value is needed.
pub fn rsi_latest<T: Numeric>(candles: &CandleSeries<T>, config: &Config<T>) -> Result<T, Error> {
    if candles.len() < config.period + 1 {
        return Err(Error::NotEnoughData);
    }

    let closes = candles.closes();
    let (gains, losses) = closes.gains_losses(Some(config.max_history));

    let ema_gains = gains.into_ewm_mean(config.alpha);
    let ema_losses = losses.into_ewm_mean(config.alpha);

    let latest_gain = ema_gains.last().unwrap();
    let latest_loss = ema_losses.last().unwrap();

    let rsi = if latest_gain.is_zero() && latest_loss.is_zero() {
        T::fifty()
    } else {
        T::hundred() * (*latest_gain / (*latest_gain + *latest_loss))
    };

    Ok(rsi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::CandleSeries;

    #[test]
    fn test_rsi_series() {
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

        let mut candles = CandleSeries::new(60);

        for (i, &price) in prices.iter().enumerate() {
            candles.push(price, 0.0, (i as u64) * 60).unwrap();
        }

        let config = Config::new_f64_wilder(14, 100);

        let rsi_values = rsi_series(&candles, &config).unwrap();

        assert_eq!(rsi_values.len(), expected_rsi.len() + 14);

        for (i, &expected) in expected_rsi.iter().enumerate() {
            let rsi_value = rsi_values[i + 14];
            assert!(
                (rsi_value - expected).abs() < 0.01,
                "RSI value mismatch at index {i}"
            );
        }
    }
}
