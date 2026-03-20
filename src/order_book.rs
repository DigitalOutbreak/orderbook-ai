use crate::errors::EngineError;
use crate::events::{BookLevelView, BookSnapshot, TopOfBook};
use crate::market_config::MarketConfig;
use crate::order::{Order, Side};
use crate::price_level::PriceLevel;
use crate::types::{OrderId, Price, SequenceNumber};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OrderLocator {
    pub side: Side,
    pub price: Price,
    pub slot: usize,
}

/// Controls whether the engine walks full-book invariants after mutations.
///
/// `Local` is the default and keeps post-mutation checks focused on touched
/// levels and index entries. `Never` exists for engine study and benchmarking
/// so mutation cost can be observed separately from invariant verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantPolicy {
    Local,
    Full,
    Never,
}

#[derive(Debug, Default)]
pub(crate) struct MutationAudit {
    touched_levels: Vec<(Side, Price)>,
    removed_orders: Vec<OrderId>,
    upserted_orders: Vec<(OrderId, OrderLocator)>,
}

impl MutationAudit {
    pub(crate) fn touch_level(&mut self, side: Side, price: Price) {
        self.touched_levels.push((side, price));
    }

    pub(crate) fn record_removed_order(&mut self, order_id: OrderId) {
        self.removed_orders.push(order_id);
    }

    pub(crate) fn record_upserted_order(&mut self, order_id: OrderId, locator: OrderLocator) {
        self.upserted_orders.push((order_id, locator));
    }
}

/// Deterministic in-memory order book for a single configured market.
///
/// Performance shape:
///
/// - `BTreeMap` keeps price levels sorted so best bid / best ask lookup stays cheap
///   and deterministic.
/// - `PriceLevel` owns FIFO order queues at each price.
/// - `order_locations` adds direct cancel lookup, trading more memory for faster
///   cancellation than a full book scan.
///
/// This is intentionally a readable engine structure, not an ultra-low-latency
/// slab-based design.
#[derive(Debug, Clone)]
pub struct OrderBook {
    pub(crate) config: MarketConfig,
    pub(crate) bids: BTreeMap<Price, PriceLevel>,
    pub(crate) asks: BTreeMap<Price, PriceLevel>,
    pub(crate) order_locations: HashMap<OrderId, OrderLocator>,
    pub(crate) next_order_id: u64,
    pub(crate) next_sequence_number: u64,
    pub(crate) invariant_policy: InvariantPolicy,
}

impl OrderBook {
    /// Creates a new empty order book for one market configuration.
    ///
    /// ```rust
    /// use solbook_core::{MarketConfig, OrderBook};
    ///
    /// let book = OrderBook::new(MarketConfig::sol_usdc());
    /// assert!(book.best_bid().is_none());
    /// assert!(book.best_ask().is_none());
    /// ```
    pub fn new(config: MarketConfig) -> Self {
        Self::with_invariant_policy(config, InvariantPolicy::Local)
    }

    /// Creates a new empty order book with an explicit invariant-check policy.
    pub fn with_invariant_policy(config: MarketConfig, invariant_policy: InvariantPolicy) -> Self {
        Self {
            config,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_locations: HashMap::new(),
            next_order_id: 1,
            next_sequence_number: 1,
            invariant_policy,
        }
    }

    /// Returns the configured market metadata and rule set.
    pub fn market_config(&self) -> &MarketConfig {
        &self.config
    }

    /// Returns the current invariant-check policy.
    pub fn invariant_policy(&self) -> InvariantPolicy {
        self.invariant_policy
    }

    /// Sets the invariant-check policy for subsequent mutations.
    pub fn set_invariant_policy(&mut self, invariant_policy: InvariantPolicy) {
        self.invariant_policy = invariant_policy;
    }

    /// Returns the current best bid, if one exists.
    pub fn best_bid(&self) -> Option<BookLevelView> {
        self.bids.last_key_value().map(|(_, level)| level.view())
    }

    /// Returns the current best ask, if one exists.
    pub fn best_ask(&self) -> Option<BookLevelView> {
        self.asks.first_key_value().map(|(_, level)| level.view())
    }

    /// Returns the current top-of-book view.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// let mut book = OrderBook::new(config.clone());
    ///
    /// book.submit_order(NewOrderRequest::limit(
    ///     config.market_id.clone(),
    ///     Side::Buy,
    ///     Quantity::new(dec!(1.000)),
    ///     Price::new(dec!(99.00)),
    /// ));
    ///
    /// let top = book.top_of_book();
    /// assert_eq!(top.best_bid.unwrap().price, Price::new(dec!(99.00)));
    /// assert!(top.best_ask.is_none());
    /// ```
    pub fn top_of_book(&self) -> TopOfBook {
        TopOfBook {
            best_bid: self.best_bid(),
            best_ask: self.best_ask(),
        }
    }

