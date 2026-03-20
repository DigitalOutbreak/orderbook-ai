---
title: Matching Engine Basics
description: The minimum matching concepts needed to understand what the terminal is showing you.
---

# Matching Engine Basics

The matching engine is the system that decides how incoming orders interact with resting orders.

## Resting order vs incoming order

When a new order arrives, the engine checks:

1. Is the order valid?
2. Does it cross the current opposite side?
3. If yes, how much executes?
4. If anything remains, should it rest?

## Maker and taker

- **maker** = the resting order already on the book
- **taker** = the incoming order that triggers execution

This distinction matters for microstructure analysis and fee models.

## FIFO

At one price level, execution priority is usually FIFO:

> first in, first out

So if two asks rest at the same price, the older one executes first.

## Events vs state

The engine produces two kinds of useful information:

- **state**: the resulting order book
- **events**: accepts, trades, cancellations, top-of-book changes

The best learning workflow is to inspect both.

## In this terminal

That is why the terminal includes:

- orderbook views for state
- recent trades for executions
- event log for narrative state changes
- snapshot/debug views for structured inspection
