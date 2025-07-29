use mizuhiki_ta::core::series::Series;
use mizuhiki_ta::indicators::rsi::{RsiConfig, rsi};

fn main() {
    println!("=== RSI Demo ===");

    let prices = vec![
        150.0, 151.2, 149.8, 152.1, 153.4, 151.9, 154.2, 155.8, 153.7, 156.1, 157.3, 155.9, 158.2,
        159.1, 157.8, 160.4, 159.2, 161.1, 162.3, 160.9, 163.4, 162.1, 164.7, 163.5, 165.8, 167.2,
        165.1, 168.4, 166.9, 169.7, 171.2, 168.8, 172.1, 170.5, 173.9, 172.3, 174.6, 173.1, 175.8,
        174.2,
    ];

    let series = Series::from_vec("AAPL_close".to_string(), prices);

    println!("Processing {} price points\n", series.len());

    let config = RsiConfig::<f64>::default();
    let result = rsi(&series, config);

    println!("--- RSI Values (last 10) ---");
    let rsi_values = result.rsi.values();
    let start_idx = rsi_values.len().saturating_sub(10);
    for (i, &rsi_val) in rsi_values[start_idx..].iter().enumerate() {
        let price = series.values()[start_idx + i];
        let signal = match rsi_val {
            rsi if rsi > 70.0 => "(overbought)",
            rsi if rsi < 30.0 => "(oversold)",
            _ => "(neutral)",
        };
        println!(
            "Day {}: ${:.2}, RSI = {:.1} {}",
            start_idx + i + 1,
            price,
            rsi_val,
            signal
        );
    }

    println!("\n--- Analysis ---");
    let last_price = series.values().last().unwrap();
    let last_rsi = rsi_values[rsi_values.len() - 1];

    println!("Current price: ${:.2}", last_price);
    println!("Current RSI: {:.1}", last_rsi);

    let signal = match last_rsi {
        rsi if rsi > 70.0 => "Potentially overbought",
        rsi if rsi < 30.0 => "Potentially oversold",
        _ => "Neutral range",
    };

    println!("Signal: {}", signal);
}
