use crate::errors::EngineError;
use crate::events::{
    BookEvent, CancelResult, CancelSummary, SubmissionResult, SubmissionSummary, TopOfBook,
};
use crate::order::{NewOrderRequest, Order, OrderType, Side, Trade};
use crate::order_book::OrderBook;
use crate::price_level::PriceLevel;
use crate::types::{OrderId, Price, Quantity};
use crate::validation::validate_new_order_request;

struct SubmissionCore {
    accepted: bool,
    order_id: Option<OrderId>,
    fully_filled: bool,
    remaining_qty: Quantity,
    events: Vec<BookEvent>,
    top_of_book: Option<TopOfBook>,
    error: Option<EngineError>,
}

struct CancelCore {
    cancelled: bool,
    order_id: OrderId,
    cancelled_qty: Quantity,
    events: Vec<BookEvent>,
    top_of_book: Option<TopOfBook>,
    error: Option<EngineError>,
}

fn submit_order_core(
    book: &mut OrderBook,
    request: NewOrderRequest,
    emit_observability: bool,
) -> SubmissionCore {
    let mut audit = crate::order_book::MutationAudit::default();
    let top_before = emit_observability.then(|| book.top_of_book());

    if let Err(error) = validate_new_order_request(book.market_config(), &request) {
        return SubmissionCore {
            accepted: false,
            order_id: None,
            fully_filled: false,
            remaining_qty: Quantity::zero(),
            events: if emit_observability {
                vec![BookEvent::OrderRejected {
                    error: error.clone(),
                }]
            } else {
                Vec::new()
            },
            top_of_book: top_before,
            error: Some(error),
        };
    }

    let order_id = book.next_order_id();
    let sequence = book.next_sequence_number();
    let mut taker_order = Order::from_request(order_id, sequence, request);
    let mut events = Vec::with_capacity(8);
    if emit_observability {
        events.push(BookEvent::OrderAccepted {
            order_id,
            sequence,
            side: taker_order.side,
            order_type: taker_order.order_type,
            quantity: taker_order.original_qty,
            price: taker_order.price,
        });
    }

    match taker_order.side {
        Side::Buy => match_against_asks(
            book,
            &mut taker_order,
            &mut events,
            emit_observability,
            &mut audit,
        ),
        Side::Sell => match_against_bids(
            book,
            &mut taker_order,
            &mut events,
            emit_observability,
            &mut audit,
        ),
    }

    let remaining_qty = taker_order.remaining_qty;
    let fully_filled = remaining_qty.is_zero();
    if taker_order.order_type == OrderType::Limit && !taker_order.remaining_qty.is_zero() {
        let resting_price = taker_order.price.expect("limit order must have a price");
        let resting_order_id = taker_order.order_id;
        let resting_side = taker_order.side;
        let locator = book.add_resting_order(taker_order);
        audit.touch_level(locator.side, locator.price);
        audit.record_upserted_order(resting_order_id, locator);
        if emit_observability {
            events.push(BookEvent::OrderRested {
                order_id: resting_order_id,
                side: resting_side,
                price: resting_price,
                remaining_qty,
            });
        }
    } else if taker_order.order_type == OrderType::Market && !fully_filled && emit_observability {
        events.push(BookEvent::MarketOrderUnfilled {
            order_id: taker_order.order_id,
            unfilled_qty: remaining_qty,
        });
    }

    let top_after = top_before.as_ref().map(|_| book.top_of_book());
    if let (Some(before), Some(after)) = (&top_before, &top_after)
        && after != before
    {
        events.push(BookEvent::TopOfBookChanged {
            top_of_book: after.clone(),
        });
    }

    if let Err(error) = book.maybe_assert_invariants(&audit) {
        return SubmissionCore {
            accepted: true,
            order_id: Some(order_id),
            fully_filled: false,
            remaining_qty,
            events,
            top_of_book: top_after,
            error: Some(error),
        };
    }

    SubmissionCore {
        accepted: true,
        order_id: Some(order_id),
        fully_filled,
        remaining_qty,
        events,
        top_of_book: top_after,
        error: None,
    }
}

