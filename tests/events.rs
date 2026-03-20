use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn submission_event_sequence_is_explicit_for_resting_limit_order() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(99.00)),
    ));

    assert_eq!(result.events.len(), 3);
    assert!(matches!(result.events[0], BookEvent::OrderAccepted { .. }));
    assert!(matches!(result.events[1], BookEvent::OrderRested { .. }));
    assert!(matches!(
        result.events[2],
        BookEvent::TopOfBookChanged { .. }
    ));
}

#[test]
fn crossing_limit_event_sequence_reports_trade_before_top_of_book_change() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    assert!(matches!(result.events[0], BookEvent::OrderAccepted { .. }));
    assert!(matches!(result.events[1], BookEvent::TradeExecuted { .. }));
    assert!(matches!(
        result.events.last(),
        Some(BookEvent::TopOfBookChanged { .. })
    ));
}

#[test]
fn top_of_book_tracks_best_prices_after_multi_step_flow() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(99.00)),
    ));
    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));
    let ask = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    ));

    assert_eq!(
        book.top_of_book().best_bid.unwrap().price,
        Price::new(dec!(100.00))
    );
    assert_eq!(
        book.top_of_book().best_ask.unwrap().price,
        Price::new(dec!(101.00))
    );

    book.cancel_order(ask.order_id.unwrap());

    let top = book.top_of_book();
    assert_eq!(top.best_bid.unwrap().price, Price::new(dec!(100.00)));
    assert!(top.best_ask.is_none());
}
