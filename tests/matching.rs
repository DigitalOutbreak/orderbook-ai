use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn resting_limit_orders_set_top_of_book() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let bid = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(99.00)),
    ));
    let ask = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.500)),
        Price::new(dec!(101.00)),
    ));

    assert!(bid.accepted);
    assert!(ask.accepted);
    assert_eq!(book.best_bid().unwrap().price, Price::new(dec!(99.00)));
    assert_eq!(book.best_ask().unwrap().price, Price::new(dec!(101.00)));
}

#[test]
fn crossing_limit_order_matches_better_prices_first() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));
    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    ));

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(101.00)),
    ));

    let trade_prices: Vec<_> = result
        .events
        .iter()
        .filter_map(|event| match event {
            BookEvent::TradeExecuted { trade } => Some(trade.price),
            _ => None,
        })
        .collect();

    assert_eq!(
        trade_prices,
        vec![Price::new(dec!(100.00)), Price::new(dec!(101.00))]
    );
    assert!(book.best_ask().is_none());
}
