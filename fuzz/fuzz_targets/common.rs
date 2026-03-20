use solbook_core::{BookSnapshot, NewOrderRequest, OrderBook, Price, Quantity, Side, TopOfBook};
use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Summary {
    Submit {
        accepted: bool,
        fully_filled: bool,
        remaining_qty: Quantity,
        event_count: usize,
        top_of_book: TopOfBook,
        error_is_none: bool,
    },
    Cancel {
        cancelled: bool,
        cancelled_qty: Quantity,
        event_count: usize,
        top_of_book: TopOfBook,
        error_is_none: bool,
    },
}

fn decode_side(byte: u8) -> Side {
    if byte.is_multiple_of(2) {
        Side::Buy
    } else {
        Side::Sell
    }
}

fn decode_quantity(byte: u8) -> Quantity {
    let half_steps = 1_u64 + u64::from(byte % 10);
    Quantity::new(rust_decimal::Decimal::from(half_steps) / rust_decimal::Decimal::from(2_u64))
}

fn decode_price(byte: u8, side: Side) -> Price {
    let cents = match side {
        Side::Buy => 9_950_u64 + u64::from(byte % 25),
        Side::Sell => 10_000_u64 + u64::from(byte % 25),
    };
    Price::new(rust_decimal::Decimal::from(cents) / rust_decimal::Decimal::from(100_u64))
}

pub fn assert_snapshot_invariants(snapshot: &BookSnapshot) {
    for pair in snapshot.bids.windows(2) {
        assert!(pair[0].price > pair[1].price);
    }
    for pair in snapshot.asks.windows(2) {
        assert!(pair[0].price < pair[1].price);
    }
    for level in &snapshot.bids {
        assert!(level.total_quantity.is_positive());
        assert!(level.order_count > 0);
    }
    for level in &snapshot.asks {
        assert!(level.total_quantity.is_positive());
        assert!(level.order_count > 0);
    }
    if let (Some(best_bid), Some(best_ask)) = (snapshot.bids.first(), snapshot.asks.first()) {
        assert!(best_bid.price < best_ask.price);
    }
}

pub fn assert_top_matches_snapshot(top: &TopOfBook, snapshot: &BookSnapshot) {
    assert_eq!(top.best_bid.as_ref(), snapshot.bids.first());
    assert_eq!(top.best_ask.as_ref(), snapshot.asks.first());
}

pub fn execute_bytes(data: &[u8]) -> (Vec<Summary>, BookSnapshot) {
    let config = solbook_core::MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());
    let mut accepted_order_ids = Vec::new();
    let mut summaries = Vec::new();

    for chunk in data.chunks(4).take(256) {
        if chunk.is_empty() {
            continue;
        }

        let opcode = chunk[0] % 3;
        match opcode {
            0 | 1 => {
                let side = decode_side(*chunk.get(1).unwrap_or(&0));
                let quantity = decode_quantity(*chunk.get(2).unwrap_or(&1));
                let request = if opcode == 0 {
                    NewOrderRequest::market(config.market_id.clone(), side, quantity)
                } else {
                    let price = decode_price(*chunk.get(3).unwrap_or(&0), side);
                    NewOrderRequest::limit(config.market_id.clone(), side, quantity, price)
                };

                let result = book.submit_order(request);
                if let Some(order_id) = result.order_id {
                    accepted_order_ids.push(order_id);
                }
                summaries.push(Summary::Submit {
                    accepted: result.accepted,
                    fully_filled: result.fully_filled,
                    remaining_qty: result.remaining_qty,
                    event_count: result.events.len(),
                    top_of_book: result.top_of_book.clone(),
                    error_is_none: result.error.is_none(),
                });
            }
            _ => {
                if accepted_order_ids.is_empty() {
                    continue;
                }
                let selector = usize::from(*chunk.get(1).unwrap_or(&0));
                let index = accepted_order_ids.len()
                    - 1
                    - min(
                        selector % accepted_order_ids.len(),
                        accepted_order_ids.len() - 1,
                    );
                let order_id = accepted_order_ids[index];
                let result = book.cancel_order(order_id);
                summaries.push(Summary::Cancel {
                    cancelled: result.cancelled,
                    cancelled_qty: result.cancelled_qty,
                    event_count: result.events.len(),
                    top_of_book: result.top_of_book.clone(),
                    error_is_none: result.error.is_none(),
                });
            }
        }

        let snapshot = book.snapshot(10);
        assert_snapshot_invariants(&snapshot);
        assert_top_matches_snapshot(&book.top_of_book(), &snapshot);
    }

    let snapshot = book.snapshot(10);
    assert_snapshot_invariants(&snapshot);
    assert_top_matches_snapshot(&book.top_of_book(), &snapshot);
    (summaries, snapshot)
}
