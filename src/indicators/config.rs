//! Configuration structures for technical indicators.

use crate::core::Numeric;

/// Configuration for technical indicators.
///
/// This struct provides common parameters for indicators that use exponential
/// moving averages and require historical data management.
///
/// # Fields
/// * `alpha` - Smoothing factor for exponential moving average (0 < alpha < 1)
/// * `period` - Number of periods for calculation (e.g., 14 for RSI-14)
/// * `max_history` - Maximum data points to retain for efficiency
///
/// # Alpha Calculation
/// * **EMA**: `alpha = 2.0 / (period + 1.0)` - Standard exponential moving average
/// * **Wilder**: `alpha = 1.0 / period` - Wilder's smoothing (used in RSI)
#[derive(Debug, Clone)]
pub struct Config<T> {
    pub alpha: T,
    pub period: usize,
    pub max_history: usize,
}

impl<T: Numeric> Config<T> {
    /// Creates a new configuration with custom alpha value.
    ///
    /// # Panics
    /// Panics if `max_history < period`.
    pub fn new(alpha: T, period: usize, max_history: usize) -> Self {
        if max_history < period {
            panic!("max_history must be greater than or equal to period");
        }

        Self {
            alpha,
            period,
            max_history,
        }
    }
}

impl Config<f64> {
    /// Creates a configuration with standard EMA smoothing for f64.
    ///
    /// Uses alpha = 2.0 / (period + 1.0) for standard exponential moving average.
    ///
    /// # Arguments
    /// * `period` - Number of periods (e.g., 14 for RSI-14)
    /// * `max_history` - Maximum data points to retain
    pub fn new_f64(period: usize, max_history: usize) -> Self {
        if max_history < period {
            panic!("max_history must be greater than or equal to period");
        }

        let alpha = 2.0 / (period as f64 + 1.0);
        Self {
            alpha,
            period,
            max_history,
        }
    }

    /// Creates a configuration with Wilder's smoothing for f64.
    ///
    /// Uses alpha = 1.0 / period for Wilder's smoothing method,
    /// commonly used in RSI and related momentum indicators.
    ///
    /// # Arguments
    /// * `period` - Number of periods (e.g., 14 for RSI-14)
    /// * `max_history` - Maximum data points to retain
    pub fn new_f64_wilder(period: usize, max_history: usize) -> Self {
        if max_history < period {
            panic!("max_history must be greater than or equal to period");
        }

        let alpha = 1.0 / period as f64;
        Self {
            alpha,
            period,
            max_history,
        }
    }
}

impl Config<f32> {
    /// Creates a configuration with standard EMA smoothing for f32.
    ///
    /// Uses alpha = 2.0 / (period + 1.0) for standard exponential moving average.
    pub fn new_f32(period: usize, max_history: usize) -> Self {
        if max_history < period {
            panic!("max_history must be greater than or equal to period");
        }

        let alpha = 2.0 / (period as f32 + 1.0);
        Self {
            alpha,
            period,
            max_history,
        }
    }

    /// Creates a configuration with Wilder's smoothing for f32.
    ///
    /// Uses alpha = 1.0 / period for Wilder's smoothing method.
    pub fn new_f32_wilder(period: usize, max_history: usize) -> Self {
        if max_history < period {
            panic!("max_history must be greater than or equal to period");
        }

        let alpha = 1.0 / period as f32;
        Self {
            alpha,
            period,
            max_history,
        }
    }
}

impl Default for Config<f64> {
    /// Default configuration: 14-period with EMA smoothing and 140 max history.
    fn default() -> Self {
        let period = 14;
        let max_history = period * 10;
        let alpha = 2.0 / (period as f64 + 1.0);
        Self {
            alpha,
            period,
            max_history,
        }
    }
}

impl Default for Config<f32> {
    /// Default configuration: 14-period with EMA smoothing and 140 max history.
    fn default() -> Self {
        let period = 14;
        let max_history = period * 10;
        let alpha = 2.0 / (period as f32 + 1.0);
        Self {
            alpha,
            period,
            max_history,
        }
    }
}
