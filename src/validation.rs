use crate::errors::{EngineError, ValidationError};
use crate::market_config::{MarketConfig, RuleValue};
use crate::order::{NewOrderRequest, OrderType};
use crate::types::{Price, Quantity};
use rust_decimal::Decimal;

pub fn validate_market_config(config: &MarketConfig) -> Result<(), EngineError> {
    if config.market_id.as_str().trim().is_empty() {
        return Err(ValidationError::EmptyMarketId.into());
    }
    if config.base_asset.trim().is_empty() {
        return Err(ValidationError::EmptyBaseAsset.into());
    }
    if config.quote_asset.trim().is_empty() {
        return Err(ValidationError::EmptyQuoteAsset.into());
    }
    if config.base_asset == config.quote_asset {
        return Err(ValidationError::DuplicateAssetSymbols.into());
    }
    if !config.tick_size.is_positive() {
        return Err(ValidationError::NonPositiveTickSize.into());
    }
    if !config.lot_size.is_positive() {
        return Err(ValidationError::NonPositiveLotSize.into());
    }
    if config.tick_size.scale() > config.price_precision {
        return Err(ValidationError::TickSizePrecisionMismatch.into());
    }
    if config.lot_size.scale() > config.quantity_precision {
        return Err(ValidationError::LotSizePrecisionMismatch.into());
    }
    Ok(())
}

pub fn validate_new_order_request(
    config: &MarketConfig,
    request: &NewOrderRequest,
) -> Result<(), EngineError> {
    if request.market_id != config.market_id {
        return Err(ValidationError::MarketMismatch {
            expected: config.market_id.clone(),
            actual: request.market_id.clone(),
        }
        .into());
    }
    if !request.quantity.is_positive() {
        return Err(ValidationError::NonPositiveQuantity.into());
    }
    validate_quantity(config, request.quantity)?;

    match request.order_type {
        OrderType::Limit => {
            let price = request
                .price
                .ok_or(ValidationError::LimitOrderRequiresPrice)?;
            if !price.is_positive() {
                return Err(ValidationError::NonPositivePrice.into());
            }
            validate_price(config, price)?;
        }
        OrderType::Market => {
            if request.price.is_some() {
                return Err(ValidationError::MarketOrderWithPrice.into());
            }
        }
    }

    Ok(())
}

pub(crate) fn validate_market_rules(
    config: &MarketConfig,
    value: RuleValue,
    step: RuleValue,
) -> Result<(), ValidationError> {
    match (value, step) {
        (RuleValue::Price(price), RuleValue::Price(tick_size)) => {
            validate_price_against_rules(config, price, tick_size)
        }
        (RuleValue::Quantity(quantity), RuleValue::Quantity(lot_size)) => {
            validate_quantity_against_rules(config, quantity, lot_size)
        }
        _ => unreachable!("rule value types must match"),
    }
}

pub fn validate_price(config: &MarketConfig, price: Price) -> Result<(), EngineError> {
    validate_price_against_rules(config, price, config.tick_size).map_err(Into::into)
}

pub fn validate_quantity(config: &MarketConfig, quantity: Quantity) -> Result<(), EngineError> {
    validate_quantity_against_rules(config, quantity, config.lot_size).map_err(Into::into)
}

fn validate_price_against_rules(
    config: &MarketConfig,
    price: Price,
    tick_size: Price,
) -> Result<(), ValidationError> {
    if price.scale() > config.price_precision {
        return Err(ValidationError::PricePrecisionExceeded {
            allowed: config.price_precision,
            actual: price.scale(),
        });
    }
    if !aligned(price.value(), tick_size.value()) {
        return Err(ValidationError::InvalidTickSize { price, tick_size });
    }
    Ok(())
}

fn validate_quantity_against_rules(
    config: &MarketConfig,
    quantity: Quantity,
    lot_size: Quantity,
) -> Result<(), ValidationError> {
    if quantity.scale() > config.quantity_precision {
        return Err(ValidationError::QuantityPrecisionExceeded {
            allowed: config.quantity_precision,
            actual: quantity.scale(),
        });
    }
    if !aligned(quantity.value(), lot_size.value()) {
        return Err(ValidationError::InvalidLotSize { quantity, lot_size });
    }
    Ok(())
}

fn aligned(value: Decimal, step: Decimal) -> bool {
    (value % step).is_zero()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_config::MarketConfig;
    use crate::order::{NewOrderRequest, Side};
    use rust_decimal_macros::dec;

    #[test]
    fn rejects_quantity_with_invalid_lot_size() {
        let config = MarketConfig::sol_usdc();
        let request = NewOrderRequest::limit(
            config.market_id.clone(),
            Side::Buy,
            Quantity::new(dec!(1.0005)),
            Price::new(dec!(100.00)),
        );

        let error = validate_new_order_request(&config, &request).unwrap_err();
        assert_eq!(
            error,
            EngineError::Validation(ValidationError::QuantityPrecisionExceeded {
                allowed: 3,
                actual: 4,
            })
        );
    }

    #[test]
    fn rejects_market_order_with_price() {
        let config = MarketConfig::sol_usdc();
        let mut request =
            NewOrderRequest::market(config.market_id.clone(), Side::Buy, Quantity::new(dec!(1)));
        request.price = Some(Price::new(dec!(100)));

        let error = validate_new_order_request(&config, &request).unwrap_err();
        assert_eq!(
            error,
            EngineError::Validation(ValidationError::MarketOrderWithPrice)
        );
    }
}
