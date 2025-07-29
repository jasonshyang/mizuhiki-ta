//! Technical analysis indicators.
//!
//! This module contains implementations of various technical analysis indicators
//! with support for multiple precision levels and high-performance calculations.
//!
//! # Available Indicators
//!
//! - [`rsi`] - Relative Strength Index with configurable period and precision
//! - [`natr`] - Normalized Average True Range for volatility analysis

pub mod natr;
pub mod rsi;
