//! OHLCV candle data structures and operations.

use crate::core::Error;
use std::fmt::Display;

use super::{Column, Numeric};

/// A single OHLCV candle with volume.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candle<T> {
    pub open: T,
    pub high: T,
    pub low: T,
    pub close: T,
    pub volume: T,
}

/// Reference to candle data without ownership.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CandleRef<'a, T> {
    pub open: &'a T,
    pub high: &'a T,
    pub low: &'a T,
    pub close: &'a T,
    pub volume: &'a T,
}

/// Time series of OHLCV candles for technical analysis.
#[derive(Debug, Clone)]
pub struct CandleSeries<T> {
    opens: Column<T>,
    highs: Column<T>,
    lows: Column<T>,
    closes: Column<T>,
    volumes: Column<T>,
    timestamps: Vec<u64>,
    timeframe: u64,
}

impl<T: Numeric> CandleSeries<T> {
    /// Creates a new candle series with specified timeframe.
    pub fn new(timeframe: u64) -> Self {
        Self {
            opens: Column::new(),
            highs: Column::new(),
            lows: Column::new(),
            closes: Column::new(),
            volumes: Column::new(),
            timestamps: Vec::new(),
            timeframe,
        }
    }

    /// Returns a reference to the opening prices column.
    pub fn opens(&self) -> &Column<T> {
        &self.opens
    }

    /// Returns a reference to the high prices column.
    pub fn highs(&self) -> &Column<T> {
        &self.highs
    }

    /// Returns a reference to the low prices column.
    pub fn lows(&self) -> &Column<T> {
        &self.lows
    }

    /// Returns a reference to the closing prices column.
    pub fn closes(&self) -> &Column<T> {
        &self.closes
    }

    /// Returns a reference to the volumes column.
    pub fn volumes(&self) -> &Column<T> {
        &self.volumes
    }

    /// Gets a candle at the specified index as a reference.
    pub fn get(&self, index: usize) -> Option<CandleRef<T>> {
        if index >= self.len() {
            return None;
        }
        Some(CandleRef {
            open: &self.opens[index],
            high: &self.highs[index],
            low: &self.lows[index],
            close: &self.closes[index],
            volume: &self.volumes[index],
        })
    }

    /// Gets a candle at the specified index as an owned value.
    pub fn get_owned(&self, index: usize) -> Option<Candle<T>> {
        if index >= self.len() {
            return None;
        }
        Some(Candle {
            open: self.opens[index],
            high: self.highs[index],
            low: self.lows[index],
            close: self.closes[index],
            volume: self.volumes[index],
        })
    }

    /// Returns the number of candles in the series.
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    /// Returns true if the series contains no candles.
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// Pushes a new price tick to the series, creating or updating candles based on timeframe.
    /// Returns an error if the timestamp is out of order.
    pub fn push(&mut self, price: T, vol: T, ts: u64) -> Result<(), Error> {
        let next_start = ts - (ts % self.timeframe);

        match self.timestamps.last() {
            None => {
                self.push_new_candle(price, vol, next_start);
            }
            Some(&last_ts) => {
                match next_start.cmp(&last_ts) {
                    // Push a new candle if the next start time is after the last candle start
                    std::cmp::Ordering::Greater => {
                        self.push_new_candle(price, vol, next_start);
                    }
                    // Update the last candle if the next start time is same as the last candle start
                    std::cmp::Ordering::Equal => {
                        self.update_last_candle(price, vol);
                    }
                    // If the next start time is before the last candle start, return an error
                    std::cmp::Ordering::Less => {
                        return Err(Error::InvalidTimestamp(next_start));
                    }
                }
            }
        }

        Ok(())
    }

    /// Pushes a new price tick without timestamp validation.
    /// Ignores out-of-order timestamps instead of returning errors.
    pub fn push_unchecked(&mut self, price: T, vol: T, ts: u64) {
        let next_start = ts - (ts % self.timeframe);

        match self.timestamps.last() {
            None => {
                self.push_new_candle(price, vol, next_start);
            }
            Some(&last_ts) => {
                match next_start.cmp(&last_ts) {
                    // Push a new candle if the next start time is after the last candle start
                    std::cmp::Ordering::Greater => {
                        self.push_new_candle(price, vol, next_start);
                    }
                    // Update the last candle if the next start time is same as the last candle start
                    std::cmp::Ordering::Equal => {
                        self.update_last_candle(price, vol);
                    }
                    // If the next start time is before the last candle start, we ignore it
                    std::cmp::Ordering::Less => {}
                }
            }
        }
    }

    /// Pushes a complete candle to the series without any validation.
    pub fn push_candle_unchecked(&mut self, candle: Candle<T>, ts: u64) {
        self.opens.push(candle.open);
        self.highs.push(candle.high);
        self.lows.push(candle.low);
        self.closes.push(candle.close);
        self.volumes.push(candle.volume);
        self.timestamps.push(ts);
    }

