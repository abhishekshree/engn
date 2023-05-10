#[allow(dead_code)]
use std::collections::HashMap;
#[derive(Debug)]
pub enum OrderType {
    Bid,
    Ask,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Price {
    integer: u64,
    fraction: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integer = price.trunc() as u64;
        let fraction = (price.fract() * scalar as f64).trunc() as u64;
        Price {
            integer,
            fraction,
            scalar,
        }
    }

    pub fn cmp(&self, other: &Price) -> std::cmp::Ordering {
        match self.integer.cmp(&other.integer) {
            std::cmp::Ordering::Equal => match self.fraction.cmp(&other.fraction) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            },
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        }
    }
}
#[derive(Debug)]
pub struct Limit {
    // price: f64, f64 bad because of inconsistent hashing
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new(),
        }
    }

    pub fn volume(&self) -> f64 {
        self.orders.iter().fold(0.0, |acc, order| acc + order.size)
    }

    pub fn get_size_by_order_id(&self, order_id: u64) -> Option<f64> {
        match self.orders.get(order_id as usize) {
            Some(order) => Some(order.size),
            None => None,
        }
    }

    pub fn is_filled_by_order_id(&self, order_id: u64) -> Option<bool> {
        match self.orders.get(order_id as usize) {
            Some(order) => Some(order.is_filled()),
            None => None,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    pub fn new(size: f64, order_type: OrderType) -> Order {
        Order { size, order_type }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        match order.order_type {
            OrderType::Bid => {
                let price = Price::new(price);

                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        // println!("limit: {:?} already got a limit", limit);
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            OrderType::Ask => {
                let price = Price::new(price);

                match self.asks.get_mut(&price) {
                    Some(limit) => {
                        // println!("limit: {:?} already got a limit", limit);
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.asks.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| a.price.cmp(&b.price));
        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.bids.values_mut().collect::<Vec<&mut Limit>>();
        limits.sort_by(|a, b| b.price.cmp(&a.price));
        limits
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.order_type {
            OrderType::Bid => self.ask_limits(),
            OrderType::Ask => self.bid_limits(),
        };

        for limit in limits {
            limit.fill_order(market_order);
            if market_order.is_filled() {
                break;
            }
        }
    }
}
