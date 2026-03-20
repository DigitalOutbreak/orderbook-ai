# solbook-core

`solbook-core` is a deterministic Rust exchange-core library for a `SOL/USDC` spot market. It provides a single-threaded in-memory order book, exact decimal validation, price-time priority matching, structured event emission, cancellation, and snapshot helpers.

The project should be read engine-first:

- `solbook-core` is the main artifact
- `web/` is the learning terminal and docs interface around the core concepts

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

This repository also includes a learning-oriented web interface in [`web/`](/Users/joeyalvarado/Developer/solbook-core/web) for studying orderbook behavior, chart state, and UI concepts alongside the engine docs.

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

If you want to study the repo as a guided project, start with [`docs/learning-path.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning-path.md) and begin at [`docs/learning/00-start-here.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/00-start-here.md). The visual mental models live in [`docs/learning/06-visual-guide.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/06-visual-guide.md), the design-tradeoff explanation lives in [`docs/learning/07-why-this-design.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/07-why-this-design.md), and the beginner-friendly performance bridge lives in [`docs/learning/08-performance-bridge.md`](/Users/joeyalvarado/Developer/solbook-core/docs/learning/08-performance-bridge.md).

## Web Learning Terminal

The repository includes [`web/`](/Users/joeyalvarado/Developer/solbook-core/web), a Next.js + shadcn study interface for reading docs and exploring trading-terminal UI ideas alongside mock orderbook state.

Run it with:

- `cd web && npm run dev`

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
