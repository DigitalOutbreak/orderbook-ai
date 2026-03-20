use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn market_orders_never_rest_and_empty_levels_are_removed() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    let result = book.submit_order(NewOrderRequest::market(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.500)),
    ));

    assert!(result.events.iter().any(|event| matches!(
        event,
        BookEvent::MarketOrderUnfilled { unfilled_qty, .. } if *unfilled_qty == Quantity::new(dec!(0.500))
    )));
    assert!(book.best_ask().is_none());
    assert!(book.best_bid().is_none());
    assert!(result.error.is_none());
}

#[test]
fn top_of_book_change_is_emitted_after_trade_and_cancel() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let resting_bid = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(99.00)),
    ));

    let crossing_sell = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(99.00)),
    ));

    assert!(crossing_sell.events.iter().any(|event| matches!(
        event,
        BookEvent::TopOfBookChanged { top_of_book } if top_of_book.best_bid.is_none()
    )));

    let resting_ask = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    ));

    let cancel = book.cancel_order(resting_ask.order_id.unwrap());

    assert!(resting_bid.accepted);
    assert!(cancel.events.iter().any(|event| matches!(
        event,
        BookEvent::TopOfBookChanged { top_of_book } if top_of_book.best_ask.is_none()
    )));
    assert!(cancel.error.is_none());
}
