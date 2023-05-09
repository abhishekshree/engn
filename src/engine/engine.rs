use std::collections::HashMap;

use super::orderbook::{OrderBook, Order};

// BTC/USD
// BTC -> Base
// USD -> Quote

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {     
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_market(&mut self, trading_pair: TradingPair) {
        self.orderbooks.insert(trading_pair.clone(), OrderBook::new());
        print!("New market added: {:?}", trading_pair.to_string());
    }

    pub fn place_limit_order(&mut self, trading_pair: TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&trading_pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            },
            None => Err(String::from("No such market")),
        }
    }
}
