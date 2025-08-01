//! Error types for technical analysis operations.

use thiserror::Error;

/// Errors that can occur during technical analysis calculations.
#[derive(Error, Debug)]
pub enum Error {
    /// Timestamp is out of order or invalid for the current series.
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),

    /// Insufficient data points for the requested calculation.
    #[error("Not enough data")]
    NotEnoughData,

    /// The time series contains no data.
    #[error("Empty time series: no data available")]
    EmptyTimeSeries,
}
