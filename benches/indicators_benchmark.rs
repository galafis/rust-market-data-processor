use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_market_data_processor::{SMA, EMA, RSI, MACD, BollingerBands};

fn indicators_benchmark(c: &mut Criterion) {
    c.bench_function("sma_update", |b| {
        let mut sma = SMA::new(20);
        let mut price = 50000.0;
        
        b.iter(|| {
            sma.update(black_box(price));
            price += 1.0;
        });
    });

    c.bench_function("ema_update", |b| {
        let mut ema = EMA::new(20);
        let mut price = 50000.0;
        
        b.iter(|| {
            ema.update(black_box(price));
            price += 1.0;
        });
    });

    c.bench_function("rsi_update", |b| {
        let mut rsi = RSI::new(14);
        let mut price = 50000.0;
        
        b.iter(|| {
            rsi.update(black_box(price));
            price += 1.0;
        });
    });

    c.bench_function("macd_update", |b| {
        let mut macd = MACD::new(12, 26, 9);
        let mut price = 50000.0;
        
        b.iter(|| {
            macd.update(black_box(price));
            price += 1.0;
        });
    });

    c.bench_function("bollinger_bands_update", |b| {
        let mut bb = BollingerBands::new(20, 2.0);
        let mut price = 50000.0;
        
        b.iter(|| {
            bb.update(black_box(price));
            price += 1.0;
        });
    });

    c.bench_function("all_indicators_combined", |b| {
        let mut sma = SMA::new(20);
        let mut ema = EMA::new(20);
        let mut rsi = RSI::new(14);
        let mut macd = MACD::new(12, 26, 9);
        let mut bb = BollingerBands::new(20, 2.0);
        let mut price = 50000.0;
        
        b.iter(|| {
            sma.update(black_box(price));
            ema.update(black_box(price));
            rsi.update(black_box(price));
            macd.update(black_box(price));
            bb.update(black_box(price));
            price += 1.0;
        });
    });
}

criterion_group!(benches, indicators_benchmark);
criterion_main!(benches);
