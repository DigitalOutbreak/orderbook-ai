use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn partial_limit_fill_rests_remaining_quantity() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.500)),
        Price::new(dec!(100.00)),
    ));

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(100.00)),
    ));

    assert!(!result.fully_filled);
    assert_eq!(result.remaining_qty, Quantity::new(dec!(0.500)));
    assert_eq!(book.best_bid().unwrap().price, Price::new(dec!(100.00)));
    assert_eq!(
        book.best_bid().unwrap().total_quantity,
        Quantity::new(dec!(0.500))
    );
    assert!(result.events.iter().any(|event| matches!(
        event,
        BookEvent::OrderRested { remaining_qty, .. } if *remaining_qty == Quantity::new(dec!(0.500))
    )));
}