    /// Returns an aggregated snapshot of both sides of the book up to `depth` levels.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// let mut book = OrderBook::new(config.clone());
    ///
    /// book.submit_order(NewOrderRequest::limit(
    ///     config.market_id.clone(),
    ///     Side::Sell,
    ///     Quantity::new(dec!(2.000)),
    ///     Price::new(dec!(101.00)),
    /// ));
    ///
    /// let snapshot = book.snapshot(5);
    /// assert_eq!(snapshot.asks.len(), 1);
    /// assert_eq!(snapshot.asks[0].price, Price::new(dec!(101.00)));
    /// ```
    pub fn snapshot(&self, depth: usize) -> BookSnapshot {
        let bids = self
            .bids
            .iter()
            .rev()
            .take(depth)
            .map(|(_, level)| level.view())
            .collect();

        let asks = self
            .asks
            .iter()
            .take(depth)
            .map(|(_, level)| level.view())
            .collect();

        BookSnapshot { bids, asks }
    }

    pub(crate) fn best_bid_price(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(price, _)| *price)
    }

    pub(crate) fn best_ask_price(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(price, _)| *price)
    }

    pub(crate) fn next_order_id(&mut self) -> OrderId {
        let id = OrderId::new(self.next_order_id);
        self.next_order_id += 1;
        id
    }

    pub(crate) fn next_sequence_number(&mut self) -> SequenceNumber {
        let sequence = SequenceNumber::new(self.next_sequence_number);
        self.next_sequence_number += 1;
        sequence
    }

    pub(crate) fn add_resting_order(&mut self, order: Order) -> OrderLocator {
        let price = order.price.expect("resting orders must have a price");
        let order_id = order.order_id;
        let side = order.side;
        let levels = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        let level = levels
            .entry(price)
            .or_insert_with(|| PriceLevel::new(price));
        let slot = level.enqueue(order);
        let locator = OrderLocator { side, price, slot };
        self.order_locations.insert(order_id, locator);
        locator
    }

    pub(crate) fn remove_resting_order(
        &mut self,
        order_id: OrderId,
    ) -> Option<(Order, OrderLocator)> {
        let locator = self.order_locations.remove(&order_id)?;
        let levels = match locator.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        let level = levels.get_mut(&locator.price)?;
        let removed = level.remove(order_id, locator.slot);
        if level.is_empty() {
            levels.remove(&locator.price);
        }
        removed.map(|order| (order, locator))
    }

    pub(crate) fn pop_best_level(&mut self, side: Side) -> Option<PriceLevel> {
        match side {
            Side::Buy => self.bids.pop_last().map(|(_, level)| level),
            Side::Sell => self.asks.pop_first().map(|(_, level)| level),
        }
    }

    pub(crate) fn put_level_back(&mut self, side: Side, level: PriceLevel) {
        if level.is_empty() {
            return;
        }
        let levels = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        levels.insert(level.price(), level);
    }

    pub(crate) fn remove_order_location(&mut self, order_id: OrderId) {
        self.order_locations.remove(&order_id);
    }

    pub(crate) fn upsert_order_location(
        &mut self,
        order_id: OrderId,
        side: Side,
        price: Price,
        slot: usize,
    ) {
        self.order_locations
            .insert(order_id, OrderLocator { side, price, slot });
    }

    pub(crate) fn maybe_assert_invariants(&self, audit: &MutationAudit) -> Result<(), EngineError> {
        match self.invariant_policy {
            InvariantPolicy::Local => self.assert_local_invariants(audit),
            InvariantPolicy::Full => self.assert_invariants(),
            InvariantPolicy::Never => Ok(()),
        }
    }

    pub(crate) fn assert_invariants(&self) -> Result<(), EngineError> {
        let mut total_orders = 0_usize;
        for level in self.bids.values() {
            total_orders += self.assert_level_invariants(level, Side::Buy)?;
        }

        for level in self.asks.values() {
            total_orders += self.assert_level_invariants(level, Side::Sell)?;
        }

        if total_orders != self.order_locations.len() {
            return Err(EngineError::InvariantViolation(
                "order location index does not match resting order count".to_string(),
            ));
        }

        Ok(())
    }

    fn assert_local_invariants(&self, audit: &MutationAudit) -> Result<(), EngineError> {
        for (side, price) in &audit.touched_levels {
            let levels = match side {
                Side::Buy => &self.bids,
                Side::Sell => &self.asks,
            };
            if let Some(level) = levels.get(price)
                && level.is_empty()
            {
                return Err(EngineError::InvariantViolation(
                    "book contains an empty touched price level".to_string(),
                ));
            }
        }

        for order_id in &audit.removed_orders {
            if self.order_locations.contains_key(order_id) {
                return Err(EngineError::InvariantViolation(
                    "removed order still exists in the location index".to_string(),
                ));
            }
        }

        for (order_id, expected_locator) in &audit.upserted_orders {
            let locator = self.order_locations.get(order_id).ok_or_else(|| {
                EngineError::InvariantViolation(
                    "upserted order is missing from the location index".to_string(),
                )
            })?;
            if locator != expected_locator {
                return Err(EngineError::InvariantViolation(
                    "upserted order location does not match the expected slot".to_string(),
                ));
            }

            let levels = match locator.side {
                Side::Buy => &self.bids,
                Side::Sell => &self.asks,
            };
            let level = levels.get(&locator.price).ok_or_else(|| {
                EngineError::InvariantViolation(
                    "upserted order level is missing from the book".to_string(),
                )
            })?;
            let order = level.order_at(locator.slot).ok_or_else(|| {
                EngineError::InvariantViolation(
                    "upserted order slot is missing from the touched level".to_string(),
                )
            })?;
            if order.order_id != *order_id
                || order.side != locator.side
                || order.price != Some(locator.price)
                || order.market_id != self.config.market_id
                || order.remaining_qty.is_zero()
            {
                return Err(EngineError::InvariantViolation(
                    "upserted order contents do not match the location index".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn assert_level_invariants(
        &self,
        level: &PriceLevel,
        expected_side: Side,
    ) -> Result<usize, EngineError> {
        if level.is_empty() {
            return Err(EngineError::InvariantViolation(
                "book contains an empty price level".to_string(),
            ));
        }

        let mut previous_sequence = None;
        let mut count = 0;
        for (slot, order) in level.iter_entries() {
            if order.side != expected_side {
                return Err(EngineError::InvariantViolation(
                    "price level contains an order on the wrong side".to_string(),
                ));
            }
            if order.market_id != self.config.market_id {
                return Err(EngineError::InvariantViolation(
                    "resting order belongs to the wrong market".to_string(),
                ));
            }
            if order.remaining_qty.is_zero() {
                return Err(EngineError::InvariantViolation(
                    "resting order has zero remaining quantity".to_string(),
                ));
            }
            let price = order.price.expect("resting order must have a price");
            if price != level.price() {
                return Err(EngineError::InvariantViolation(
                    "order price does not match its level".to_string(),
                ));
            }
            self.config
                .validate_price(price)
                .map_err(EngineError::from)?;
            self.config
                .validate_quantity(order.remaining_qty)
                .map_err(EngineError::from)?;
            if let Some(previous) = previous_sequence
                && order.sequence <= previous
            {
                return Err(EngineError::InvariantViolation(
                    "level sequence ordering is not FIFO".to_string(),
                ));
            }

            let locator = self.order_locations.get(&order.order_id).ok_or_else(|| {
                EngineError::InvariantViolation(
                    "resting order is missing from the location index".to_string(),
                )
            })?;
            if locator.side != expected_side || locator.price != price || locator.slot != slot {
                return Err(EngineError::InvariantViolation(
                    "order location index does not match level storage".to_string(),
                ));
            }

            previous_sequence = Some(order.sequence);
            count += 1;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::NewOrderRequest;
    use rust_decimal_macros::dec;

    #[test]
    fn snapshot_orders_bids_desc_and_asks_asc() {
        let config = MarketConfig::sol_usdc();
        let mut book = OrderBook::new(config.clone());

        let orders = [
            NewOrderRequest::limit(
                config.market_id.clone(),
                Side::Buy,
                dec!(1.000).into(),
                dec!(99.00).into(),
            ),
            NewOrderRequest::limit(
                config.market_id.clone(),
                Side::Buy,
                dec!(1.000).into(),
                dec!(100.00).into(),
            ),
            NewOrderRequest::limit(
                config.market_id.clone(),
                Side::Sell,
                dec!(1.000).into(),
                dec!(101.00).into(),
            ),
            NewOrderRequest::limit(
                config.market_id.clone(),
                Side::Sell,
                dec!(1.000).into(),
                dec!(102.00).into(),
            ),
        ];

        for request in orders {
            let result = book.submit_order(request);
            assert!(result.accepted);
        }

        let snapshot = book.snapshot(2);
        assert_eq!(snapshot.bids[0].price, Price::new(dec!(100.00)));
        assert_eq!(snapshot.bids[1].price, Price::new(dec!(99.00)));
        assert_eq!(snapshot.asks[0].price, Price::new(dec!(101.00)));
        assert_eq!(snapshot.asks[1].price, Price::new(dec!(102.00)));
    }
}
