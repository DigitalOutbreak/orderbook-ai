use rust_decimal::Decimal;
use std::fmt;

/// Logical market identifier such as `SOL/USDC`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarketId(String);

impl MarketId {
    /// Creates a new market identifier from owned or borrowed string data.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the underlying market identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MarketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MarketId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for MarketId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Monotonic identifier assigned to each accepted order.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OrderId(u64);

impl OrderId {
    /// Creates a new order identifier from a raw integer.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Deterministic FIFO sequence assigned when an order is accepted.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    /// Creates a new sequence number from a raw integer.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Exact decimal price wrapper used for all order and trade prices.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Price(Decimal);

impl Price {
    /// Creates a new price from an exact decimal value.
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    /// Returns the underlying decimal value.
    pub fn value(self) -> Decimal {
        self.0
    }

    /// Returns the scale of the decimal representation.
    pub fn scale(self) -> u32 {
        self.0.scale()
    }

    /// Returns `true` when the price is strictly greater than zero.
    pub fn is_positive(self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Self::new(value)
    }
}

/// Exact decimal quantity wrapper used for order sizes and trade sizes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Quantity(Decimal);

impl Quantity {
    /// Creates a new quantity from an exact decimal value.
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    /// Returns the zero quantity.
    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    /// Returns the underlying decimal value.
    pub fn value(self) -> Decimal {
        self.0
    }

    /// Returns the scale of the decimal representation.
    pub fn scale(self) -> u32 {
        self.0.scale()
    }

    /// Returns `true` when the quantity is equal to zero.
    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Returns `true` when the quantity is strictly greater than zero.
    pub fn is_positive(self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Decimal> for Quantity {
    fn from(value: Decimal) -> Self {
        Self::new(value)
    }
}
