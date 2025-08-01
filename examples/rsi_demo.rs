use mizuhiki_ta::{
    core::CandleSeries,
    indicators::{Config, rsi_series},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut candles = CandleSeries::<f64>::new(60_000);

    let prices = [
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61,
        46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.21, 46.25, 45.71, 46.45, 45.78, 45.35,
        44.03, 44.18, 44.22, 44.57,
    ];

    for (i, &close) in prices.iter().enumerate() {
        let timestamp = (i as u64) * 60_000; // 1-minute intervals
        let volume = 1000.0 + (i as f64) * 10.0; // Synthetic volume
        candles.push(close, volume, timestamp)?;
    }

    // Configure RSI with 14-period EMA smoothing
    let config = Config::new_f64(14, 50);

    // Calculate RSI
    let rsi_values = rsi_series(&candles, config)?;

    println!("RSI (Relative Strength Index) Demo");
    println!("===================================");
    println!("Candles: {}", candles.len());
    println!("RSI values: {}", rsi_values.len());
    println!();

    // Display the last 10 RSI values with prices
    let start_idx = rsi_values.len().saturating_sub(10);
    for i in start_idx..rsi_values.len() {
        let price_idx = i + (candles.len() - rsi_values.len());
        println!(
            "Price: {:.2} | RSI[{}]: {:.2}",
            prices[price_idx], i, rsi_values[i]
        );
    }

    println!();
    if let Some(&last_rsi) = rsi_values.last() {
        let interpretation = match last_rsi {
            rsi if rsi > 70.0 => "Overbought (>70)",
            rsi if rsi < 30.0 => "Oversold (<30)",
            _ => "Neutral (30-70)",
        };
        println!("Latest RSI: {:.2} - {}", last_rsi, interpretation);
    }

    Ok(())
}
