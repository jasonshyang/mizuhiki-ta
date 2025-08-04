use crate::{
    core::{CandleSeries, Column, Error, Numeric},
    indicators::Config,
};

/// Calculate Normalized Average True Range (NATR) for a candle series.
///
/// Normalized Average True Range attempts to normalize the average true range
/// by expressing it as a percentage of the closing price. This allows for
/// comparison of volatility across different price levels and securities.
///
/// # Algorithm
///
/// ```text
/// TR[i] = max(high[i] - low[i],
///             |high[i] - close[i-1]|,
///             |low[i] - close[i-1]|)
///
/// ATR = EMA(TR, period)
/// NATR = (ATR / close) * 100
/// ```
///
/// # Arguments
/// * `candles` - Series of OHLC candles with high, low, close data
/// * `config` - Configuration with period and smoothing parameters
///
/// # Returns
/// A column of NATR values expressed as percentages
///
/// # Errors
/// Returns `Error::NotEnoughData` if insufficient candles for calculation.
pub fn natr_series<T: Numeric>(
    candles: &CandleSeries<T>,
    config: &Config<T>,
) -> Result<Column<T>, Error> {
    // We need at least `period + 1` candles to calculate RSI
    // Because we lose the first candle when calculating true range
    if candles.len() < config.period + 1 {
        return Err(Error::NotEnoughData);
    }

    let tr = candles.true_range(Some(config.max_history));
    let atr = tr.into_ewm_mean(config.alpha);
    let closes = candles.closes();

    let hundred = T::hundred();
    Ok(atr
        .iter()
        .zip(closes.iter())
        .map(|(atr_value, close)| {
            if atr_value.is_zero() {
                T::ZERO
            } else {
                hundred * (*atr_value / *close)
            }
        })
        .collect())
}

/// Calculate the latest NATR value for a candle series.
/// This is more efficient than `natr_series` when only the most recent value is needed.
pub fn natr_latest<T: Numeric>(candles: &CandleSeries<T>, config: &Config<T>) -> Result<T, Error> {
    if candles.len() < config.period + 1 {
        return Err(Error::NotEnoughData);
    }

    let tr = candles.true_range(Some(config.max_history));
    let atr = tr.into_ewm_mean(config.alpha);
    let closes = candles.closes();

    let latest_atr = atr.last().unwrap();
    let latest_close = closes.last().unwrap();

    if latest_atr.is_zero() {
        Ok(T::ZERO)
    } else {
        Ok(T::hundred() * (*latest_atr / *latest_close))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Candle, CandleSeries};

    fn get_test_data() -> CandleSeries<f64> {
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

        let timestamps: Vec<i64> = (0..highs.len() as i64).map(|i| i * 60).collect();
        let mut candles = CandleSeries::new(60);

        for i in 0..highs.len() {
            candles.push_candle_unchecked(
                Candle {
                    open: 0.0, // Open is not used in NATR calculation
                    high: highs[i],
                    low: lows[i],
                    close: closes[i],
                    volume: 0.0, // Volume is not used in NATR calculation
                },
                timestamps[i] as u64,
            );
        }
        candles
    }

    #[test]
    fn test_natr_series() {
        let candles = get_test_data();

        // This is not exactly same as pandas-ta, because pandas-ta use pandas' `adjust=True` mode
        // This was ran with `adjust=False` mode
        // (https://pandas.pydata.org/docs/reference/api/pandas.DataFrame.ewm.html)
        // (https://github.com/Data-Analisis/Technical-Analysis-Indicators---Pandas/blob/master/pandas_ta/overlap/rma.py#L16)
        let expected_natr = vec![
            1.1932, 1.1795, 1.1714, 1.1703, 1.1400, 1.0955, 1.0831, 1.0797, 1.0438, 1.1151, 1.1457,
            1.1232, 1.1028, 1.1910, 1.2825, 1.3450, 1.4020, 1.3933, 1.3839, 1.3747, 1.3917, 1.3976,
            1.4204, 1.4190, 1.4220, 1.4134, 1.3996, 1.3856, 1.3939, 1.4006, 1.4095, 1.4565, 1.5124,
            1.5477, 1.5658, 1.5694, 1.5662, 1.5802,
        ];

        let config = Config::new_f64_wilder(14, 100);
        let natr = natr_series(&candles, &config).unwrap();

        assert_eq!(natr.len(), expected_natr.len() + 1); // 1 initial NaN value

        for (i, &expected) in expected_natr.iter().enumerate() {
            let natr_value = natr[i + 1];
            assert!(
                (natr_value - expected).abs() < 0.01,
                "Mismatch at index {i}"
            );
        }
    }
}
