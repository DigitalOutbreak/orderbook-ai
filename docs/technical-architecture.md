---
title: Technical Architecture
---

# Technical Architecture

This file is the implementation-facing architecture summary for `solbook-core`.

## System role

The crate is an exchange-core library responsible for:

- validating order requests
- assigning deterministic identifiers
- matching with price-time priority
- maintaining in-memory book state
- emitting structured events
- exposing top-of-book and depth snapshots

The crate is not responsible for networking, persistence, balances, settlement, fees, margin, liquidation, or blockchain integration.

## Core data structures

- `BTreeMap<Price, PriceLevel>` for bids and asks
- slot-indexed linked storage inside each `PriceLevel` for FIFO behavior plus direct level-local removal
- `HashMap<OrderId, OrderLocator>` for cancellation lookup
- exact arithmetic via `rust_decimal::Decimal`

For the performance-oriented explanation of why those structures were chosen
and where they will become bottlenecks, see
[`docs/engine-performance.md`](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md).

## Determinism rules

- accepted orders receive monotonic `OrderId` and `SequenceNumber`
- FIFO depends on sequence assignment, not timestamps
- the engine is single-threaded and purely in-memory
- invalid requests are rejected explicitly and never auto-normalized

## Validation rules

- market ID must match the configured market
- quantity must be positive and lot-aligned
- limit orders require a positive price
- market orders must not provide a price
- prices must be tick-aligned
- price and quantity precision must not exceed market limits
- market configuration itself must be internally consistent

## Matching rules

- limit buys match the lowest ask prices first while `ask <= limit_price`
- limit sells match the highest bid prices first while `bid >= limit_price`
- market orders ignore a limit and sweep until filled or liquidity is exhausted
- trade price is the maker's resting price
- partial fills are supported on both maker and taker sides
- unfilled market quantity expires immediately
- unfilled limit quantity rests on the book

## Invariants

- market orders never rest
- no resting order has zero remaining quantity
- bid side contains only buy orders
- ask side contains only sell orders
- FIFO at a price level is preserved
- empty price levels are removed eagerly
- resting orders always match the configured market
- resting values obey tick, lot, and precision rules

## Public API

- `OrderBook::new(MarketConfig)`
- `OrderBook::submit_order(NewOrderRequest) -> SubmissionResult`
- `OrderBook::cancel_order(OrderId) -> CancelResult`
- `OrderBook::best_bid() -> Option<BookLevelView>`
- `OrderBook::best_ask() -> Option<BookLevelView>`
- `OrderBook::top_of_book() -> TopOfBook`
- `OrderBook::snapshot(depth) -> BookSnapshot`

## Adapter boundary

The intended boundary for future frontend or service adapters is:

- `BookEvent`
- `SubmissionResult`
- `CancelResult`
- `TopOfBook`
- `BookSnapshot`

The crate supports an optional `serde` feature so those types can cross a JSON boundary without forcing serialization dependencies into the default core build.

That boundary is intentionally secondary. The project should be read as a
matching engine with optional frontend or service adapters, not as a web
service with some engine logic inside it.

## Architecture decisions

- The matching flow is kept in `matching.rs` as explicit free functions plus `OrderBook` methods, so state ownership remains in `order_book.rs` and execution flow remains easy to review.
- A `HashMap<OrderId, OrderLocator>` is used for cancellation because it provides direct lookup without introducing heavy abstractions or duplicating full order state.
- Result objects carry both events and summary helpers, so downstream code can stay ergonomic without losing inspectability.
- The benchmark scaffold uses Criterion in `benches/throughput.rs` so performance measurement can evolve without contaminating the core library with benchmark-only code.
- Replay-style regression fixtures are kept in integration tests rather than the library so deterministic scenario validation can grow without broadening the core API.
