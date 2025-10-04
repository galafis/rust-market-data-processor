use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Price level in the order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub quantity: f64,
}

/// Order book for a trading symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<OrderedFloat, f64>,
    pub asks: BTreeMap<OrderedFloat, f64>,
    pub last_update: i64,
}

/// Wrapper for f64 to make it orderable in BTreeMap
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OrderedFloat(pub f64);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl OrderBook {
    /// Create a new order book
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update: 0,
        }
    }

    /// Update bid level
    pub fn update_bid(&mut self, price: f64, quantity: f64) {
        let key = OrderedFloat(price);
        if quantity == 0.0 {
            self.bids.remove(&key);
        } else {
            self.bids.insert(key, quantity);
        }
    }

    /// Update ask level
    pub fn update_ask(&mut self, price: f64, quantity: f64) {
        let key = OrderedFloat(price);
        if quantity == 0.0 {
            self.asks.remove(&key);
        } else {
            self.asks.insert(key, quantity);
        }
    }

    /// Get best bid (highest buy price)
    pub fn best_bid(&self) -> Option<(f64, f64)> {
        self.bids.iter().next_back().map(|(k, v)| (k.0, *v))
    }

    /// Get best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<(f64, f64)> {
        self.asks.iter().next().map(|(k, v)| (k.0, *v))
    }

    /// Get mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    /// Get spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid > 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Get top N levels of bids
    pub fn top_bids(&self, n: usize) -> Vec<PriceLevel> {
        self.bids
            .iter()
            .rev()
            .take(n)
            .map(|(k, v)| PriceLevel {
                price: k.0,
                quantity: *v,
            })
            .collect()
    }

    /// Get top N levels of asks
    pub fn top_asks(&self, n: usize) -> Vec<PriceLevel> {
        self.asks
            .iter()
            .take(n)
            .map(|(k, v)| PriceLevel {
                price: k.0,
                quantity: *v,
            })
            .collect()
    }

    /// Calculate total volume at bid side
    pub fn total_bid_volume(&self) -> f64 {
        self.bids.values().sum()
    }

    /// Calculate total volume at ask side
    pub fn total_ask_volume(&self) -> f64 {
        self.asks.values().sum()
    }

    /// Calculate volume imbalance
    pub fn volume_imbalance(&self) -> f64 {
        let bid_vol = self.total_bid_volume();
        let ask_vol = self.total_ask_volume();
        let total = bid_vol + ask_vol;
        
        if total > 0.0 {
            (bid_vol - ask_vol) / total
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_creation() {
        let ob = OrderBook::new("BTCUSD".to_string());
        assert_eq!(ob.symbol, "BTCUSD");
        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
    }

    #[test]
    fn test_update_bid() {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        ob.update_bid(50000.0, 1.5);
        ob.update_bid(49999.0, 2.0);
        
        assert_eq!(ob.bids.len(), 2);
        assert_eq!(ob.best_bid(), Some((50000.0, 1.5)));
    }

    #[test]
    fn test_update_ask() {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        ob.update_ask(50001.0, 1.0);
        ob.update_ask(50002.0, 1.5);
        
        assert_eq!(ob.asks.len(), 2);
        assert_eq!(ob.best_ask(), Some((50001.0, 1.0)));
    }

    #[test]
    fn test_mid_price() {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        ob.update_bid(50000.0, 1.0);
        ob.update_ask(50002.0, 1.0);
        
        assert_eq!(ob.mid_price(), Some(50001.0));
    }

    #[test]
    fn test_spread() {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        ob.update_bid(50000.0, 1.0);
        ob.update_ask(50002.0, 1.0);
        
        assert_eq!(ob.spread(), Some(2.0));
    }

    #[test]
    fn test_volume_imbalance() {
        let mut ob = OrderBook::new("BTCUSD".to_string());
        ob.update_bid(50000.0, 3.0);
        ob.update_ask(50001.0, 1.0);
        
        let imbalance = ob.volume_imbalance();
        assert!(imbalance > 0.0); // More bids than asks
    }
}
