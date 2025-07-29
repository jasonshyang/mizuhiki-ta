use mizuhiki_ta::core::series::Series;
use mizuhiki_ta::indicators::natr::{NatrConfig, natr};

fn main() {
    println!("=== NATR Demo ===");
    let sample_data = vec![
        (100.0, 102.5, 99.5, 101.0),
        (101.5, 103.0, 100.5, 102.0),
        (102.2, 103.8, 101.8, 103.5),
        (103.0, 104.5, 102.0, 102.5),
        (102.8, 105.0, 102.0, 104.8),
        (105.2, 106.0, 104.0, 105.5),
        (105.0, 105.8, 103.5, 104.0),
        (104.2, 105.5, 103.0, 105.0),
        (104.8, 106.5, 104.0, 106.2),
        (106.0, 107.0, 105.0, 106.8),
        (107.2, 108.0, 106.0, 106.5),
        (106.8, 107.5, 105.5, 107.0),
        (106.5, 108.5, 105.0, 108.0),
        (108.2, 109.0, 107.0, 107.5),
        (107.0, 107.8, 105.5, 106.0),
        (106.2, 107.0, 104.0, 104.5),
        (104.8, 105.5, 102.0, 102.8),
        (103.0, 104.0, 101.5, 103.5),
        (103.2, 104.8, 102.0, 104.2),
        (104.0, 105.0, 103.0, 104.5),
    ];

    let mut opens = Vec::new();
    let mut highs = Vec::new();
    let mut lows = Vec::new();
    let mut closes = Vec::new();

    for (o, h, l, c) in sample_data {
        opens.push(o);
        highs.push(h);
        lows.push(l);
        closes.push(c);
    }

    let open_series = Series::from_vec("AAPL_open".to_string(), opens);
    let high_series = Series::from_vec("AAPL_high".to_string(), highs);
    let low_series = Series::from_vec("AAPL_low".to_string(), lows);
    let close_series = Series::from_vec("AAPL_close".to_string(), closes);

    println!("Processing {} price bars\n", close_series.len());

    let config = NatrConfig::<f64>::default();
    let result = natr(&high_series, &low_series, &close_series, config);

    println!("--- OHLC Data and NATR Values (last 10 days) ---");
    let start_idx = (close_series.len().saturating_sub(10)).max(0);

    for i in start_idx..close_series.len() {
        let day = i + 1;
        let open = open_series.values()[i];
        let high = high_series.values()[i];
        let low = low_series.values()[i];
        let close = close_series.values()[i];
        let tr = result.true_range.values()[i];
        let atr = result.atr.values()[i];
        let natr = result.natr.values()[i];

        let volatility_level = if natr > 4.0 {
            "HIGH"
        } else if natr > 2.0 {
            "MEDIUM"
        } else {
            "LOW"
        };

        println!(
            "Day {:2}: O=${:6.2} H=${:6.2} L=${:6.2} C=${:6.2} | TR=${:4.2} ATR=${:4.2} NATR={:4.1}% ({})",
            day, open, high, low, close, tr, atr, natr, volatility_level
        );
    }

    println!("\n--- Volatility Analysis ---");
    let latest_natr = *result.natr.values().last().unwrap();
    let latest_atr = *result.atr.values().last().unwrap();
    let latest_close = *close_series.values().last().unwrap();

    println!("Current price: ${:.2}", latest_close);
    println!("Current ATR: ${:.2}", latest_atr);
    println!("Current NATR: {:.2}%", latest_natr);

    let interpretation = match latest_natr {
        n if n > 5.0 => "Very High Volatility - Consider wider stops, potential breakout/breakdown",
        n if n > 3.0 => "High Volatility - Good for swing trading, watch for direction",
        n if n > 2.0 => "Moderate Volatility - Normal trading conditions",
        n if n > 1.0 => "Low Volatility - Potential consolidation, breakout may be coming",
        _ => "Very Low Volatility - Market may be coiling for significant move",
    };

    println!("Interpretation: {}", interpretation);

    let avg_natr: f64 = result.natr.values().iter().sum::<f64>() / result.natr.len() as f64;
    println!("Average NATR over period: {:.2}%", avg_natr);

    let mut max_natr = 0.0;
    let mut min_natr = f64::MAX;
    let mut max_day = 0;
    let mut min_day = 0;

    for (i, &natr_val) in result.natr.values().iter().enumerate() {
        if natr_val > max_natr {
            max_natr = natr_val;
            max_day = i + 1;
        }
        if natr_val < min_natr {
            min_natr = natr_val;
            min_day = i + 1;
        }
    }

    println!("Highest volatility: {:.2}% on day {}", max_natr, max_day);
    println!("Lowest volatility: {:.2}% on day {}", min_natr, min_day);
}
