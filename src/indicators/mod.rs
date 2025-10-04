use std::collections::VecDeque;

/// Simple Moving Average calculator
pub struct SMA {
    period: usize,
    values: VecDeque<f64>,
}

impl SMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            values: VecDeque::with_capacity(period),
        }
    }

    pub fn update(&mut self, value: f64) -> Option<f64> {
        self.values.push_back(value);
        
        if self.values.len() > self.period {
            self.values.pop_front();
        }
        
        if self.values.len() == self.period {
            Some(self.values.iter().sum::<f64>() / self.period as f64)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.values.clear();
    }
}

/// Exponential Moving Average calculator
pub struct EMA {
    period: usize,
    multiplier: f64,
    current: Option<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        let multiplier = 2.0 / (period as f64 + 1.0);
        Self {
            period,
            multiplier,
            current: None,
        }
    }

    pub fn update(&mut self, value: f64) -> Option<f64> {
        match self.current {
            Some(prev) => {
                let ema = (value - prev) * self.multiplier + prev;
                self.current = Some(ema);
                Some(ema)
            }
            None => {
                self.current = Some(value);
                Some(value)
            }
        }
    }

    pub fn reset(&mut self) {
        self.current = None;
    }
}

/// RSI (Relative Strength Index) calculator
pub struct RSI {
    period: usize,
    gains: VecDeque<f64>,
    losses: VecDeque<f64>,
    prev_close: Option<f64>,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            gains: VecDeque::with_capacity(period),
            losses: VecDeque::with_capacity(period),
            prev_close: None,
        }
    }

    pub fn update(&mut self, close: f64) -> Option<f64> {
        if let Some(prev) = self.prev_close {
            let change = close - prev;
            
            if change > 0.0 {
                self.gains.push_back(change);
                self.losses.push_back(0.0);
            } else {
                self.gains.push_back(0.0);
                self.losses.push_back(change.abs());
            }
            
            if self.gains.len() > self.period {
                self.gains.pop_front();
                self.losses.pop_front();
            }
            
            if self.gains.len() == self.period {
                let avg_gain = self.gains.iter().sum::<f64>() / self.period as f64;
                let avg_loss = self.losses.iter().sum::<f64>() / self.period as f64;
                
                if avg_loss == 0.0 {
                    return Some(100.0);
                }
                
                let rs = avg_gain / avg_loss;
                let rsi = 100.0 - (100.0 / (1.0 + rs));
                
                self.prev_close = Some(close);
                return Some(rsi);
            }
        }
        
        self.prev_close = Some(close);
        None
    }

    pub fn reset(&mut self) {
        self.gains.clear();
        self.losses.clear();
        self.prev_close = None;
    }
}

/// Bollinger Bands calculator
pub struct BollingerBands {
    sma: SMA,
    period: usize,
    std_dev: f64,
    values: VecDeque<f64>,
}

impl BollingerBands {
    pub fn new(period: usize, std_dev: f64) -> Self {
        Self {
            sma: SMA::new(period),
            period,
            std_dev,
            values: VecDeque::with_capacity(period),
        }
    }

    pub fn update(&mut self, value: f64) -> Option<(f64, f64, f64)> {
        self.values.push_back(value);
        
        if self.values.len() > self.period {
            self.values.pop_front();
        }
        
        if let Some(middle) = self.sma.update(value) {
            if self.values.len() == self.period {
                let variance = self.values
                    .iter()
                    .map(|v| (v - middle).powi(2))
                    .sum::<f64>() / self.period as f64;
                
                let std = variance.sqrt();
                let upper = middle + (self.std_dev * std);
                let lower = middle - (self.std_dev * std);
                
                return Some((upper, middle, lower));
            }
        }
        
        None
    }

    pub fn reset(&mut self) {
        self.sma.reset();
        self.values.clear();
    }
}

/// MACD (Moving Average Convergence Divergence) calculator
pub struct MACD {
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
}

impl MACD {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast_ema: EMA::new(fast_period),
            slow_ema: EMA::new(slow_period),
            signal_ema: EMA::new(signal_period),
        }
    }

    pub fn update(&mut self, close: f64) -> Option<(f64, f64, f64)> {
        if let (Some(fast), Some(slow)) = (self.fast_ema.update(close), self.slow_ema.update(close)) {
            let macd_line = fast - slow;
            
            if let Some(signal_line) = self.signal_ema.update(macd_line) {
                let histogram = macd_line - signal_line;
                return Some((macd_line, signal_line, histogram));
            }
        }
        
        None
    }

    pub fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let mut sma = SMA::new(3);
        
        assert_eq!(sma.update(10.0), None);
        assert_eq!(sma.update(20.0), None);
        assert_eq!(sma.update(30.0), Some(20.0));
        assert_eq!(sma.update(40.0), Some(30.0));
    }

    #[test]
    fn test_ema() {
        let mut ema = EMA::new(3);
        
        let result1 = ema.update(10.0);
        assert!(result1.is_some());
        
        let result2 = ema.update(20.0);
        assert!(result2.is_some());
        assert!(result2.unwrap() > 10.0);
    }

    #[test]
    fn test_rsi() {
        let mut rsi = RSI::new(14);
        
        // Feed some data
        for i in 1..=20 {
            rsi.update(50.0 + (i as f64));
        }
        
        let result = rsi.update(70.0);
        assert!(result.is_some());
        
        let rsi_value = result.unwrap();
        assert!(rsi_value >= 0.0 && rsi_value <= 100.0);
    }

    #[test]
    fn test_bollinger_bands() {
        let mut bb = BollingerBands::new(20, 2.0);
        
        // Feed some data
        for i in 1..=25 {
            let result = bb.update(50.0 + (i as f64 % 10) as f64);
            
            if i >= 20 {
                assert!(result.is_some());
                let (upper, middle, lower) = result.unwrap();
                assert!(upper > middle);
                assert!(middle > lower);
            }
        }
    }

    #[test]
    fn test_macd() {
        let mut macd = MACD::new(12, 26, 9);
        
        // Feed some data
        for i in 1..=50 {
            macd.update(50.0 + (i as f64));
        }
        
        let result = macd.update(100.0);
        assert!(result.is_some());
    }
}
