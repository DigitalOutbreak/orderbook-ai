# ARCHITECTURE_EXPLANATION.md

## Purpose

This file explains the architecture sections in plain English so the implementation agent knows what each part means, what file it belongs to, and what rules it should follow.

Use this alongside the execution prompt and the technical architecture file.

---

## Module Responsibilities

Module responsibilities define what each source file is responsible for.

The goal is to prevent everything from being dumped into one giant file and to keep the codebase organized by responsibility.

Recommended mapping:

- `types.rs` = core domain value types like IDs, price, quantity, and sequence number
- `order.rs` = order-related structs and enums such as side, order type, order request, stored order, and trade
- `market_config.rs` = market rules such as tick size, lot size, precision, and asset symbols
- `price_level.rs` = FIFO queue behavior for resting orders at one price
- `order_book.rs` = main order book state, bid side, ask side, and sequencing ownership
- `matching.rs` = matching logic and execution flow
- `validation.rs` = request validation and market rule enforcement
- `events.rs` = emitted events and structured result objects
- `errors.rs` = typed engine errors

Do not blur responsibilities between modules without a strong architectural reason.

---

## Domain Model

The domain model is the vocabulary of the system.

It defines the important business objects the engine uses and should make the code easier to reason about.

Examples of domain concepts:
- market id
- order id
- sequence number
- price
- quantity
- order
- trade
- market config

The domain model should be explicit and strongly typed where it improves correctness and readability.

---

## Order Model

The order model defines:
- what an incoming order request looks like
- what an accepted and stored order looks like
- what a trade execution looks like

Use this separation:

- `NewOrderRequest` = caller input
- `Order` = internal engine-owned state
- `Trade` = execution record

Do not merge caller input and stored order state carelessly.
They have different responsibilities.

---

## Market Config

Market config is the rulebook for a market.

It should define:
- market id
- base symbol
- quote symbol
- tick size
- lot size
- price precision
- quantity precision

Use market config to validate orders and keep the engine market-aware while still staying generic.

For this project, the first market is `SOL/USDC`, but the design should remain internally market-agnostic.

---

## Book Structure

Book structure means how resting orders are stored.

The order book should have:
- a bid side
- an ask side
- price levels
- FIFO ordering inside each price level

A price level is all resting orders at one price.
FIFO means the earliest accepted order at that price gets filled first.

---

## Order Book

The order book is the main state owner.

It should own:
- market config
- bid levels
- ask levels
- next sequence number

It is responsible for preserving invariants and exposing the main engine API.

The order book should be the clear center of state ownership.

---

## Public API

The public API is what outside callers are allowed to use.

Keep it small and coherent.

At minimum, it should expose:
- order submission
- order cancellation
- best bid query
- best ask query
- top-of-book query
- snapshot query

Do not expose unnecessary internals just for convenience.

A small, coherent public API is usually better than a broad convenience surface.

---

## Result Types

Result types are structured outputs returned by the engine.

Use structured result objects for:
- order submission
- cancellation

These should include:
- relevant identifiers
- emitted events
- optional summary helpers such as remaining quantity or whether the order was fully filled

Events remain the source of truth.
Summary helpers are convenience features and must never contradict events.

---

## Events

Events are structured records of meaningful state transitions in the engine.

Examples:
- order accepted
- order rejected
- order rested
- trade executed
- order cancelled
- top of book changed
- market order partially unfilled

Events should be:
- explicit
- deterministic
- inspectable
- truthful to what actually happened

Do not emit misleading events just to satisfy a test or convenience layer.

---

## Validation

Validation is the rule-checking step before matching.

It must reject:
- non-positive quantity
- non-positive price for limit orders
- invalid tick size alignment
- invalid lot size alignment
- market-rule violations

Do not silently normalize bad input in this project.
Reject invalid input explicitly.

Validation should be separate from matching logic.

---

## Matching Rules

Matching rules define how incoming orders interact with resting orders.

### Limit Buy

A limit buy should:
- match against the lowest ask prices first
- only match while the ask price is less than or equal to the buy price
- preserve FIFO inside each ask price level
- rest the remainder on the bid side if quantity remains

### Limit Sell

A limit sell should:
- match against the highest bid prices first
- only match while the bid price is greater than or equal to the sell price
- preserve FIFO inside each bid price level
- rest the remainder on the ask side if quantity remains

### Market Buy

A market buy should:
- consume the lowest asks until filled or the ask book is exhausted
- never rest on the book
- expire any unfilled remainder immediately

### Market Sell

A market sell should:
- consume the highest bids until filled or the bid book is exhausted
- never rest on the book
- expire any unfilled remainder immediately

---

## Cleanup

Cleanup means removing empty or invalid leftover state after changes.

At minimum:
- remove empty price levels eagerly
- avoid stale best bid and best ask state
- do not leave zero-quantity resting orders behind

Cleanup is part of correctness, not an optional polish step.

---

## Snapshot and Summary Helpers

These are read-only helpers that make the engine easier to inspect, test, and demo.

Examples:
- top of book
- book level view
- book snapshot
- remaining quantity
- fully filled status

These helpers should improve usability without replacing the event stream as the true record of what happened.

---

## Invariants

Invariants are conditions that must always remain true if the engine is correct.

Examples:
- market orders never rest on the book
- no resting order has zero remaining quantity
- bids contain only buy orders
- asks contain only sell orders
- FIFO is preserved by sequence number
- empty price levels are removed
- resting orders conform to market rules
- all resting orders belong to the configured market

Implementation and tests must preserve these invariants at all times.

---

## Testing Strategy

Use different test layers for different purposes.

### Unit tests

Use unit tests for:
- isolated logic
- FIFO behavior
- validation rules
- helper behavior
- top-of-book and snapshot behavior where isolated

### Integration tests

Use integration tests for:
- end-to-end order submission
- matching workflows
- partial fills
- cancellation
- market sweeps
- best bid and best ask updates
- snapshot output after state changes

### Invariant-oriented tests

Use invariant tests for:
- checking that state remains valid after operations
- ensuring no zero-quantity resting orders remain
- ensuring price levels are cleaned up correctly
- ensuring side separation remains correct

Prefer behavior-focused tests over fragile implementation-detail tests.

---

## Design Constraints

These are limits that protect the architecture.

For this project:
- single-threaded core
- in-memory state
- no networking in the core
- no persistence in the core
- no balances, fees, or settlement in the core
- no async runtime in the core
- no lock-free internals in the core
- no floats for financial values

These constraints are deliberate and should not be bypassed casually.

---

## Future Extensibility

The design should leave room for future systems, but those future systems must not distort the current implementation.

Possible future extensions:
- multi-market router
- replay engine
- benchmark suite
- market data feed
- balance or ledger layer
- settlement simulation

Do not build those into the core yet.
Design for future extension, but implement the core cleanly first.

---

## Final Guidance for the Implementation Agent

When interpreting the architecture:

- keep file responsibilities clear
- keep the domain model explicit
- keep validation separate from matching
- keep matching rules deterministic
- keep state ownership obvious
- keep events truthful
- keep helpers secondary to real state and event flow
- keep invariants central to both code and tests
- keep the core focused on exchange behavior, not future surrounding systems
