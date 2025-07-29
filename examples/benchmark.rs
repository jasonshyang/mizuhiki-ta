use mizuhiki_ta::core::series::Series;
use mizuhiki_ta::core::traits::Numeric;
use mizuhiki_ta::indicators::natr::{NatrConfig, natr};
use mizuhiki_ta::indicators::rsi::{RsiConfig, rsi};
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::time::Instant;

/// Custom decimal type for benchmarking
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Decimal {
    value: i64, // Store as fixed-point with 6 decimal places
}

impl Decimal {
    fn new(val: f64) -> Self {
        Self {
            value: (val * 1_000_000.0).round() as i64,
        }
    }

    fn to_f64(self) -> f64 {
        self.value as f64 / 1_000_000.0
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            value: (self.value * other.value) / 1_000_000,
        }
    }
}

impl Div for Decimal {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            value: (self.value * 1_000_000) / other.value,
        }
    }
}

impl Rem for Decimal {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Self {
            value: self.value % other.value,
        }
    }
}

impl Neg for Decimal {
    type Output = Self;
    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for Decimal {
    fn mul_assign(&mut self, other: Self) {
        self.value = (self.value * other.value) / 1_000_000;
    }
}

impl DivAssign for Decimal {
    fn div_assign(&mut self, other: Self) {
        self.value = (self.value * 1_000_000) / other.value;
    }
}

impl RemAssign for Decimal {
    fn rem_assign(&mut self, other: Self) {
        self.value %= other.value;
    }
}

impl Sum for Decimal {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), Add::add)
    }
}

impl Product for Decimal {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl Numeric for Decimal {
    fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    fn zero() -> Self {
        Self { value: 0 }
    }
    fn one() -> Self {
        Self { value: 1_000_000 }
    }
    fn fifty() -> Self {
        Self { value: 50_000_000 }
    }
    fn hundred() -> Self {
        Self { value: 100_000_000 }
    }
}

fn generate_price_data(size: usize) -> Vec<i32> {
    let mut prices = Vec::with_capacity(size);
    let mut price = 100_000;

    for i in 0..size {
        let trend = ((i * 314) % 62832) as f64 / 10000.0;
        let volatility = ((i * 157) % 31416) as f64 / 10000.0;

        let change = (trend.sin() * 2000.0 + volatility.cos() * 800.0) as i32;
        price += change;
        prices.push(price);
    }

    prices
}

fn generate_ohlc_data(size: usize) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut open = Vec::with_capacity(size);
    let mut high = Vec::with_capacity(size);
    let mut low = Vec::with_capacity(size);
    let mut close = Vec::with_capacity(size);

    let mut price = 100_000;
    for i in 0..size {
        let trend = ((i * 314) % 62832) as f64 / 10000.0;
        let volatility = ((i * 157) % 31416) as f64 / 10000.0;

        open.push(price);
        let day_high = price + (volatility.abs() * 1500.0 + 500.0) as i32;
        let day_low = price - (volatility.abs() * 1500.0 + 500.0) as i32;
        let day_close = price + (trend.sin() * 2000.0 + volatility.cos() * 800.0) as i32;

        high.push(day_high);
        low.push(day_low);
        close.push(day_close);

        price = day_close;
    }

    (open, high, low, close)
}

