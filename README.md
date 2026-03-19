# solbook-core

`solbook-core` is a deterministic Rust exchange-core library for a `SOL/USDC` spot market. It provides a single-threaded in-memory order book, exact decimal validation, price-time priority matching, structured event emission, cancellation, and snapshot helpers.

The project should be read engine-first:

- `solbook-core` is the main artifact
- `solbook-cli` is a local terminal study tool around the core
- `solbook-egui` is a richer native visual study tool around the core, now built with `iced`
- `solbook-api` is an optional wrapper for demos and future clients

## Scope

The crate is intentionally library-first and deliberately scoped:

- one flagship market configuration: `SOL/USDC`
- internally market-agnostic engine state
- limit and market orders
- FIFO within each price level
- deterministic sequencing via internal sequence numbers
- top-of-book and depth snapshots
- no networking, persistence, balances, fees, or blockchain integration in core

## Architecture

The core modules follow the responsibilities defined in the handoff docs:

- `types.rs`: strongly typed identifiers and exact financial values
- `order.rs`: external order requests, internal orders, sides, order types, and trades
- `market_config.rs`: market rulebook and flagship `SOL/USDC` config
- `validation.rs`: market config and order validation
- `price_level.rs`: FIFO level storage and level-local removal at a single price
- `order_book.rs`: state ownership, queries, and invariant checks
- `matching.rs`: deterministic matching and cancellation flows
- `events.rs`: event stream and structured result types
- `errors.rs`: typed validation and engine errors

The engine uses `rust_decimal` for exact arithmetic and monotonic `OrderId` plus `SequenceNumber` counters for deterministic replayable behavior.

If your goal is to learn matching engines rather than adapters, start with
[`docs/engine-performance.md`](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md).

## Public API

```rust
use rust_decimal_macros::dec;
use solbook_core::{MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

let config = MarketConfig::sol_usdc();
let mut book = OrderBook::new(config.clone());

let buy = NewOrderRequest::limit(
    config.market_id.clone(),
    Side::Buy,
    Quantity::new(dec!(2.000)),
    Price::new(dec!(100.00)),
);

let result = book.submit_order(buy);
assert!(result.accepted);
assert_eq!(book.best_bid().unwrap().price, Price::new(dec!(100.00)));
```

Primary public methods:

- `submit_order(NewOrderRequest) -> SubmissionResult`
- `submit_order_minimal(NewOrderRequest) -> SubmissionSummary`
- `cancel_order(OrderId) -> CancelResult`
- `cancel_order_minimal(OrderId) -> CancelSummary`
- `with_invariant_policy(MarketConfig, InvariantPolicy) -> OrderBook`
- `best_bid() -> Option<BookLevelView>`
- `best_ask() -> Option<BookLevelView>`
- `top_of_book() -> TopOfBook`
- `snapshot(depth) -> BookSnapshot`

## Optional adapter layer

If a later project needs a visual UI or external clients, the core is already shaped for an adapter layer:

- `BookSnapshot`, `TopOfBook`, `BookEvent`, `SubmissionResult`, and `CancelResult` are stable structured outputs
- the crate exposes deterministic state transitions, which is what a frontend needs for replay and time-travel debugging
- optional `serde` support is available for JSON transport

Enable it with:

- `cargo add solbook-core --features serde`

or in a workspace dependency:

```toml
solbook-core = { path = "../solbook-core", features = ["serde"] }
```

That is the intended boundary for a future HTTP API, WebSocket stream, Tauri app, or other external adapter.

This repository includes one optional adapter crate for that boundary in `solbook-api/`.

## Invariants

The implementation preserves these invariants after every successful mutation:

- market orders never rest on the book
- no resting order has zero remaining quantity
- bid levels contain only buy orders
- ask levels contain only sell orders
- FIFO inside a price level is preserved by sequence number
- empty levels are removed eagerly
- resting orders always belong to the configured market
- resting prices and quantities must conform to market rules

## Docs

Supporting docs live in [`docs/architecture.md`](/Users/joeyalvarado/Developer/solbook-core/docs/architecture.md), [`docs/glossary.md`](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md), [`docs/milestones.md`](/Users/joeyalvarado/Developer/solbook-core/docs/milestones.md), and [`docs/technical-architecture.md`](/Users/joeyalvarado/Developer/solbook-core/docs/technical-architecture.md).

For engine internals and tradeoffs, read [`docs/engine-performance.md`](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md).

