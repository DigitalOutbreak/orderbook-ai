---
title: Glossary
---

# Glossary

This glossary explains the main exchange terms used by `solbook-core`, what they mean, and why they matter in this engine.

How to use it:

1. When a chapter uses a word you do not recognize, stop here.
2. Read only the term you need.
3. Go back to the chapter and reread the paragraph.

This file is intentionally repetitive. That is a feature, not a bug. It is
meant to help the learning material feel less dense.

## Order book

The order book is the engine's in-memory state for active buy and sell interest. Matching happens by comparing an incoming order with the resting orders already in the book.

## Bid

A bid is a buy order. In `SOL/USDC`, a bid means someone wants to buy `SOL` and pay `USDC`.

## Ask

An ask is a sell order. In `SOL/USDC`, an ask means someone wants to sell `SOL` for `USDC`.

## Spread

The spread is the difference between the best ask and the best bid. Example: best bid `100.00`, best ask `100.05`, spread `0.05`.

## Best bid

The highest resting buy price in the book. This is the most aggressive bid currently available.

## Best ask

The lowest resting sell price in the book. This is the cheapest ask currently available.

## Top of book

The compact view containing the current best bid and best ask. `solbook-core` exposes this through `TopOfBook`.

## FIFO

First in, first out. If two orders rest at the same price, the one accepted earlier must execute first.

## Price level

A price level is all resting orders at the same price. The engine stores each price level in FIFO order and keeps enough internal indexing to remove a known resting order without scanning the whole level.

## Queue

A queue adds items at the back and removes them from the front. That is the abstract FIFO behavior required inside one price level, even though the current engine implements that behavior with slot-indexed linked storage instead of a raw queue container.

## Slot

A slot is an internal index used to locate a resting order inside a price level. The engine uses slots to support level-local removal without a linear scan and stores that slot inside `OrderLocator`.

## Limit order

A limit order includes a price cap or floor. If it cannot fully execute immediately, any remaining quantity may rest on the book.

## Market order

A market order has no limit price. It consumes the best available opposite-side liquidity until it is filled or the book runs out. In this engine, market orders never rest.

## Resting order

A resting order is an order, or remainder of an order, that stays on the book waiting for a future match.

## Crossing order

A crossing order is an incoming order whose price is good enough to match the current opposite side immediately.

Example:
If the best ask is `100.00`, then a buy limit at `100.00` or `100.01` is crossing because it can execute right away.

## Maker

The maker is the resting order in a trade. Makers add liquidity.

## Taker

The taker is the incoming order that triggers the trade. Takers remove liquidity.

## Liquidity

Liquidity is the executable resting interest already available in the book. More liquidity usually means easier execution.

## Slippage

Slippage is the difference between the expected price and the actual execution prices achieved. A market order that sweeps multiple levels can experience slippage.

## Tick size

The minimum legal price increment for a market. If the tick size is `0.01`, a price like `100.005` is invalid and must be rejected.

## Lot size

The minimum legal quantity increment for a market. If the lot size is `0.001`, a size like `1.0005` is invalid and must be rejected.

## Price precision

The maximum number of decimal places allowed for prices in the market. This engine rejects extra precision instead of normalizing it.

## Quantity precision

The maximum number of decimal places allowed for quantities in the market.

## Validation

Validation is the pre-matching rule check. It ensures positive quantities, positive limit prices, correct market ID, tick alignment, lot alignment, and precision limits.

## Rejection

A rejection means the engine refuses invalid input. `solbook-core` rejects invalid orders explicitly instead of silently adjusting them.

## Normalization

Normalization would mean automatically changing a bad input to fit market rules. This engine does not do that for orders.

## Sequence number

A monotonic number assigned when the engine accepts an order. It is the deterministic FIFO tiebreaker inside a price level.

## Deterministic

Deterministic means the same valid inputs, submitted in the same order, produce the same state and events every time.

## NewOrderRequest

The caller-facing input type for a new order submission. It stays separate from stored `Order` state so external input and engine-owned state do not blur together.

## Order

The internal stored form of an accepted order. It includes generated identifiers, remaining quantity, and the assigned sequence number.

## OrderId

The unique identifier for an accepted order. `cancel_order` uses it to target a resting order.

## Cancel

Cancel means asking the engine to remove a resting order by `OrderId`. In this project, cancellation only works for orders that still have resting quantity on the book.

## Partial fill

A partial fill means only part of an order executed. The remaining quantity may either keep matching, rest, or expire immediately if the order was a market order.

## Full fill

A full fill means the entire requested quantity has executed. In `SubmissionResult`, this is exposed through `fully_filled`.

## Trade

