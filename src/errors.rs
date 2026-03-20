use crate::types::{MarketId, OrderId, Price, Quantity};
use thiserror::Error;

/// Validation failures raised before matching or resting state mutation occurs.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    #[error("market mismatch: expected {expected}, got {actual}")]
    MarketMismatch {
        expected: MarketId,
        actual: MarketId,
    },
    #[error("market id must not be empty")]
    EmptyMarketId,
    #[error("base asset must not be empty")]
    EmptyBaseAsset,
    #[error("quote asset must not be empty")]
    EmptyQuoteAsset,
    #[error("base and quote assets must differ")]
    DuplicateAssetSymbols,
    #[error("tick size must be positive")]
    NonPositiveTickSize,
    #[error("lot size must be positive")]
    NonPositiveLotSize,
    #[error("quantity must be positive")]
    NonPositiveQuantity,
    #[error("price must be positive")]
    NonPositivePrice,
    #[error("limit orders require a price")]
    LimitOrderRequiresPrice,
    #[error("market orders must not include a price")]
    MarketOrderWithPrice,
    #[error("price precision exceeded: max {allowed}, got {actual}")]
    PricePrecisionExceeded { allowed: u32, actual: u32 },
    #[error("quantity precision exceeded: max {allowed}, got {actual}")]
    QuantityPrecisionExceeded { allowed: u32, actual: u32 },
    #[error("price {price} does not align with tick size {tick_size}")]
    InvalidTickSize { price: Price, tick_size: Price },
    #[error("quantity {quantity} does not align with lot size {lot_size}")]
    InvalidLotSize {
        quantity: Quantity,
        lot_size: Quantity,
    },
    #[error("tick size scale exceeds price precision")]
    TickSizePrecisionMismatch,
    #[error("lot size scale exceeds quantity precision")]
    LotSizePrecisionMismatch,
}

/// Top-level engine error type for validation, lookup, and invariant failures.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EngineError {
    #[error(transparent)]
    Validation(ValidationError),
    #[error("order {0} was not found")]
    OrderNotFound(OrderId),
    #[error("invariant violated: {0}")]
    InvariantViolation(String),
}

impl From<ValidationError> for EngineError {
    fn from(value: ValidationError) -> Self {
        Self::Validation(value)
    }
}
