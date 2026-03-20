use rust_decimal_macros::dec;
use solbook_core::{
    InvariantPolicy, MarketConfig, NewOrderRequest, OrderBook, OrderId, Price, Quantity, Side,
};

#[test]
fn minimal_submission_matches_rich_submission_book_state() {
    let config = MarketConfig::sol_usdc();

    let mut rich_book = OrderBook::new(config.clone());
    let rich = rich_book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    let mut minimal_book = OrderBook::new(config.clone());
    let minimal = minimal_book.submit_order_minimal(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    assert_eq!(minimal.accepted, rich.accepted);
    assert_eq!(minimal.order_id, rich.order_id);
    assert_eq!(minimal.fully_filled, rich.fully_filled);
    assert_eq!(minimal.remaining_qty, rich.remaining_qty);
    assert_eq!(minimal.error, rich.error);
    assert_eq!(minimal_book.top_of_book(), rich_book.top_of_book());
    assert_eq!(minimal_book.snapshot(5), rich_book.snapshot(5));
}

#[test]
fn minimal_cancel_matches_rich_cancel_book_state() {
    let config = MarketConfig::sol_usdc();

    let mut rich_book = OrderBook::new(config.clone());
    let rich_id = rich_book
        .submit_order(NewOrderRequest::limit(
            config.market_id.clone(),
            Side::Sell,
            Quantity::new(dec!(2.000)),
            Price::new(dec!(101.00)),
        ))
        .order_id
        .unwrap();
    let rich = rich_book.cancel_order(rich_id);

    let mut minimal_book = OrderBook::new(config.clone());
    let minimal_id = minimal_book
        .submit_order_minimal(NewOrderRequest::limit(
            config.market_id.clone(),
            Side::Sell,
            Quantity::new(dec!(2.000)),
            Price::new(dec!(101.00)),
        ))
        .order_id
        .unwrap();
    let minimal = minimal_book.cancel_order_minimal(minimal_id);

    assert_eq!(minimal.cancelled, rich.cancelled);
    assert_eq!(minimal.order_id, rich.order_id);
    assert_eq!(minimal.cancelled_qty, rich.cancelled_qty);
    assert_eq!(minimal.error, rich.error);
    assert_eq!(minimal_book.top_of_book(), rich_book.top_of_book());
    assert_eq!(minimal_book.snapshot(5), rich_book.snapshot(5));
}

#[test]
fn minimal_paths_report_typed_errors() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let rejected = book.submit_order_minimal(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.005)),
    ));
    assert!(!rejected.accepted);
    assert!(rejected.error.is_some());

    let cancel = book.cancel_order_minimal(OrderId::new(999));
    assert!(!cancel.cancelled);
    assert!(cancel.error.is_some());
}

#[test]
fn disabling_invariant_walks_preserves_book_state_for_valid_flows() {
    let config = MarketConfig::sol_usdc();

    let mut checked = OrderBook::new(config.clone());
    let mut unchecked = OrderBook::with_invariant_policy(config.clone(), InvariantPolicy::Never);

    let resting_sell = NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(101.00)),
    );
    let crossing_buy = NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    );

    checked.submit_order_minimal(resting_sell.clone());
    checked.submit_order_minimal(crossing_buy.clone());
    unchecked.submit_order_minimal(resting_sell);
    unchecked.submit_order_minimal(crossing_buy);

    assert_eq!(checked.top_of_book(), unchecked.top_of_book());
    assert_eq!(checked.snapshot(5), unchecked.snapshot(5));
    assert_eq!(unchecked.invariant_policy(), InvariantPolicy::Never);
}