fn benchmark_rsi_types() {
    println!("=== RSI Numeric Type Comparison ===\n");

    let sizes = vec![1000, 10000, 50000];

    for &size in &sizes {
        println!("--- Data size: {} points ---", size);

        let price_data = generate_price_data(size);

        let f32_prices: Vec<f32> = price_data.iter().map(|&p| p as f32 / 1000.0).collect();
        let f64_prices: Vec<f64> = price_data.iter().map(|&p| p as f64 / 1000.0).collect();
        let decimal_prices: Vec<Decimal> = price_data
            .iter()
            .map(|&p| Decimal::new(p as f64 / 1000.0))
            .collect();

        let f32_series = Series::from_vec("benchmark_f32".to_string(), f32_prices);
        let f64_series = Series::from_vec("benchmark_f64".to_string(), f64_prices);
        let decimal_series = Series::from_vec("benchmark_decimal".to_string(), decimal_prices);

        // f32 benchmark
        let start = Instant::now();
        let f32_config = RsiConfig::<f32>::default();
        let _result_f32 = rsi(&f32_series, f32_config);
        let f32_duration = start.elapsed();

        // f64 benchmark
        let start = Instant::now();
        let f64_config = RsiConfig::<f64>::default();
        let _result_f64 = rsi(&f64_series, f64_config);
        let f64_duration = start.elapsed();

        // Decimal benchmark
        let start = Instant::now();
        let decimal_config = RsiConfig::new(14, Decimal::new(2.0 / 15.0));
        let _result_decimal = rsi(&decimal_series, decimal_config);
        let decimal_duration = start.elapsed();

        let f32_ms = f32_duration.as_secs_f64() * 1000.0;
        let f64_ms = f64_duration.as_secs_f64() * 1000.0;
        let decimal_ms = decimal_duration.as_secs_f64() * 1000.0;

        println!("  f32:             {:>8.3} ms", f32_ms);
        println!("  f64:             {:>8.3} ms", f64_ms);
        println!("  Decimal:         {:>8.3} ms", decimal_ms);
        println!("  f32 throughput:  {:>8.0} points/ms", size as f64 / f32_ms);
        println!("  f64 throughput:  {:>8.0} points/ms", size as f64 / f64_ms);
        println!(
            "  Dec throughput:  {:>8.0} points/ms",
            size as f64 / decimal_ms
        );
        println!();
    }
}

fn benchmark_natr_types() {
    println!("=== NATR Numeric Type Comparison ===\n");

    let sizes = vec![1000, 10000, 50000];

    for &size in &sizes {
        println!("--- Data size: {} points ---", size);

        let (_, high_data, low_data, close_data) = generate_ohlc_data(size);

        let f32_high: Vec<f32> = high_data.iter().map(|&p| p as f32 / 1000.0).collect();
        let f32_low: Vec<f32> = low_data.iter().map(|&p| p as f32 / 1000.0).collect();
        let f32_close: Vec<f32> = close_data.iter().map(|&p| p as f32 / 1000.0).collect();

        let f64_high: Vec<f64> = high_data.iter().map(|&p| p as f64 / 1000.0).collect();
        let f64_low: Vec<f64> = low_data.iter().map(|&p| p as f64 / 1000.0).collect();
        let f64_close: Vec<f64> = close_data.iter().map(|&p| p as f64 / 1000.0).collect();

        let decimal_high: Vec<Decimal> = high_data
            .iter()
            .map(|&p| Decimal::new(p as f64 / 1000.0))
            .collect();
        let decimal_low: Vec<Decimal> = low_data
            .iter()
            .map(|&p| Decimal::new(p as f64 / 1000.0))
            .collect();
        let decimal_close: Vec<Decimal> = close_data
            .iter()
            .map(|&p| Decimal::new(p as f64 / 1000.0))
            .collect();

        let f32_high_series = Series::from_vec("high_f32".to_string(), f32_high);
        let f32_low_series = Series::from_vec("low_f32".to_string(), f32_low);
        let f32_close_series = Series::from_vec("close_f32".to_string(), f32_close);

        let f64_high_series = Series::from_vec("high_f64".to_string(), f64_high);
        let f64_low_series = Series::from_vec("low_f64".to_string(), f64_low);
        let f64_close_series = Series::from_vec("close_f64".to_string(), f64_close);

        let decimal_high_series = Series::from_vec("high_decimal".to_string(), decimal_high);
        let decimal_low_series = Series::from_vec("low_decimal".to_string(), decimal_low);
        let decimal_close_series = Series::from_vec("close_decimal".to_string(), decimal_close);

        // f32 benchmark
        let start = Instant::now();
        let f32_config = NatrConfig::<f32>::default();
        let _result_f32 = natr(
            &f32_high_series,
            &f32_low_series,
            &f32_close_series,
            f32_config,
        );
        let f32_duration = start.elapsed();

        // f64 benchmark
        let start = Instant::now();
        let f64_config = NatrConfig::<f64>::default();
        let _result_f64 = natr(
            &f64_high_series,
            &f64_low_series,
            &f64_close_series,
            f64_config,
        );
        let f64_duration = start.elapsed();

        // Decimal benchmark
        let start = Instant::now();
        let decimal_config = NatrConfig::new(14, Decimal::new(2.0 / 15.0));
        let _result_decimal = natr(
            &decimal_high_series,
            &decimal_low_series,
            &decimal_close_series,
            decimal_config,
        );
        let decimal_duration = start.elapsed();

        let f32_ms = f32_duration.as_secs_f64() * 1000.0;
        let f64_ms = f64_duration.as_secs_f64() * 1000.0;
        let decimal_ms = decimal_duration.as_secs_f64() * 1000.0;

        println!("  f32:             {:>8.3} ms", f32_ms);
        println!("  f64:             {:>8.3} ms", f64_ms);
        println!("  Decimal:         {:>8.3} ms", decimal_ms);
        println!("  f32 throughput:  {:>8.0} points/ms", size as f64 / f32_ms);
        println!("  f64 throughput:  {:>8.0} points/ms", size as f64 / f64_ms);
        println!(
            "  Dec throughput:  {:>8.0} points/ms",
            size as f64 / decimal_ms
        );
        println!();
    }
}

