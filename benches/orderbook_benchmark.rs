use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_market_data_processor::OrderBook;

fn orderbook_updates(c: &mut Criterion) {
    c.bench_function("orderbook_update_bid", |b| {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        let mut price = 50000.0;
        
        b.iter(|| {
            ob.update_bid(black_box(price), black_box(1.0));
            price += 0.01;
        });
    });

    c.bench_function("orderbook_update_ask", |b| {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        let mut price = 50000.0;
        
        b.iter(|| {
            ob.update_ask(black_box(price), black_box(1.0));
            price += 0.01;
        });
    });

    c.bench_function("orderbook_best_bid_ask", |b| {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        
        // Pre-populate orderbook
        for i in 0..100 {
            ob.update_bid(50000.0 - i as f64, 1.0);
            ob.update_ask(50001.0 + i as f64, 1.0);
        }
        
        b.iter(|| {
            black_box(ob.best_bid());
            black_box(ob.best_ask());
        });
    });

    c.bench_function("orderbook_mid_price_spread", |b| {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        
        // Pre-populate orderbook
        for i in 0..100 {
            ob.update_bid(50000.0 - i as f64, 1.0);
            ob.update_ask(50001.0 + i as f64, 1.0);
        }
        
        b.iter(|| {
            black_box(ob.mid_price());
            black_box(ob.spread());
            black_box(ob.spread_percentage());
        });
    });

    c.bench_function("orderbook_volume_calculations", |b| {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        
        // Pre-populate orderbook
        for i in 0..100 {
            ob.update_bid(50000.0 - i as f64, (i + 1) as f64);
            ob.update_ask(50001.0 + i as f64, (i + 1) as f64);
        }
        
        b.iter(|| {
            black_box(ob.total_bid_volume());
            black_box(ob.total_ask_volume());
            black_box(ob.volume_imbalance());
        });
    });
}

criterion_group!(benches, orderbook_updates);
criterion_main!(benches);