If you want to study the repo as a guided project, start with [`docs/learning-path.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning-path.md) and begin at [`docs/learning/00-start-here.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/00-start-here.md). The visual mental models live in [`docs/learning/06-visual-guide.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/06-visual-guide.md), the design-tradeoff explanation lives in [`docs/learning/07-why-this-design.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/07-why-this-design.md), the beginner-friendly performance bridge lives in [`docs/learning/08-performance-bridge.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/08-performance-bridge.md), the terminal study workflow lives in [`docs/learning/09-learn-with-the-tui.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/09-learn-with-the-tui.md), and the richer native visual path lives in [`docs/learning/10-learn-with-egui.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/10-learn-with-egui.md).

## Study GUI

The repository now includes [`solbook-egui/`](/Users/joeyalvarado/Developer/solbook-core/solbook-egui), a native `iced` desktop app for learning the engine with a compact fixed-layout study interface.

Run it with:

- `cargo run -p solbook-egui`

What it gives you:

- a disciplined terminal-style header with engine metrics
- a stacked order-book ladder with asks above, spread in the middle, and bids below
- a fixed right control rail for scenarios, submit, and cancel flows
- seeded demo liquidity on startup
- compact state and recent-event panels under the ladder
- quick actions plus manual order entry without any internal scrolling

This is the recommended visual study interface if the terminal UI feels too clunky.

## Study TUI

The repository now includes [`solbook-cli/`](/Users/joeyalvarado/Developer/solbook-core/solbook-cli), a small `ratatui` terminal app for learning the engine visually without leaving the terminal.

Run it with:

- `cargo run -p solbook-cli`

What it shows:

- asks, spread, and bids
- recent events
- recent accepted order ids
- live simulation stats
- simple order-entry and cancel forms
- local engine execution time for each action
- canned scenarios for resting, crossing, market sweeping, and cancel flow
- seeded live demo profiles for balanced flow, buy pressure, sell pressure, and cancel-heavy flow

This is a study tool, not a benchmark harness. Use it to see how removing ask liquidity changes the visible best ask, how selling removes bid liquidity, and how cancels clean up resting levels.

It now starts with seeded demo liquidity by default so you can experiment immediately without manually building a book first.

If you want the transport layer contract for a future UI, read [`docs/api-contract.md`](/Users/joeyalvarado/Developer/solbook-core/docs/api-contract.md).

## Quality gates

Production readiness for this crate means:

- `cargo fmt`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo package`

The repository also includes GitHub Actions workflows for stable validation and nightly fuzz-target builds in `.github/workflows/`.

The repository currently includes integration tests for matching, FIFO, market orders, partial fills, and cancellations, plus module-level unit tests for validation and price-level behavior.

## Benchmark scaffold

The repo now includes a Criterion benchmark scaffold in [`benches/throughput.rs`](/Users/joeyalvarado/Developer/solbook-core/benches/throughput.rs) for repeatable submission throughput measurements.

There is also an isolated data-structure benchmark in [`benches/price_level_prototypes.rs`](/Users/joeyalvarado/Developer/solbook-core/benches/price_level_prototypes.rs) for comparing candidate price-level storage designs outside the main engine.

Run it with:

- `cargo bench`
- `cargo bench --bench price_level_prototypes`
- `./scripts/profile_bench.sh`

Current benchmark coverage:

- resting-order insertion throughput
- resting-order insertion throughput through the lean summary path
- resting-order insertion throughput through the lean summary path with invariant walks disabled
- crossing-order flow throughput against seeded liquidity
- crossing-order flow throughput through the lean summary path
- crossing-order flow throughput through the lean summary path with invariant walks disabled
- cancellation throughput against a populated book
- cancellation throughput against a populated book through the lean summary path
- cancellation throughput against a populated book through the lean summary path with invariant walks disabled
- cancellation throughput for a deep single price level
- cancellation throughput for a deep single price level through the lean summary path
- cancellation throughput for a deep single price level through the lean summary path with invariant walks disabled
- same-price FIFO sweep throughput
- mixed insert-and-cross churn

The current engine keeps FIFO via a level-local linked slot structure rather
than a raw `VecDeque`, so cancellation can use the existing order-location index
and stored slot to remove a resting order without scanning linearly inside a
deep level.

The default invariant mode is `InvariantPolicy::Local`, which checks only the
levels and index entries touched by a mutation. Use `InvariantPolicy::Full` when
you want a whole-book verification pass after every mutation.

Current local baseline notes live in [`docs/performance.md`](/Users/joeyalvarado/Developer/solbook-core/docs/performance.md).

## Deterministic regression fixtures

The integration suite now includes replay-style deterministic fixtures in [`tests/support/mod.rs`](/Users/joeyalvarado/Developer/solbook-core/tests/support/mod.rs) and [`tests/replay.rs`](/Users/joeyalvarado/Developer/solbook-core/tests/replay.rs). These scenarios assert that repeated runs over the same operation stream produce identical events, summaries, and final snapshots.

There is also a seeded mixed-operation stress test and a property-based replayability test in [`tests/property.rs`](/Users/joeyalvarado/Developer/solbook-core/tests/property.rs) that assert ordering, positive resting quantities, best-price consistency, and repeatable outcomes across generated flows.

## Fuzzing

Real fuzz targets now live under [`fuzz/`](/Users/joeyalvarado/Developer/solbook-core/fuzz).

Setup:

- `cargo install cargo-fuzz`

Examples:

- `cargo fuzz run order_flow`
- `cargo fuzz run replay_consistency`
- `./scripts/run_fuzz.sh order_flow`
- `./scripts/run_fuzz.sh replay_consistency`

The fuzz targets stress mixed order submission and cancellation flows and assert snapshot/top-of-book consistency plus replay determinism.

Seed corpora live in:

- [`fuzz/corpus/order_flow`](/Users/joeyalvarado/Developer/solbook-core/fuzz/corpus/order_flow)
- [`fuzz/corpus/replay_consistency`](/Users/joeyalvarado/Developer/solbook-core/fuzz/corpus/replay_consistency)

## Release hygiene

The repository now includes:

- [`CHANGELOG.md`](/Users/joeyalvarado/Developer/solbook-core/CHANGELOG.md)
- [`LICENSE`](/Users/joeyalvarado/Developer/solbook-core/LICENSE)

Package validation can be checked with:

- `cargo package`

## Adapter crate

`solbook-api` is a thin HTTP adapter around the core. It keeps the matching engine deterministic and single-owner by serializing access behind a `Mutex<OrderBook>` and exposing structured JSON endpoints.

Current endpoints:

- `GET /health`
- `GET /market`
- `GET /book/top`
- `GET /book?depth=10`
- `GET /events`
- `POST /orders`
- `DELETE /orders/{order_id}`

Run it with:

- `cargo run -p solbook-api`

Adapter response model:

- read and mutation endpoints return `{"sequence": <u64>, "data": ...}`
- live updates stream over SSE from `GET /events`
- the stream event name is currently `book_update`

This is the contract a future frontend should target.
# orderbook-ai
