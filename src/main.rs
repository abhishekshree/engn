mod engine;

use engine::orderbook::{Order, OrderBook, OrderType};
// use engine::engine::MatchingEngine;

fn main() {
    // let price = Price::new(1.23);

    let buy_order_from_alice = Order::new(5.5, OrderType::Bid);
    let buy_order_from_bob = Order::new(1.5, OrderType::Bid);

    let mut order_book = OrderBook::new();
    order_book.add_order(1.23, buy_order_from_alice);
    order_book.add_order(1.23, buy_order_from_bob);

    let sell_order_from_alice = Order::new(6.5, OrderType::Ask);
    order_book.add_order(20.0, sell_order_from_alice);

    print!("{:?}", order_book);

    // let eng = MatchingEngine::new();
}
