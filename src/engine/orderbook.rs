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

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    pub fn new(size: f64, order_type: OrderType) -> Order {
        Order {
            size,
            order_type,
        }
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
                    Some(limit) =>{
                        // println!("limit: {:?} already got a limit", limit);
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }

            },
            OrderType::Ask => {
                let price = Price::new(price);
                
                match self.asks.get_mut(&price) {
                    Some(limit) => {
                        // println!("limit: {:?} already got a limit", limit);
                        limit.add_order(order);
                    },
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            },
        }
    }
}
