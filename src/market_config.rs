use crate::errors::{EngineError, ValidationError};
use crate::types::{MarketId, Price, Quantity};
use crate::validation::{validate_market_config, validate_market_rules};
use rust_decimal::Decimal;

/// Market metadata and validation rules for one order book instance.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketConfig {
    pub market_id: MarketId,
    pub base_asset: String,
    pub quote_asset: String,
    pub tick_size: Price,
    pub lot_size: Quantity,
    pub price_precision: u32,
    pub quantity_precision: u32,
}

impl MarketConfig {
    /// Creates a validated market configuration.
    pub fn new(
        market_id: MarketId,
        base_asset: impl Into<String>,
        quote_asset: impl Into<String>,
        tick_size: Price,
        lot_size: Quantity,
        price_precision: u32,
        quantity_precision: u32,
    ) -> Result<Self, EngineError> {
        let config = Self {
            market_id,
            base_asset: base_asset.into(),
            quote_asset: quote_asset.into(),
            tick_size,
            lot_size,
            price_precision,
            quantity_precision,
        };

        validate_market_config(&config)?;
        Ok(config)
    }

    /// Returns the flagship SOL/USDC market used throughout the MVP examples and tests.
    ///
    /// ```rust
    /// use solbook_core::MarketConfig;
    ///
    /// let config = MarketConfig::sol_usdc();
    ///
    /// assert_eq!(config.market_id.as_str(), "SOL/USDC");
    /// assert_eq!(config.base_asset, "SOL");
    /// assert_eq!(config.quote_asset, "USDC");
    /// ```
    pub fn sol_usdc() -> Self {
        Self::new(
            MarketId::from("SOL/USDC"),
            "SOL",
            "USDC",
            Price::new(Decimal::new(1, 2)),
            Quantity::new(Decimal::new(1, 3)),
            2,
            3,
        )
        .expect("default SOL/USDC config must be valid")
    }

    /// Validates a price against this market's precision and tick rules.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{MarketConfig, Price};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// assert!(config.validate_price(Price::new(dec!(100.00))).is_ok());
    /// ```
    pub fn validate_price(&self, price: Price) -> Result<(), ValidationError> {
        validate_market_rules(self, price.into(), self.tick_size.into())
    }

    /// Validates a quantity against this market's precision and lot rules.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{MarketConfig, Quantity};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// assert!(config.validate_quantity(Quantity::new(dec!(1.000))).is_ok());
    /// ```
    pub fn validate_quantity(&self, quantity: Quantity) -> Result<(), ValidationError> {
        validate_market_rules(self, quantity.into(), self.lot_size.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuleValue {
    Price(Price),
    Quantity(Quantity),
}

impl From<Price> for RuleValue {
    fn from(value: Price) -> Self {
        Self::Price(value)
    }
}

impl From<Quantity> for RuleValue {
    fn from(value: Quantity) -> Self {
        Self::Quantity(value)
    }
}
