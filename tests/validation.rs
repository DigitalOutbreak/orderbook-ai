use rust_decimal_macros::dec;
use solbook_core::{
    EngineError, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side, ValidationError,
};

#[test]
fn rejects_market_mismatch() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let result = book.submit_order(NewOrderRequest::limit(
        "BTC/USDC".into(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.00)),
    ));

    assert!(!result.accepted);
    assert_eq!(
        result.error,
        Some(EngineError::Validation(ValidationError::MarketMismatch {
            expected: config.market_id.clone(),
            actual: "BTC/USDC".into(),
        }))
    );
}

#[test]
fn rejects_invalid_tick_size_alignment() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let result = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(100.005)),
    ));

    assert!(!result.accepted);
    assert_eq!(
        result.error,
        Some(EngineError::Validation(
            ValidationError::PricePrecisionExceeded {
                allowed: 2,
                actual: 3,
            }
        ))
    );
}

#[test]
fn rejects_invalid_lot_alignment_without_precision_violation() {
    let config = MarketConfig::new(
        "SOL/USDC".into(),
        "SOL",
        "USDC",
        Price::new(dec!(0.01)),
        Quantity::new(dec!(0.005)),
        2,
        3,
    )
    .unwrap();
    let mut book = OrderBook::new(config.clone());

    let invalid = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(1.001)),
        Price::new(dec!(100.00)),
    ));

    assert!(!invalid.accepted);
    assert_eq!(
        invalid.error,
        Some(EngineError::Validation(ValidationError::InvalidLotSize {
            quantity: Quantity::new(dec!(1.001)),
            lot_size: Quantity::new(dec!(0.005)),
        }))
    );
}

#[test]
fn rejects_duplicate_market_assets_in_config() {
    let result = MarketConfig::new(
        "SOL/SOL".into(),
        "SOL",
        "SOL",
        Price::new(dec!(0.01)),
        Quantity::new(dec!(0.001)),
        2,
        3,
    );

    assert_eq!(
        result.unwrap_err(),
        EngineError::Validation(ValidationError::DuplicateAssetSymbols)
    );
}