fn accuracy_comparison() {
    println!("=== Accuracy Comparison ===\n");

    let test_data = vec![
        44340, 44090, 44150, 43610, 44330, 44830, 45850, 46080, 45890, 46030, 46830, 46690, 46450,
        46590, 46300, 46280, 46280, 46000, 46030, 46410, 46220, 45640,
    ];

    let f32_prices: Vec<f32> = test_data.iter().map(|&p| p as f32 / 1000.0).collect();
    let f64_prices: Vec<f64> = test_data.iter().map(|&p| p as f64 / 1000.0).collect();
    let decimal_prices: Vec<Decimal> = test_data
        .iter()
        .map(|&p| Decimal::new(p as f64 / 1000.0))
        .collect();

    let f32_series = Series::from_vec("test_f32".to_string(), f32_prices);
    let f64_series = Series::from_vec("test_f64".to_string(), f64_prices);
    let decimal_series = Series::from_vec("test_decimal".to_string(), decimal_prices);

    let f32_config = RsiConfig::<f32>::default();
    let f64_config = RsiConfig::<f64>::default();
    let decimal_config = RsiConfig::new(14, Decimal::new(2.0 / 15.0));

    let f32_result = rsi(&f32_series, f32_config);
    let f64_result = rsi(&f64_series, f64_config);
    let decimal_result = rsi(&decimal_series, decimal_config);

    let f32_values = f32_result.rsi.values();
    let f64_values = f64_result.rsi.values();
    let decimal_values = decimal_result.rsi.values();

    println!("--- RSI accuracy comparison (last 5 values, using Decimal as baseline) ---");
    for i in (f32_values.len().saturating_sub(5))..f32_values.len() {
        let decimal_val = decimal_values[i].to_f64();
        let f32_diff = (decimal_val - f32_values[i] as f64).abs();
        let f64_diff = (decimal_val - f64_values[i]).abs();

        println!(
            "  Point {}: f32={:.6}, f64={:.6}, decimal={:.6}",
            i, f32_values[i], f64_values[i], decimal_val
        );
        println!(
            "           f32_diff={:.8}, f64_diff={:.8}",
            f32_diff, f64_diff
        );
    }
    println!();
}

fn main() {
    println!("🚀 mizuhiki-ta Numeric Type Performance Analysis\n");

    benchmark_rsi_types();
    benchmark_natr_types();
    accuracy_comparison();
}
