use proptest::prelude::*;
use proptest::test_runner::TestCaseResult;
use rust_decimal::Decimal;
use solbook_core::{
    BookSnapshot, CancelResult, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side,
    SubmissionResult, TopOfBook,
};

#[derive(Debug, Clone)]
enum Operation {
    SubmitLimit {
        side: Side,
        quantity_millis: u64,
        price_cents: u64,
    },
    SubmitMarket {
        side: Side,
        quantity_millis: u64,
    },
    Cancel {
        index_from_end: usize,
    },
}

fn op_strategy() -> impl Strategy<Value = Operation> {
    prop_oneof![
        (any::<bool>(), 1_u64..=5_u64, 9_950_u64..=10_070_u64).prop_map(
            |(is_buy, qty_steps, price_cents)| Operation::SubmitLimit {
                side: if is_buy { Side::Buy } else { Side::Sell },
                quantity_millis: qty_steps * 500,
                price_cents,
            }
        ),
        (any::<bool>(), 1_u64..=6_u64).prop_map(|(is_buy, qty_steps)| Operation::SubmitMarket {
            side: if is_buy { Side::Buy } else { Side::Sell },
            quantity_millis: qty_steps * 500,
        }),
        (0_usize..32_usize).prop_map(|index_from_end| Operation::Cancel { index_from_end }),
    ]
}

fn qty_from_millis(quantity_millis: u64) -> Quantity {
    Quantity::new(Decimal::from(quantity_millis) / Decimal::from(1_000_u64))
}

fn price_from_cents(price_cents: u64) -> Price {
    Price::new(Decimal::from(price_cents) / Decimal::from(100_u64))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubmissionSummary {
    accepted: bool,
    fully_filled: bool,
    event_count: usize,
    remaining_qty: Quantity,
    error_is_none: bool,
    top_of_book: TopOfBook,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CancelSummary {
    cancelled: bool,
    event_count: usize,
    cancelled_qty: Quantity,
    error_is_none: bool,
    top_of_book: TopOfBook,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OutcomeSummary {
    Submission(SubmissionSummary),
    Cancel(CancelSummary),
}

fn summarize_submission(result: &SubmissionResult) -> SubmissionSummary {
    SubmissionSummary {
        accepted: result.accepted,
        fully_filled: result.fully_filled,
        event_count: result.events.len(),
        remaining_qty: result.remaining_qty,
        error_is_none: result.error.is_none(),
        top_of_book: result.top_of_book.clone(),
    }
}

fn summarize_cancel(result: &CancelResult) -> CancelSummary {
    CancelSummary {
        cancelled: result.cancelled,
        event_count: result.events.len(),
        cancelled_qty: result.cancelled_qty,
        error_is_none: result.error.is_none(),
        top_of_book: result.top_of_book.clone(),
    }
}

fn assert_snapshot_invariants(snapshot: &BookSnapshot) -> TestCaseResult {
    for level in &snapshot.bids {
        prop_assert!(level.total_quantity.is_positive());
        prop_assert!(level.order_count > 0);
    }
    for level in &snapshot.asks {
        prop_assert!(level.total_quantity.is_positive());
        prop_assert!(level.order_count > 0);
    }

    for pair in snapshot.bids.windows(2) {
        prop_assert!(pair[0].price > pair[1].price);
    }
    for pair in snapshot.asks.windows(2) {
        prop_assert!(pair[0].price < pair[1].price);
    }

    if let (Some(best_bid), Some(best_ask)) = (snapshot.bids.first(), snapshot.asks.first()) {
        prop_assert!(best_bid.price < best_ask.price);
    }

    Ok(())
}

fn assert_top_matches_snapshot(summary_top: &TopOfBook, snapshot: &BookSnapshot) -> TestCaseResult {
    prop_assert_eq!(summary_top.best_bid.as_ref(), snapshot.bids.first());
    prop_assert_eq!(summary_top.best_ask.as_ref(), snapshot.asks.first());
    Ok(())
}

fn execute(ops: &[Operation]) -> (Vec<OutcomeSummary>, solbook_core::BookSnapshot) {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());
    let mut accepted_order_ids = Vec::new();
    let mut summaries = Vec::with_capacity(ops.len());

    for op in ops {
        match op {
            Operation::SubmitLimit {
                side,
                quantity_millis,
                price_cents,
            } => {
                let result = book.submit_order(NewOrderRequest::limit(
                    config.market_id.clone(),
                    *side,
                    qty_from_millis(*quantity_millis),
                    price_from_cents(*price_cents),
                ));
                if let Some(order_id) = result.order_id {
                    accepted_order_ids.push(order_id);
                }
                summaries.push(OutcomeSummary::Submission(summarize_submission(&result)));
            }
            Operation::SubmitMarket {
                side,
                quantity_millis,
            } => {
                let result = book.submit_order(NewOrderRequest::market(
                    config.market_id.clone(),
                    *side,
                    qty_from_millis(*quantity_millis),
                ));
                if let Some(order_id) = result.order_id {
                    accepted_order_ids.push(order_id);
                }
                summaries.push(OutcomeSummary::Submission(summarize_submission(&result)));
            }
            Operation::Cancel { index_from_end } => {
                if accepted_order_ids.is_empty() {
                    continue;
                }
                let target = accepted_order_ids
                    [accepted_order_ids.len() - 1 - (*index_from_end % accepted_order_ids.len())];
                let result = book.cancel_order(target);
                summaries.push(OutcomeSummary::Cancel(summarize_cancel(&result)));
            }
        }

        let snapshot = book.snapshot(10);
        assert_snapshot_invariants(&snapshot).expect("generated snapshot must preserve invariants");
        let top = book.top_of_book();
        assert_top_matches_snapshot(&top, &snapshot)
            .expect("top-of-book must match the snapshot front levels");
    }

    (summaries, book.snapshot(10))
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 128,
        max_shrink_iters: 10_000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn mixed_operation_sequences_preserve_replayable_results(ops in prop::collection::vec(op_strategy(), 1..80)) {
        let first = execute(&ops);
        let second = execute(&ops);

        prop_assert_eq!(&first, &second);
        assert_snapshot_invariants(&first.1)?;

        for outcome in &first.0 {
            match outcome {
                OutcomeSummary::Submission(summary) => {
                    prop_assert!(summary.event_count > 0);
                    if summary.accepted {
                        prop_assert!(summary.error_is_none);
                    }
                }
                OutcomeSummary::Cancel(summary) => {
                    if summary.cancelled {
                        prop_assert!(summary.error_is_none);
                    }
                }
            }
        }
    }
}
