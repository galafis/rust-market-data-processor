use rust_market_data_processor::{OrderBook, SMA, EMA, RSI, MACD};
use tracing::{info, Level};
use tracing_subscriber;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Rust Market Data Processor");

    // Demonstrate OrderBook
    demo_orderbook();

    // Demonstrate Technical Indicators
    demo_indicators();

    info!("Demo completed successfully");
}

fn demo_orderbook() {
    info!("=== OrderBook Demo ===");
    
    let mut ob = OrderBook::new("BTCUSD".to_string());
    
    // Add some bids
    ob.update_bid(50000.0, 1.5);
    ob.update_bid(49999.0, 2.0);
    ob.update_bid(49998.0, 1.0);
    
    // Add some asks
    ob.update_ask(50001.0, 1.0);
    ob.update_ask(50002.0, 1.5);
    ob.update_ask(50003.0, 2.0);
    
    info!("Symbol: {}", ob.symbol);
    
    if let Some((price, qty)) = ob.best_bid() {
        info!("Best Bid: ${:.2} @ {:.4}", price, qty);
    }
    
    if let Some((price, qty)) = ob.best_ask() {
        info!("Best Ask: ${:.2} @ {:.4}", price, qty);
    }
    
    if let Some(mid) = ob.mid_price() {
        info!("Mid Price: ${:.2}", mid);
    }
    
    if let Some(spread) = ob.spread() {
        info!("Spread: ${:.2}", spread);
    }
    
    if let Some(spread_pct) = ob.spread_percentage() {
        info!("Spread %: {:.4}%", spread_pct);
    }
    
    let imbalance = ob.volume_imbalance();
    info!("Volume Imbalance: {:.4}", imbalance);
    
    info!("Top 3 Bids:");
    for level in ob.top_bids(3) {
        info!("  ${:.2} @ {:.4}", level.price, level.quantity);
    }
    
    info!("Top 3 Asks:");
    for level in ob.top_asks(3) {
        info!("  ${:.2} @ {:.4}", level.price, level.quantity);
    }
}

fn demo_indicators() {
    info!("=== Technical Indicators Demo ===");
    
    // Sample price data
    let prices = vec![
        50000.0, 50100.0, 50050.0, 50200.0, 50150.0,
        50300.0, 50250.0, 50400.0, 50350.0, 50500.0,
        50450.0, 50600.0, 50550.0, 50700.0, 50650.0,
        50800.0, 50750.0, 50900.0, 50850.0, 51000.0,
    ];
    
    // SMA Demo
    info!("--- SMA (10 period) ---");
    let mut sma = SMA::new(10);
    for (i, &price) in prices.iter().enumerate() {
        if let Some(value) = sma.update(price) {
            info!("Price: ${:.2}, SMA: ${:.2}", price, value);
        } else {
            info!("Price: ${:.2}, SMA: warming up ({}/10)", price, i + 1);
        }
    }
    
    // EMA Demo
    info!("--- EMA (10 period) ---");
    let mut ema = EMA::new(10);
    for &price in prices.iter().take(5) {
        if let Some(value) = ema.update(price) {
            info!("Price: ${:.2}, EMA: ${:.2}", price, value);
        }
    }
    
    // RSI Demo
    info!("--- RSI (14 period) ---");
    let mut rsi = RSI::new(14);
    for &price in prices.iter() {
        if let Some(value) = rsi.update(price) {
            info!("Price: ${:.2}, RSI: {:.2}", price, value);
        }
    }
    
    // MACD Demo
    info!("--- MACD (12, 26, 9) ---");
    let mut macd = MACD::new(12, 26, 9);
    for &price in prices.iter() {
        if let Some((macd_line, signal, histogram)) = macd.update(price) {
            info!(
                "Price: ${:.2}, MACD: {:.2}, Signal: {:.2}, Histogram: {:.2}",
                price, macd_line, signal, histogram
            );
        }
    }
}
