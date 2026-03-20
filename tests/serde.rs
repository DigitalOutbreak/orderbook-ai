#![cfg(feature = "serde")]

use rust_decimal_macros::dec;
use solbook_core::{BookSnapshot, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn snapshot_and_submission_result_serialize_for_adapter_layers() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));
    let snapshot = book.snapshot(5);

    let result_json = serde_json::to_string(&result).unwrap();
    let snapshot_json = serde_json::to_string(&snapshot).unwrap();
    let decoded_snapshot: BookSnapshot = serde_json::from_str(&snapshot_json).unwrap();

    assert!(result_json.contains("accepted"));
    assert!(snapshot_json.contains("bids"));
    assert_eq!(decoded_snapshot, snapshot);
}
