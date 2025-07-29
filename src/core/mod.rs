//! Core data structures and traits for technical analysis.
//!
//! This module provides the fundamental building blocks for the technical analysis library:
//!
//! - [`traits`] - Core traits defining numeric and indexable types
//! - [`mod@column`] - One-dimensional numeric data structure for calculations  
//! - [`series`] - Labeled data structure combining data with an index

pub mod column;
pub mod series;
pub mod traits;
