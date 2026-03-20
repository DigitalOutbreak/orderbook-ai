use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn cancel_existing_order_removes_it_from_the_book() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let submission = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(99.00)),
    ));

    let cancel = book.cancel_order(submission.order_id.unwrap());

    assert!(cancel.cancelled);
    assert_eq!(cancel.cancelled_qty, Quantity::new(dec!(2.000)));
    assert!(book.best_bid().is_none());
}

#[test]
fn cancel_missing_order_returns_typed_error() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config);

    let result = book.cancel_order(solbook_core::OrderId::new(999));

    assert!(!result.cancelled);
    assert_eq!(
        result.error,
        Some(solbook_core::EngineError::OrderNotFound(
            solbook_core::OrderId::new(999)
        ))
    );
}

#[test]
fn cancel_after_partial_fill_removes_only_remaining_resting_quantity() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let maker = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(3.000)),
        Price::new(dec!(101.00)),
    ));

    let taker = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.250)),
        Price::new(dec!(101.00)),
    ));

    let cancel = book.cancel_order(maker.order_id.unwrap());

    assert!(taker.accepted);
    assert!(cancel.cancelled);
    assert_eq!(cancel.cancelled_qty, Quantity::new(dec!(1.750)));
    assert!(cancel.events.iter().any(|event| matches!(
        event,
        BookEvent::OrderCancelled { cancelled_qty, .. } if *cancelled_qty == Quantity::new(dec!(1.750))
    )));
    assert!(book.best_ask().is_none());
}

#[test]
fn partial_fill_then_cancel_preserves_bid_side_top_of_book() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(100.00)),
    ));
    let ask = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(101.00)),
    ));

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.250)),
        Price::new(dec!(101.00)),
    ));

    let cancel = book.cancel_order(ask.order_id.unwrap());

    assert!(cancel.cancelled);
    assert_eq!(book.best_bid().unwrap().price, Price::new(dec!(100.00)));
    assert!(book.best_ask().is_none());
}
