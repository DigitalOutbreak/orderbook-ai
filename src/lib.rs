//! Deterministic exchange-core library for a single in-memory spot order book.
//!
//! The crate exposes a compact public API centered on [`OrderBook`], strong domain
//! types, explicit validation, and structured event emission.
//!
//! Enable the optional `serde` feature when you want to move snapshots, events,
//! or execution results across a JSON boundary for a frontend or service adapter.
//!
//! # Example
//!
//! ```rust
//! use rust_decimal_macros::dec;
//! use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};
//!
//! let config = MarketConfig::sol_usdc();
//! let mut book = OrderBook::new(config.clone());
//!
//! let resting = book.submit_order(NewOrderRequest::limit(
//!     config.market_id.clone(),
//!     Side::Sell,
//!     Quantity::new(dec!(1.500)),
//!     Price::new(dec!(101.00)),
//! ));
//!
//! let incoming = book.submit_order(NewOrderRequest::limit(
//!     config.market_id.clone(),
//!     Side::Buy,
//!     Quantity::new(dec!(1.000)),
//!     Price::new(dec!(101.00)),
//! ));
//!
//! assert!(resting.accepted);
//! assert!(incoming.accepted);
//! assert!(incoming.events.iter().any(|event| matches!(event, BookEvent::TradeExecuted { .. })));
//! assert_eq!(book.best_ask().unwrap().total_quantity, Quantity::new(dec!(0.500)));
//! ```
mod errors;
mod events;
mod market_config;
mod matching;
mod order;
mod order_book;
mod price_level;
mod types;
mod validation;

/// Typed engine and validation errors returned by the crate.
pub use crate::errors::{EngineError, ValidationError};
/// Structured event and query view types emitted by the order book.
pub use crate::events::{
    BookEvent, BookLevelView, BookSnapshot, CancelResult, CancelSummary, SubmissionResult,
    SubmissionSummary, TopOfBook,
};
/// Market metadata and rule configuration.
pub use crate::market_config::MarketConfig;
/// Order-domain types for requests, stored orders, trades, and side selection.
pub use crate::order::{NewOrderRequest, Order, OrderType, Side, Trade};
/// Deterministic in-memory order book and matching engine entry point.
pub use crate::order_book::{InvariantPolicy, OrderBook};
/// Strongly typed identifiers and exact financial values.
pub use crate::types::{MarketId, OrderId, Price, Quantity, SequenceNumber};