fn cancel_order_core(
    book: &mut OrderBook,
    order_id: OrderId,
    emit_observability: bool,
) -> CancelCore {
    let mut audit = crate::order_book::MutationAudit::default();
    let top_before = emit_observability.then(|| book.top_of_book());
    let Some((cancelled_order, locator)) = book.remove_resting_order(order_id) else {
        return CancelCore {
            cancelled: false,
            order_id,
            cancelled_qty: Quantity::zero(),
            events: Vec::new(),
            top_of_book: top_before,
            error: Some(EngineError::OrderNotFound(order_id)),
        };
    };
    audit.touch_level(locator.side, locator.price);
    audit.record_removed_order(order_id);

    let mut events = Vec::with_capacity(2);
    if emit_observability {
        events.push(BookEvent::OrderCancelled {
            order_id,
            side: cancelled_order.side,
            price: cancelled_order
                .price
                .expect("resting order must have a price"),
            cancelled_qty: cancelled_order.remaining_qty,
        });
    }
    let top_after = top_before.as_ref().map(|_| book.top_of_book());
    if let (Some(before), Some(after)) = (&top_before, &top_after)
        && after != before
    {
        events.push(BookEvent::TopOfBookChanged {
            top_of_book: after.clone(),
        });
    }

    let error = book.maybe_assert_invariants(&audit).err();
    CancelCore {
        cancelled: true,
        order_id,
        cancelled_qty: cancelled_order.remaining_qty,
        events,
        top_of_book: top_after,
        error,
    }
}

fn match_against_asks(
    book: &mut OrderBook,
    taker_order: &mut Order,
    events: &mut Vec<BookEvent>,
    emit_observability: bool,
    audit: &mut crate::order_book::MutationAudit,
) {
    while !taker_order.remaining_qty.is_zero() {
        let Some(best_ask_price) = book.best_ask_price() else {
            break;
        };
        if !crosses_limit(taker_order, best_ask_price) {
            break;
        }

        let Some(level) = book.pop_best_level(Side::Sell) else {
            break;
        };
        audit.touch_level(Side::Sell, best_ask_price);
        let level = fill_from_level(book, taker_order, level, events, emit_observability, audit);
        book.put_level_back(Side::Sell, level);
    }
}

fn match_against_bids(
    book: &mut OrderBook,
    taker_order: &mut Order,
    events: &mut Vec<BookEvent>,
    emit_observability: bool,
    audit: &mut crate::order_book::MutationAudit,
) {
    while !taker_order.remaining_qty.is_zero() {
        let Some(best_bid_price) = book.best_bid_price() else {
            break;
        };
        if !crosses_limit(taker_order, best_bid_price) {
            break;
        }

        let Some(level) = book.pop_best_level(Side::Buy) else {
            break;
        };
        audit.touch_level(Side::Buy, best_bid_price);
        let level = fill_from_level(book, taker_order, level, events, emit_observability, audit);
        book.put_level_back(Side::Buy, level);
    }
}

fn fill_from_level(
    book: &mut OrderBook,
    taker_order: &mut Order,
    mut level: PriceLevel,
    events: &mut Vec<BookEvent>,
    emit_observability: bool,
    audit: &mut crate::order_book::MutationAudit,
) -> PriceLevel {
    // This loop is the matching hot path. It repeatedly consumes the oldest
    // maker at the best price and only allocates for emitted trade events.
    while !taker_order.remaining_qty.is_zero() {
        let Some((mut maker_order, _slot)) = level.pop_front() else {
            break;
        };

        let traded_qty = Quantity::new(
            taker_order
                .remaining_qty
                .value()
                .min(maker_order.remaining_qty.value()),
        );
        taker_order.remaining_qty =
            Quantity::new(taker_order.remaining_qty.value() - traded_qty.value());
        maker_order.remaining_qty =
            Quantity::new(maker_order.remaining_qty.value() - traded_qty.value());

        if emit_observability {
            let trade = Trade {
                market_id: taker_order.market_id.clone(),
                price: maker_order
                    .price
                    .expect("resting maker orders must have a price"),
                quantity: traded_qty,
                maker_order_id: maker_order.order_id,
                taker_order_id: taker_order.order_id,
                maker_sequence: maker_order.sequence,
                taker_sequence: taker_order.sequence,
                taker_side: taker_order.side,
            };
            events.push(BookEvent::TradeExecuted { trade });
        }

        if maker_order.remaining_qty.is_zero() {
            book.remove_order_location(maker_order.order_id);
            audit.record_removed_order(maker_order.order_id);
        } else {
            let maker_order_id = maker_order.order_id;
            let maker_side = maker_order.side;
            let maker_price = maker_order
                .price
                .expect("resting maker orders must have a price");
            let slot = level.push_front(maker_order);
            let locator = crate::order_book::OrderLocator {
                side: maker_side,
                price: maker_price,
                slot,
            };
            book.upsert_order_location(maker_order_id, maker_side, maker_price, slot);
            audit.record_upserted_order(maker_order_id, locator);
            break;
        }
    }

    level
}

