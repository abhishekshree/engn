#[cfg(test)]
use super::orderbook::*;

#[test]
fn limit_order_fill() {
    let price = Price::new(10000.0);
    let mut limit = Limit::new(price);

    let buy_limit_order = Order::new(100.0, OrderType::Bid);
    limit.add_order(buy_limit_order);

    let mut sell_market_order = Order::new(99.0, OrderType::Ask);
    limit.fill_order(&mut sell_market_order);

    assert_eq!(sell_market_order.is_filled(), true);
    assert_eq!(limit.get_size_by_order_id(0), Some(1.0));
}

#[test]
fn limit_order_multi_fill() {
    let price = Price::new(10000.0);
    let mut limit = Limit::new(price);
    let buy_limit_order_a = Order::new(100.0, OrderType::Bid);
    let buy_limit_order_b = Order::new(100.0, OrderType::Bid);
    limit.add_order(buy_limit_order_a);
    limit.add_order(buy_limit_order_b);

    let mut market_sell_order = Order::new(199.0, OrderType::Ask);
    limit.fill_order(&mut market_sell_order);

    assert_eq!(market_sell_order.is_filled(), true);
    assert_eq!(limit.is_filled_by_order_id(0), Some(true));
    assert_eq!(limit.is_filled_by_order_id(1), Some(false));
    assert_eq!(limit.get_size_by_order_id(1), Some(1.0));
}

#[test]
fn limit_total_volume() {
    let price = Price::new(10000.0);
    let mut limit = Limit::new(price);
    let buy_limit_order_a = Order::new(100.0, OrderType::Bid);
    let buy_limit_order_b = Order::new(100.0, OrderType::Bid);

    limit.add_order(buy_limit_order_a);
    limit.add_order(buy_limit_order_b);

    assert_eq!(limit.volume(), 200.0)
}

#[test]
fn fill_market_order_ask() {
    let mut order_book = OrderBook::new();
    order_book.add_order(10000.0, Order::new(100.0, OrderType::Ask));
    order_book.add_order(100.0, Order::new(100.0, OrderType::Ask));
    order_book.add_order(500.0, Order::new(100.0, OrderType::Ask));
    order_book.add_order(200.0, Order::new(100.0, OrderType::Ask));

    let mut market_order = Order::new(300.0, OrderType::Bid);
    order_book.fill_market_order(&mut market_order);

    assert_eq!(market_order.is_filled(), true);
}
