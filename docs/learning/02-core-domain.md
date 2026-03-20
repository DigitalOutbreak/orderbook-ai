---
title: 02 Core Domain
---

# 02 Core Domain

## Goal

Learn the main types before trying to follow the matching logic.

This chapter is about vocabulary and ownership.

If a term is unfamiliar, stop and look it up in
[docs/glossary.md](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
before continuing.

## Why this matters

A lot of confusion in exchange code comes from mixing up:

- input sent by the caller
- state owned by the engine
- output produced by a trade

This repo intentionally keeps those separate.

## Read these files

- [src/types.rs](/Users/joeyalvarado/Developer/solbook-core/src/types.rs)
- [src/order.rs](/Users/joeyalvarado/Developer/solbook-core/src/order.rs)
- [src/market_config.rs](/Users/joeyalvarado/Developer/solbook-core/src/market_config.rs)
- [src/errors.rs](/Users/joeyalvarado/Developer/solbook-core/src/errors.rs)
- [src/events.rs](/Users/joeyalvarado/Developer/solbook-core/src/events.rs)

## The most important type split

### `NewOrderRequest`

This is what an outside caller asks for.

It is a request, not a promise.

It may still be rejected.

### `Order`

This is what the engine stores after it accepts an order.

It has engine-owned fields such as:

- `order_id`
- `sequence`
- `remaining_qty`

### `Trade`

This is an execution record.

It is not a request and not a resting order.

It is the record of something that already happened.

## Why wrapper types exist

You will see types like:

- `Price`
- `Quantity`
- `OrderId`
- `SequenceNumber`

These exist so the engine does not pass around loose primitives everywhere.

That helps because:

- prices and quantities have market rules
- IDs and sequences mean different things
- accidental mixing becomes harder

## The market config

`MarketConfig` is the rulebook for the book.

It defines:

- which market this book is for
- tick size
- lot size
- allowed decimal precision

That is why validation can be strict and deterministic.

## Errors are part of the design

Do not treat errors as side noise.

In this project, typed errors are part of the engine contract.

That means:

- invalid orders get rejected explicitly
- bad data is not silently rounded or fixed
- a reader can tell whether a failure came from validation or book state

## A good beginner question

Pick one field and trace it across the system.

Good choices:

- `remaining_qty`
- `price`
- `order_id`
- `sequence`

For that field, ask:

1. where is it introduced?
2. who owns it?
3. when can it change?
4. which outputs expose it?

## Words to look up if needed

- [NewOrderRequest](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Order](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Trade](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Market config](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Validation](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Rejection](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Sequence number](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)

## Check yourself

By the end of this chapter, you should be able to explain:

- why `NewOrderRequest` and `Order` are not the same thing
- why `Trade` is a separate output type
- why the engine wraps raw numbers in domain types
- why validation belongs before matching