fn crosses_limit(order: &Order, resting_price: Price) -> bool {
    match order.order_type {
        OrderType::Market => true,
        OrderType::Limit => match (
            order.side,
            order.price.expect("limit order must have a price"),
        ) {
            (Side::Buy, price) => resting_price <= price,
            (Side::Sell, price) => resting_price >= price,
        },
    }
}

impl OrderBook {
    /// Validates and submits a new order into the book, returning structured events and summaries.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// let mut book = OrderBook::new(config.clone());
    ///
    /// let result = book.submit_order(NewOrderRequest::limit(
    ///     config.market_id.clone(),
    ///     Side::Buy,
    ///     Quantity::new(dec!(1.000)),
    ///     Price::new(dec!(100.00)),
    /// ));
    ///
    /// assert!(result.accepted);
    /// assert!(result.events.iter().any(|event| matches!(event, BookEvent::OrderAccepted { .. })));
    /// ```
    pub fn submit_order(&mut self, request: NewOrderRequest) -> SubmissionResult {
        let core = submit_order_core(self, request, true);
        SubmissionResult {
            accepted: core.accepted,
            order_id: core.order_id,
            fully_filled: core.fully_filled,
            remaining_qty: core.remaining_qty,
            events: core.events,
            top_of_book: core.top_of_book.unwrap_or_default(),
            error: core.error,
        }
    }

    /// Validates and submits a new order while returning a lean summary.
    ///
    /// This path exists to help benchmark or study the matching engine without
    /// paying for full event-vector and top-of-book result construction.
    pub fn submit_order_minimal(&mut self, request: NewOrderRequest) -> SubmissionSummary {
        let core = submit_order_core(self, request, false);
        SubmissionSummary {
            accepted: core.accepted,
            order_id: core.order_id,
            fully_filled: core.fully_filled,
            remaining_qty: core.remaining_qty,
            error: core.error,
        }
    }

    /// Cancels a resting order by ID if it is still present in the book.
    ///
    /// ```rust
    /// use rust_decimal_macros::dec;
    /// use solbook_core::{MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};
    ///
    /// let config = MarketConfig::sol_usdc();
    /// let mut book = OrderBook::new(config.clone());
    ///
    /// let submission = book.submit_order(NewOrderRequest::limit(
    ///     config.market_id.clone(),
    ///     Side::Buy,
    ///     Quantity::new(dec!(1.000)),
    ///     Price::new(dec!(99.00)),
    /// ));
    ///
    /// let result = book.cancel_order(submission.order_id.unwrap());
    /// assert!(result.cancelled);
    /// ```
    pub fn cancel_order(&mut self, order_id: OrderId) -> CancelResult {
        let core = cancel_order_core(self, order_id, true);
        CancelResult {
            cancelled: core.cancelled,
            order_id: core.order_id,
            cancelled_qty: core.cancelled_qty,
            events: core.events,
            top_of_book: core.top_of_book.unwrap_or_default(),
            error: core.error,
        }
    }

    /// Cancels a resting order by ID while returning a lean summary.
    ///
    /// This path exists to help benchmark or study cancel-path cost without
    /// paying for full event-vector and top-of-book result construction.
    pub fn cancel_order_minimal(&mut self, order_id: OrderId) -> CancelSummary {
        let core = cancel_order_core(self, order_id, false);
        CancelSummary {
            cancelled: core.cancelled,
            order_id: core.order_id,
            cancelled_qty: core.cancelled_qty,
            error: core.error,
        }
    }
}
