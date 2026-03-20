---
title: 05 Guided Exercises
---

# 05 Guided Exercises

This chapter turns the repository into a study project instead of just a codebase.

The goal is not to race through the exercises.
The goal is to make the engine feel predictable.

## Before you start

Keep [docs/glossary.md](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md) open in another tab.

If a word feels vague, stop and look it up.

## Exercise 1: Trace a resting order

Goal:

- understand how a valid limit order becomes resting book state

Do this:

1. Read [tests/matching.rs](/Users/joeyalvarado/Developer/solbook-core/tests/matching.rs).
2. Read [src/validation.rs](/Users/joeyalvarado/Developer/solbook-core/src/validation.rs).
3. Read [src/matching.rs](/Users/joeyalvarado/Developer/solbook-core/src/matching.rs).
4. Run `cargo test matching -- --nocapture`.

Questions to answer:

- When is the order accepted?
- When is the `OrderId` assigned?
- Why does the order rest instead of trade?
- Which structure stores it afterward?

## Exercise 2: Trace a crossing trade

Goal:

- understand how maker and taker behavior differ

Do this:

1. Read [tests/partial_fills.rs](/Users/joeyalvarado/Developer/solbook-core/tests/partial_fills.rs).
2. Read [src/matching.rs](/Users/joeyalvarado/Developer/solbook-core/src/matching.rs).
3. Read [src/events.rs](/Users/joeyalvarado/Developer/solbook-core/src/events.rs).

Questions to answer:

- Why is the trade priced at the maker price?
- What happens to unfilled taker quantity?
- Which event tells you top-of-book changed?

## Exercise 3: Understand cancel

Goal:

- understand why direct cancellation lookup exists

Do this:

1. Read [tests/cancels.rs](/Users/joeyalvarado/Developer/solbook-core/tests/cancels.rs).
2. Read [src/order_book.rs](/Users/joeyalvarado/Developer/solbook-core/src/order_book.rs).
3. Read [src/price_level.rs](/Users/joeyalvarado/Developer/solbook-core/src/price_level.rs).

Questions to answer:

- Why is there a locator map?
- What role does the stored slot play?
- What book state has to change on cancel?
- What happens if the order does not exist?

## Exercise 4: Read performance notes slowly

Goal:

- connect the data structures to their costs

Do this:

1. Read [docs/engine-performance.md](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md).
2. Run `cargo bench --bench throughput submit_resting_limit_orders_minimal_no_invariants -- --sample-size 10`.
3. Run `cargo bench --bench throughput cancel_deep_single_price_level_minimal_no_invariants -- --sample-size 10`.

Questions to answer:

- Which path is the actual matching hot path?
- Which costs come from matching logic and which come from result assembly?
- Why did full invariant walks matter so much?
- Why was one attempted optimization rejected?

## Exercise 5: Map core concepts onto the learning terminal

Goal:

- see how engine outputs become a study interface

Do this:

1. Open the learning terminal in [`web/`](/Users/joeyalvarado/Developer/solbook-core/web).
2. Submit a few mock orders through the order form.
3. Inspect the orderbook, recent trades, and event log after each action.

Questions to answer:

- Which parts of the UI seem to come directly from core concepts like `BookSnapshot`, `TopOfBook`, or `BookEvent`?
- Which parts are clearly presentation choices for learning?
- What becomes easier to understand visually than it was in raw code?

## Exercise 6: Explain the system back to yourself

Write a one-page note answering:

- what the core owns
- what the learning UI owns
- what a future frontend should own
- what should never move into the core
- how bids, asks, and FIFO are represented
- how cancel finds a resting order

If you can explain those boundaries and state transitions cleanly, you know the project well enough to extend it safely.
