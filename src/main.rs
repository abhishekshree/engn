enum OrderType {
    Bid,
    Ask,
}

#[derive(Eq, PartialEq, Debug)]
struct Price {
    integer: u64,
    fraction: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
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

struct Limit {  
    // price: f64, f64 bad because of inconsistent hashing
    price: Price,
    orders: Vec<Order>,
}

struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    fn new(size: f64, order_type: OrderType) -> Order {
        Order {
            size,
            order_type,
        }
    }
}

fn main() {
    let price = Price::new(1.23);
    println!("{:?}", price);
}
