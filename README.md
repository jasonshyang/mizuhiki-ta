# mizuhiki-ta

**Mizuhiki-ta** (水引) is a fast, extensible technical analysis library for Rust.

- Works with different numeric types (f32, f64, etc.) via a simple trait-based API.
- Easy to extend: implement one trait to add your own numeric type.

## Documentation
```bash
# Generate and open the documentation
cargo doc --no-deps --open
```

## Usage
```rust
use mizuhiki_ta::{
    core::{CandleSeries, Candle},
    indicators::{rsi_series, MomentumConfig},
};

// Create candle series
let mut candles = CandleSeries::<f64>::new(60_000); // 1-minute candles
let prices = [/* .. */];
let timestamps = [/* .. */];
let volumes = [/* .. */];
for i in 0..prices.len() {
    candles.push(prices[i], timestamps[i], volumes[i]).unwrap();
}

// Configure RSI with 14-period EMA smoothing, maximum history of 140
let config = MomentumConfig::new_f64(14, 140);
let rsi = rsi_series(&candles, config).unwrap();
```


## Demo
```bash
# RSI Demo
cargo run --example rsi_demo

# NATR Demo
cargo run --example natr_demo
```