A trade records an execution between a maker and a taker, including price, quantity, both order IDs, and both sequence numbers.

## Venue

Venue means where a trade happened or which system produced it.

In a real trading terminal, venue might be:

- an exchange
- a matching engine
- an ECN
- an internal simulator

In a learning UI, venue often helps answer, "is this real market activity or a demo feed?"

## Event

An event is a structured record of a state transition such as acceptance, a trade, a cancellation, or a top-of-book change.

## Event stream

The ordered list of events returned by submission or cancellation. It is the inspectable history of what happened during that action.

## Execution report

An execution report is the structured result object returned by the engine. `SubmissionResult` and `CancelResult` are the execution-report style outputs in this crate.

## SubmissionResult

The result of `submit_order`. It includes acceptance status, emitted events, remaining quantity, top-of-book, and any rejection or invariant error.

## CancelResult

The result of `cancel_order`. It includes whether cancellation succeeded, the cancelled quantity, emitted events, top-of-book, and any error.

## SubmissionSummary

The lean result of `submit_order_minimal`. It exists for engine study and benchmarking, and intentionally omits event vectors and top-of-book helper objects.

## CancelSummary

The lean result of `cancel_order_minimal`. It exists for engine study and benchmarking, and intentionally omits event vectors and top-of-book helper objects.

## Summary helpers

Summary helpers are convenience fields such as `fully_filled`, `remaining_qty`, `best_bid`, `best_ask`, `top_of_book`, and `snapshot(depth)`. They help inspection but do not replace the event stream.

## TopOfBook

The helper type containing the best bid and best ask views.

## Book snapshot

A read-only aggregated depth view of the order book. `snapshot(depth)` returns bid levels highest-first and ask levels lowest-first.

## BookLevelView

One aggregated level in a snapshot or top-of-book response. It includes price, total quantity, and order count.

## Market config

The market rulebook. It defines market ID, base asset, quote asset, tick size, lot size, and precision constraints.

## Base asset

The asset being bought or sold. In `SOL/USDC`, the base asset is `SOL`.

## Quote asset

The asset used to price the base asset. In `SOL/USDC`, the quote asset is `USDC`.

## Mock

Mock means fake data used for learning, testing, or UI development.

If a trade row says the source is mock, it means the trade did not come from a live exchange. It was generated to help you study the interface or engine behavior safely.

## Simulated

Simulated means behavior that imitates a real market or engine without being connected to one.

A simulated trade feed may look realistic, but it is still synthetic. In practice, "mock" and "simulated" are often close in meaning, but simulated usually suggests behavior shaped to resemble real execution more closely.

## Paper trading

Paper trading means placing pretend orders and tracking pretend fills without using real money.

It is useful for learning interfaces and workflows, but it is still not live trading.

## Live

Live means connected to real market or production activity.

If a book or trade feed is live, the prices, orders, and trades come from an actual running market or production system rather than a mock or simulator.

## Invariant

An invariant is a condition that must always remain true if the engine is correct. Examples include no zero-quantity resting orders and no empty resting levels.

## InvariantPolicy

The policy that controls whether the engine performs local mutation checks, a full post-mutation invariant walk, or no invariant walk at all. `Local` is the default. `Full` is the strongest check. `Never` exists for engine study and benchmarking.

## Empty level cleanup

When the last order leaves a price level, the engine removes that level immediately. This keeps top-of-book queries accurate and state compact.

## Bids side

The bids side is the collection of all resting buy orders, grouped into price levels and ordered from highest price to lowest.

## Asks side

The asks side is the collection of all resting sell orders, grouped into price levels and ordered from lowest price to highest.

## Producer

A producer is any future component that submits orders into the engine, such as a network gateway or replay runner.

## Consumer

A consumer is any future component that reads outputs from the engine, such as an event sink or market-data adapter.

## Matching engine

The matching engine is the logic that compares incoming orders to resting liquidity and produces trades.

## Single-threaded core

This crate keeps one mutation owner for the order book. That simplifies determinism, reviewability, and correctness.

## Benchmark

A benchmark is a repeatable performance measurement. The crate is structured so a benchmark suite can be added without refactoring the core.

## Prototype benchmark

A prototype benchmark measures candidate data structures outside the main engine. In this repo, `price_level_prototypes` is used to compare level-storage ideas before risking the main implementation.

## Slab

A slab is a storage pattern that keeps objects in indexed slots and reuses freed slots. It can help with stable handles and O(1) removal, but it adds bookkeeping and can still lose if the design is too hash-heavy or pointer-heavy.

## Price-time priority

The matching rule used by the engine: better prices execute before worse prices, and equal prices execute in FIFO order.
