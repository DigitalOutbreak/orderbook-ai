use crate::types::{MarketId, OrderId, Price, Quantity, SequenceNumber};

/// Trading side for an order or trade aggressor.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    /// Returns the opposite side of the market.
    pub fn opposite(self) -> Self {
        match self {
            Self::Buy => Self::Sell,
            Self::Sell => Self::Buy,
        }
    }
}

/// Supported order categories for the MVP engine.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderType {
    Limit,
    Market,
}

/// External order submission request accepted by the public API.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOrderRequest {
    pub market_id: MarketId,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
}

impl NewOrderRequest {
    /// Builds a new limit-order request.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{NewOrderRequest, Price, Quantity, Side};
    ///
    /// let request = NewOrderRequest::limit(
    ///     "SOL/USDC".into(),
    ///     Side::Buy,
    ///     Quantity::new(dec!(1.000)),
    ///     Price::new(dec!(100.00)),
    /// );
    ///
    /// assert_eq!(request.price, Some(Price::new(dec!(100.00))));
    /// ```
    pub fn limit(market_id: MarketId, side: Side, quantity: Quantity, price: Price) -> Self {
        Self {
            market_id,
            side,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
        }
    }

    /// Builds a new market-order request.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{NewOrderRequest, Quantity, Side};
    ///
    /// let request = NewOrderRequest::market(
    ///     "SOL/USDC".into(),
    ///     Side::Sell,
    ///     Quantity::new(dec!(2.500)),
    /// );
    ///
    /// assert!(request.price.is_none());
    /// ```
    pub fn market(market_id: MarketId, side: Side, quantity: Quantity) -> Self {
        Self {
            market_id,
            side,
            order_type: OrderType::Market,
            quantity,
            price: None,
        }
    }
}

/// Internal engine-owned representation of an accepted order.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub order_id: OrderId,
    pub market_id: MarketId,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Option<Price>,
    pub original_qty: Quantity,
    pub remaining_qty: Quantity,
    pub sequence: SequenceNumber,
}

impl Order {
    /// Converts a validated external request into stored engine state.
    pub fn from_request(
        order_id: OrderId,
        sequence: SequenceNumber,
        request: NewOrderRequest,
    ) -> Self {
        Self {
            order_id,
            market_id: request.market_id,
            side: request.side,
            order_type: request.order_type,
            price: request.price,
            original_qty: request.quantity,
            remaining_qty: request.quantity,
            sequence,
        }
    }

    /// Returns `true` if the order has no remaining quantity.
    pub fn is_filled(&self) -> bool {
        self.remaining_qty.is_zero()
    }

    /// Returns the limit price for limit orders.
    pub fn limit_price(&self) -> Option<Price> {
        self.price
    }
}

/// Immutable trade record emitted for each execution.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub market_id: MarketId,
    pub price: Price,
    pub quantity: Quantity,
    pub maker_order_id: OrderId,
    pub taker_order_id: OrderId,
    pub maker_sequence: SequenceNumber,
    pub taker_sequence: SequenceNumber,
    pub taker_side: Side,
}