    /// Calculate the true range for each candle in this series.
    ///
    /// True range is defined as the maximum of:
    /// - The difference between the current candle's high and low.
    /// - The absolute difference between the current candle's high and the previous candle's close.
    /// - The absolute difference between the current candle's low and the previous candle's close.
    ///
    /// Because the first candle has no previous candle, its true range is simply the high minus the low.
    pub fn true_range(&self, max_history: Option<usize>) -> Column<T> {
        if self.is_empty() {
            return Column::new();
        }

        let len = self.len();
        let start = match max_history {
            Some(max) => len.saturating_sub(max),
            None => 0,
        };

        let mut tr = Column::with_capacity(len - start);
        for i in start..len {
            let candle = self.get(i).unwrap();
            if i == 0 {
                let simple_range = *candle.high - *candle.low;
                tr.push(simple_range);
            } else {
                let prev_candle = self.get(i - 1).unwrap();
                let range = candle.true_range(&prev_candle);
                tr.push(range);
            }
        }
        tr
    }

    /// Push a new candle with the given price, volume, and start timestamp.
    fn push_new_candle(&mut self, price: T, vol: T, start_ts: u64) {
        self.opens.push(price);
        self.highs.push(price);
        self.lows.push(price);
        self.closes.push(price);
        self.volumes.push(vol);
        self.timestamps.push(start_ts);
    }

    fn update_last_candle(&mut self, price: T, vol: T) {
        let i = self.len() - 1;
        if price > self.highs[i] {
            self.highs[i] = price;
        }
        if price < self.lows[i] {
            self.lows[i] = price;
        }
        self.closes[i] = price;
        self.volumes[i] += vol;
    }
}

impl<T: Numeric> Candle<T> {
    /// Calculates the true range between this candle and the previous candle.
    pub fn true_range(&self, prev: &Candle<T>) -> T {
        let hl = self.high - self.low;
        let hc = (self.high - prev.close).abs();
        let lc = (self.low - prev.close).abs();

        hl.max(hc).max(lc)
    }
}

impl<T: Numeric> CandleRef<'_, T> {
    /// Calculates the true range between this candle reference and the previous candle reference.
    pub fn true_range(&self, prev: &CandleRef<T>) -> T {
        let hl = *self.high - *self.low;
        let hc = (*self.high - *prev.close).abs();
        let lc = (*self.low - *prev.close).abs();

        hl.max(hc).max(lc)
    }
}

impl<'a, T> From<&'a Candle<T>> for CandleRef<'a, T> {
    fn from(candle: &'a Candle<T>) -> Self {
        CandleRef {
            open: &candle.open,
            high: &candle.high,
            low: &candle.low,
            close: &candle.close,
            volume: &candle.volume,
        }
    }
}

impl<T: Numeric> From<CandleRef<'_, T>> for Candle<T> {
    fn from(candle_ref: CandleRef<T>) -> Self {
        Candle {
            open: *candle_ref.open,
            high: *candle_ref.high,
            low: *candle_ref.low,
            close: *candle_ref.close,
            volume: *candle_ref.volume,
        }
    }
}

impl<T: Numeric + Display> Display for Candle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Candle[O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.0}]",
            self.open, self.high, self.low, self.close, self.volume
        )
    }
}

impl<T: Numeric + Display> Display for CandleRef<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CandleRef[O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.0}]",
            self.open, self.high, self.low, self.close, self.volume
        )
    }
}

impl<T: Numeric + Display> Display for CandleSeries<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CANDLE SERIES")?;
        write!(f, "\n├─ Candles: {}", self.len())?;
        write!(f, "\n├─ Timeframe: {}", self.timeframe)?;

        if self.is_empty() {
            write!(f, "\n└─ Status: EMPTY")?;
            return Ok(());
        }

        // Get first and last candles for range info
        let first = self.get_owned(0).unwrap();
        let latest = self.get_owned(self.len() - 1).unwrap();

        // Calculate price change and percentage
        let price_change = latest.close - first.open;

        // Find high and low across the series
        let mut series_high = self.highs()[0];
        let mut series_low = self.lows()[0];

        for &high in self.highs().iter() {
            if high > series_high {
                series_high = high;
            }
        }

        for &low in self.lows().iter() {
            if low < series_low {
                series_low = low;
            }
        }

        // Calculate total volume
        let total_volume: T = self.volumes().iter().copied().sum();

        // Get timestamps
        let start_timestamp = self.timestamps[0];
        let latest_timestamp = self.timestamps[self.len() - 1];

        write!(f, "\n├─ Start Time: {start_timestamp}")?;
        write!(f, "\n├─ Latest Time: {latest_timestamp}")?;
        write!(
            f,
            "\n├─ Price Range: {:.2} - {:.2} (Spread: {:.2})",
            series_low,
            series_high,
            series_high - series_low
        )?;
        write!(f, "\n├─ Price Change: {:.2}", price_change)?;
        write!(f, "\n├─ Total Volume: {:.0}", total_volume)?;

        // Show last few candles for recent price action
        let show_count = std::cmp::min(14, self.len());
        write!(f, "\n├─ Recent {} Candles:", show_count)?;

        for (idx, i) in (self.len().saturating_sub(show_count)..self.len()).enumerate() {
            if let Some(candle) = self.get_owned(i) {
                let prefix = if idx == show_count - 1 {
                    "└─ "
                } else {
                    "│  "
                };
                write!(
                    f,
                    "\n{} O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.0}",
                    prefix, candle.open, candle.high, candle.low, candle.close, candle.volume
                )?;
            }
        }

        if show_count == 0 {
            write!(
                f,
                "\n└─ Latest: O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.0}",
                latest.open, latest.high, latest.low, latest.close, latest.volume
            )?;
        }

        Ok(())
    }
}
