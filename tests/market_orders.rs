use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn market_buy_sweeps_multiple_ask_levels() {
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
        Quantity::new(dec!(2.000)),
        Price::new(dec!(101.00)),
    ));

    let result = book.submit_order(NewOrderRequest::market(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.500)),
    ));

    let trades: Vec<_> = result
        .events
        .iter()
        .filter_map(|event| match event {
            BookEvent::TradeExecuted { trade } => Some((trade.price, trade.quantity)),
            _ => None,
        })
        .collect();

    assert_eq!(
        trades,
        vec![
            (Price::new(dec!(100.00)), Quantity::new(dec!(1.000))),
            (Price::new(dec!(101.00)), Quantity::new(dec!(1.500))),
        ]
    );
    assert!(result.fully_filled);
    assert_eq!(book.best_ask().unwrap().price, Price::new(dec!(101.00)));
    assert_eq!(
        book.best_ask().unwrap().total_quantity,
        Quantity::new(dec!(0.500))
    );
}

#[test]
fn market_order_remainder_expires_and_never_rests() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let result = book.submit_order(NewOrderRequest::market(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
    ));

    assert!(!result.fully_filled);
    assert_eq!(book.snapshot(1).bids.len(), 0);
    assert_eq!(book.snapshot(1).asks.len(), 0);
    assert!(result.events.iter().any(|event| matches!(
        event,
        BookEvent::MarketOrderUnfilled { unfilled_qty, .. } if *unfilled_qty == Quantity::new(dec!(1.000))
    )));
}
