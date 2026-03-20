use crate::errors::EngineError;
use crate::order::{OrderType, Side, Trade};
use crate::types::{OrderId, Price, Quantity, SequenceNumber};

/// Aggregated read-only view of one price level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookLevelView {
    pub price: Price,
    pub total_quantity: Quantity,
    pub order_count: usize,
}

/// Depth-limited read-only snapshot of the bid and ask books.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BookSnapshot {
    pub bids: Vec<BookLevelView>,
    pub asks: Vec<BookLevelView>,
}

/// Compact best-bid and best-ask view for the current book state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TopOfBook {
    pub best_bid: Option<BookLevelView>,
    pub best_ask: Option<BookLevelView>,
}

/// Structured event emitted during submission or cancellation processing.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookEvent {
    OrderAccepted {
        order_id: OrderId,
        sequence: SequenceNumber,
        side: Side,
        order_type: OrderType,
        quantity: Quantity,
        price: Option<Price>,
    },
    OrderRejected {
        error: EngineError,
    },
    OrderRested {
        order_id: OrderId,
        side: Side,
        price: Price,
        remaining_qty: Quantity,
    },
    TradeExecuted {
        trade: Trade,
    },
    OrderCancelled {
        order_id: OrderId,
        side: Side,
        price: Price,
        cancelled_qty: Quantity,
    },
    MarketOrderUnfilled {
        order_id: OrderId,
        unfilled_qty: Quantity,
    },
    TopOfBookChanged {
        top_of_book: TopOfBook,
    },
}

/// Structured result returned from order submission.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionResult {
    pub accepted: bool,
    pub order_id: Option<OrderId>,
    pub fully_filled: bool,
    pub remaining_qty: Quantity,
    pub events: Vec<BookEvent>,
    pub top_of_book: TopOfBook,
    pub error: Option<EngineError>,
}

impl SubmissionResult {
    /// Creates a rejected submission result with an `OrderRejected` event.
    pub fn rejected(error: EngineError, top_of_book: TopOfBook) -> Self {
        Self {
            accepted: false,
            order_id: None,
            fully_filled: false,
            remaining_qty: Quantity::zero(),
            events: vec![BookEvent::OrderRejected {
                error: error.clone(),
            }],
            top_of_book,
            error: Some(error),
        }
    }
}

/// Lean submission summary for engine-focused benchmarking and analysis.
///
/// This intentionally omits event vectors and top-of-book snapshots so callers
/// can observe engine mutation cost with less result-construction overhead.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionSummary {
    pub accepted: bool,
    pub order_id: Option<OrderId>,
    pub fully_filled: bool,
    pub remaining_qty: Quantity,
    pub error: Option<EngineError>,
}

/// Structured result returned from order cancellation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelResult {
    pub cancelled: bool,
    pub order_id: OrderId,
    pub cancelled_qty: Quantity,
    pub events: Vec<BookEvent>,
    pub top_of_book: TopOfBook,
    pub error: Option<EngineError>,
}

/// Lean cancellation summary for engine-focused benchmarking and analysis.
///
/// This intentionally omits event vectors and top-of-book snapshots so callers
/// can study cancel-path cost separately from observability cost.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSummary {
    pub cancelled: bool,
    pub order_id: OrderId,
    pub cancelled_qty: Quantity,
    pub error: Option<EngineError>,
}
