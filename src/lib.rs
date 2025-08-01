//! # mizuhiki-ta
//!
//! Fast, extensible technical analysis library for Rust with trait-based numeric types.
//!
//! ## Example
//! ```rust
//! use mizuhiki_ta::{
//!     core::CandleSeries,
//!     indicators::{rsi_series, Config}
//! };
//!
//! # fn main() -> Result<(), mizuhiki_ta::core::Error> {
//! // Create a new candle series with 60-second timeframe
//! let mut candles = CandleSeries::<f64>::new(60_000);
//!
//! // Add sample price data - RSI needs at least 15 data points for 14-period calculation
//! let prices = vec![
//!     44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42,
//!     45.84, 46.08, 45.89, 46.03, 46.83, 47.69, 46.04, 46.59
//! ];
//!
//! // Push price data to construct candles automatically
//! for (i, &price) in prices.iter().enumerate() {
//!     let timestamp = (i as u64) * 60_000; // 60-second intervals
//!     candles.push(price, 1000.0, timestamp)?; // price, volume, timestamp
//! }
//!
//! // Calculate RSI with 14-period configuration
//! let config = Config::new_f64(14, 50);
//! let rsi_values = rsi_series(&candles, config)?;
//!
//! println!("RSI calculated for {} candles", rsi_values.len());
//! # Ok(())
//! # }
//! ```

pub mod core;
pub mod indicators;
