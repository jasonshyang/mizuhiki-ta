use mizuhiki_ta::{
    core::{Candle, CandleSeries},
    indicators::{Config, natr_series},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut candles = CandleSeries::<f64>::new(60_000);

    let sample_data = [
        (100.0, 105.0, 98.0, 102.0, 1000.0),
        (102.0, 108.0, 99.0, 106.0, 1200.0),
        (106.0, 110.0, 103.0, 108.0, 1100.0),
        (108.0, 112.0, 105.0, 109.0, 900.0),
        (109.0, 115.0, 107.0, 113.0, 1300.0),
        (113.0, 117.0, 111.0, 115.0, 1000.0),
        (115.0, 119.0, 113.0, 117.0, 1100.0),
        (117.0, 121.0, 115.0, 119.0, 1200.0),
        (119.0, 123.0, 117.0, 121.0, 1000.0),
        (121.0, 125.0, 119.0, 123.0, 1100.0),
        (123.0, 127.0, 121.0, 125.0, 1300.0),
        (125.0, 129.0, 123.0, 127.0, 1000.0),
        (127.0, 131.0, 125.0, 129.0, 1200.0),
        (129.0, 133.0, 127.0, 131.0, 1100.0),
        (131.0, 135.0, 129.0, 133.0, 1000.0),
    ];

    for (i, (open, high, low, close, volume)) in sample_data.iter().enumerate() {
        let timestamp = (i as u64) * 60_000; // 1-minute intervals
        let candle = Candle {
            open: *open,
            high: *high,
            low: *low,
            close: *close,
            volume: *volume,
        };
        candles.push_candle_unchecked(candle, timestamp);
    }

    // Configure NATR with 14-period and Wilder smoothing
    let config = Config::new_f64_wilder(14, 50);

    // Calculate NATR
    let natr_values = natr_series(&candles, &config)?;

    println!("NATR (Normalized Average True Range) Demo");
    println!("==========================================");
    println!("Candles: {}", candles.len());
    println!("NATR values: {}", natr_values.len());
    println!();

    // Display the last 5 NATR values
    let start_idx = natr_values.len().saturating_sub(5);
    for i in start_idx..natr_values.len() {
        println!("NATR[{}]: {:.4}%", i, natr_values[i]);
    }

    Ok(())
}
