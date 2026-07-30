---
title: 03 Matching Flow
---

# 03 Matching Flow

## Goal

Understand how one order moves through the engine.

Do not try to understand every helper at once.
Focus on one concrete story.

If words like maker, taker, crossing, or resting feel fuzzy, stop and look them
up in [docs/glossary.md](../glossary.md)
before reading the code.

## The story to follow

Use this simple scenario:

1. a sell limit order enters the book and rests
2. a buy limit order arrives with a price high enough to cross
3. the engine trades them
4. the book updates
5. events and summaries are returned

That one scenario teaches most of the engine.

If you want a picture before reading the code, keep
[docs/learning/06-visual-guide.md](../learning/06-visual-guide.md)
open beside this chapter.

## Read these files

- [src/validation.rs](../../src/validation.rs)
- [src/price_level.rs](../../src/price_level.rs)
- [src/order_book.rs](../../src/order_book.rs)
- [src/matching.rs](../../src/matching.rs)

## Tests to read with the code

- [tests/matching.rs](../../tests/matching.rs)
- [tests/partial_fills.rs](../../tests/partial_fills.rs)
- [tests/fifo.rs](../../tests/fifo.rs)

## The engine path in plain English

For a new order, the engine roughly does this:

1. check the request
2. assign identifiers
3. decide whether it crosses the opposite side
4. if it crosses, consume the oldest maker at the best price
5. repeat until the order is filled or can no longer cross
6. if a limit order still has quantity left, rest it on the book
7. return events and summary data

## The main ideas to understand

### Best price comes before time priority

The engine first chooses the best price on the opposite side.

Only after price is chosen does FIFO matter inside that price level.

### FIFO only matters among equals

If two orders rest at the same price, the earlier accepted order executes first.

### Maker price sets the trade price

The incoming order does not set a new trade price.

The trade executes at the resting maker's price.

### Market orders do not rest

If a market order cannot fully fill, the leftover quantity expires.

### Limit orders can rest

If a limit order has leftover quantity and cannot keep crossing, it becomes a
resting order.

## The book structures involved

While reading, keep this mental picture in mind:

- `BTreeMap` chooses the best bid or ask price level
- `PriceLevel` preserves FIFO inside one price
- `OrderLocator` helps cancel find a resting order later

You do not need to understand every implementation detail yet.
You only need to understand each structure's job.

## What to watch carefully

- when IDs and sequence numbers are assigned
- when the core decides an order crosses
- why trade price comes from the maker
- why market orders never rest
- where empty levels are removed
- how `OrderLocator` and `PriceLevel` cooperate to support cancel
- where invariant checking happens after mutation
- what invariants check, and what they do not change

## Words to look up if needed

- [Crossing order](../glossary.md)
- [Maker](../glossary.md)
- [Taker](../glossary.md)
- [Partial fill](../glossary.md)
- [Resting order](../glossary.md)
- [Price level](../glossary.md)
- [Invariant](../glossary.md)

## Study prompt

Open `submit_order` in [src/matching.rs](../../src/matching.rs) and explain it in ordinary language.

If you can describe:

- what gets validated
- when matching starts
- when resting happens
- what gets returned

without leaning on Rust-specific words, you understand the flow.
