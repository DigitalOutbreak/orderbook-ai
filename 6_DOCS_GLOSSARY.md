# DOCS_GLOSSARY.md

# solbook-core — Beginner-Friendly Glossary

This glossary is intentionally beginner-friendly. For important terms, it explains:
- what the term means
- why it matters in this project
- a small example when helpful

## Order book
A data structure that stores active buy and sell orders for a market such as `SOL/USDC`.

Why it matters:
The order book is the core state of the engine. Matching happens by comparing incoming orders to what is already resting in the book.

## Bid
A buy order.

Example:
Buy 5 SOL at 100 USDC.

## Ask
A sell order.

Example:
Sell 5 SOL at 101 USDC.

## Spread
The difference between the best ask and the best bid.

Example:
Best bid = 100
Best ask = 101
Spread = 1

Why it matters:
The spread is a quick measure of market tightness and available liquidity.

## Best bid
The highest resting buy price in the book.

## Best ask
The lowest resting sell price in the book.

## Top of book
A compact view of the best bid and best ask.

Why it matters:
It is the simplest market-data view and is useful in examples, tests, and future APIs.

## Price-time priority
The standard matching rule:
1. Better price executes first
2. For equal price, earlier accepted order executes first

## FIFO
First In, First Out.

Within the same price level, the oldest accepted order gets filled first.

## Price level
A bucket of resting orders at the same price.

Example:
All buy orders at 100.00 belong to the same price level.

## Queue
A data structure where items are added at the back and removed from the front.

Why it matters:
A queue is a natural way to preserve FIFO inside a price level.

## Limit order
An order that specifies a price.

Example:
Buy 3 SOL at 98 USDC.

If it cannot fully execute immediately, it may rest on the book.

## Market order
An order that does not specify a limit price and instead consumes the best available opposite-side liquidity.

Important:
In this project, market orders never rest on the book.

## Resting order
An order, or remaining portion of an order, that stays in the book waiting to be matched.

## Crossing order
An incoming order whose price is good enough to immediately match the opposite side.

Example:
If best ask is 100 and you submit a buy limit at 101, that order crosses the spread.

## Partial fill
When only part of an order is executed.

Example:
Buy 10 SOL at 100
Only 4 SOL is available
Result: 4 filled, 6 remaining

## Full fill
When the entire order quantity is executed.

## Maker
An order that adds liquidity to the book by resting.

## Taker
An order that removes liquidity by matching immediately.

## Liquidity
The amount of executable resting interest in the market.

Why it matters:
More liquidity usually means easier execution and smaller price impact.

## Slippage
The difference between the expected execution price and the actual execution prices achieved.

Why it matters:
Market orders that sweep multiple price levels can experience slippage.

## Tick size
The minimum allowed price increment for a market.

Example:
If tick size is 0.01, valid prices are:
100.00
100.01
100.02

Invalid:
100.005

Why it matters:
The engine uses tick size validation to reject prices that do not align with market rules.

## Lot size
The minimum allowed quantity increment for a market.

Example:
If lot size is 0.001, valid sizes are:
1.000
1.001
1.002

Invalid:
1.0005

Why it matters:
The engine uses lot size validation to reject quantities that do not align with market rules.

## Price precision
How many decimal places a price is expected to use for a market.

## Quantity precision
How many decimal places an order size is expected to use for a market.

## Validation
The process of checking whether an incoming order request is allowed before it reaches matching logic.

Examples:
- quantity must be positive
- price must be positive for limit orders
- price must align to tick size
- quantity must align to lot size

## Rejection
When the engine refuses an invalid request instead of trying to fix it silently.

Why it matters:
This project prefers explicit rejection over hidden normalization.

## Normalization
Automatically changing input data to fit market rules.

Example:
Changing 100.005 to 100.01.

Important:
For this MVP, invalid inputs should be rejected rather than auto-normalized.

## Sequence number
A monotonic number assigned by the engine when an order is accepted.

Why it matters:
This is how FIFO ordering stays deterministic without depending on wall-clock time.

## Deterministic
Given the same valid inputs in the same order, the engine should produce the same outputs every time.

Why it matters:
Determinism is important for testing, debugging, replay, and correctness.

## NewOrderRequest
The external request the caller submits to the engine.

Why it matters:
Separating request input from stored internal order state keeps the API cleaner.

## Order
The internal engine-owned representation of an accepted order.

It includes things like:
- generated order ID
- assigned sequence number
- remaining quantity

## Order ID
A unique identifier for an accepted order.

Why it matters:
Cancellation and event tracking rely on order IDs.

## Cancel
A request to remove a resting order from the book by its order ID.

## Trade
A successful execution between a maker order and a taker order.

A trade usually includes:
- execution price
- executed quantity
- maker order ID
- taker order ID

## Event
A structured record describing a meaningful state transition in the engine.

Examples:
- OrderAccepted
- OrderRejected
- OrderRested
- TradeExecuted
- OrderCancelled
- TopOfBookChanged
- MarketOrderUnfilled

## Event stream
A sequence of emitted events representing what happened during order processing.

Why it matters:
Event streams make behavior easier to test, inspect, and replay.

## Execution report
A structured result returned by the engine after an action such as order submission or cancellation.

## SubmissionResult
A result returned after submitting an order.

It can include:
- generated order ID
- emitted events
- convenience summary fields

## CancelResult
A result returned after attempting to cancel an order.

## Summary helpers
Convenience fields or query results that make the engine easier to inspect without replacing the event stream as the source of truth.

Examples:
- fully_filled
- remaining_qty
- top_of_book
- snapshot(depth)

## TopOfBook
A helper type containing the best bid and best ask.

## Book snapshot
A read-only aggregated view of the book, often limited by depth.

Why it matters:
Snapshots make the engine easier to demo, test, and inspect.

## BookLevelView
A snapshot-friendly representation of one price level, usually including:
- price
- total quantity
- order count

## Market config
The market-specific rules and metadata for a book.

Typical fields:
- market ID
- base symbol
- quote symbol
- tick size
- lot size
- price precision
- quantity precision

## Base asset
The asset being bought or sold.

In `SOL/USDC`, the base asset is `SOL`.

## Quote asset
The asset used to price the base asset.

In `SOL/USDC`, the quote asset is `USDC`.

## Invariant
A condition that should always remain true if the engine is correct.

Examples:
- market orders never rest
- no resting order has zero quantity
- bids contain only buy orders
- asks contain only sell orders
- empty price levels are removed

## Empty level cleanup
The rule that a price level should be removed once it no longer contains any resting orders.

Why it matters:
This keeps best bid/best ask logic cleaner and avoids stale state.

## Bids side
The collection of resting buy orders.

## Asks side
The collection of resting sell orders.

## Matching engine
The logic that compares incoming orders against the book and creates trades.

## Single-threaded core
The design choice that one thread owns and mutates the matching engine state in the MVP.

Why it matters:
This makes correctness and determinism much easier.

## Producer
A component that submits work into the system.

Future example:
A network gateway could act as a producer of order requests.

## Consumer
A component that receives outputs from the system.

Future example:
A market-data adapter could consume engine events.

## Benchmark
A repeatable performance measurement.

Why it matters:
Benchmarks help measure throughput and latency once correctness is stable.
