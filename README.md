# mizuhiki-ta

**Mizuhiki-ta** (水引) is a fast, extensible technical analysis library for Rust.

- Works with many numeric types (`f32`, `f64`, `i32`, etc.) via a simple trait-based API.
- Easy to extend: implement one trait to add your own numeric type.
 
## Documentation

```bash
# Generate and open the documentation
cargo doc --no-deps --open
```

## Usage

```rust
use mizuhiki_ta::core::series::Series;
use mizuhiki_ta::indicators::rsi::{rsi, RsiConfig};

let prices = vec![44.34, 44.30, 44.29, 44.19, 44.21, 44.29, 44.40, 44.54, 44.71, 44.89];
let series = Series::from_vec("price".to_string(), prices);
let config = RsiConfig::<f64>::from_period(14);
let result = rsi(&series, config);
println!("RSI values: {:?}", result.rsi.values());
```

## Demo

```bash
# RSI Demo
cargo run --example rsi_demo

# NATR Demo
cargo run --example natr_demo

# Performance comparison
cargo run --example benchmark
```