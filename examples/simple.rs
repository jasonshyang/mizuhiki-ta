use mizuhiki_ta::core::series::Series;
use mizuhiki_ta::indicators::rsi::{RsiConfig, rsi};

fn main() {
    let prices = vec![
        44.34, 44.30, 44.29, 44.19, 44.21, 44.29, 44.40, 44.54, 44.71, 44.89,
    ];
    let series = Series::from_vec("price".to_string(), prices);
    let config = RsiConfig::<f64>::from_period(14);
    let result = rsi(&series, config);
    println!("RSI values: {:?}", result.rsi.values());
    // RSI values: [50.0, 0.0, 0.0, 0.0, 14.2636772816616, 48.299400411095725, 68.27845874334307, 79.76291854761195, 86.57353859095393, 90.48551066962546]
}
