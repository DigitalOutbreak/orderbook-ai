---
title: Architecture
---

# Architecture

`solbook-core` is a library-first exchange core. The `OrderBook` owns the market configuration, resting bid and ask levels, deterministic ID generators, and the index used for cancellations.

## Module layout

- `types.rs`: `MarketId`, `OrderId`, `SequenceNumber`, `Price`, `Quantity`
- `order.rs`: `Side`, `OrderType`, `NewOrderRequest`, `Order`, `Trade`
- `market_config.rs`: market metadata and rules
- `validation.rs`: request validation and rule checks
- `price_level.rs`: FIFO resting-order level storage per price
- `order_book.rs`: state ownership, queries, snapshots, invariant checks
- `matching.rs`: submission, matching, and cancellation flow
- `events.rs`: event stream plus `SubmissionResult` and `CancelResult`
- `errors.rs`: typed validation and engine errors

## State model

The book stores bids and asks in `BTreeMap<Price, PriceLevel>`.

- bid priority: highest price first via reverse iteration
- ask priority: lowest price first via forward iteration
- FIFO at each price: slot-indexed linked storage inside `PriceLevel`
- cancellation lookup: `HashMap<OrderId, OrderLocator>`

This keeps the first production version explicit and reviewable without speculative abstractions.

## Matching model

Every accepted order receives:

- a monotonic `OrderId`
- a monotonic `SequenceNumber`

Matching rules:

- limit buy crosses asks while `ask <= buy_price`
- limit sell crosses bids while `bid >= sell_price`
- market orders sweep the opposite book until filled or exhausted
- market orders never rest
- unfilled limit quantity rests on its own side
- trade price is always the resting maker price
- empty levels are removed eagerly

## Event model

Each order submission or cancellation returns structured results containing emitted `BookEvent` values.

Core events:

- `OrderAccepted`
- `OrderRejected`
- `OrderRested`
- `TradeExecuted`
- `OrderCancelled`
- `MarketOrderUnfilled`
- `TopOfBookChanged`

Events are the source of truth. Summary fields like `fully_filled`, `remaining_qty`, and `top_of_book` exist as ergonomic helpers and are derived from the same state transitions.

## Architecture decisions

- Exact arithmetic uses `rust_decimal` instead of floats to enforce tick, lot, and precision rules deterministically.
- Matching is implemented as explicit mutation over concrete structures instead of generic strategy abstractions. That keeps the core logic auditable.
- Invariant checks live close to `OrderBook` state ownership so tests and future adapters can reason about state safety from one place.
- `PriceLevel` uses slot-indexed linked storage because it keeps FIFO explicit while allowing direct removal once `OrderLocator` has already found the right level and slot.
