---
title: Market Orders
description: How market orders consume liquidity and why they change the book differently from resting limit orders.
---

# Market Orders

A market order does not specify a limit price.

Instead, it says:

> Execute against the best available opposite-side liquidity right now.

## What market orders do

Market orders **remove liquidity**.

They do not rest on the book.

That means they tend to affect:

- recent trades
- top of book
- depth near the touch

## Example

If you submit a market buy:

- it trades against the best ask first
- then the next ask if more size is needed
- then the next level, and so on

This is often called **sweeping the book**.

## Slippage

If there is not enough size at the best price, a market order may execute across multiple levels.

That creates slippage:

> your execution average becomes worse than the best visible price you started with

## Why this matters in the UI

A good terminal should make it obvious that market orders affect:

- the tape
- top of book
- depth shape
- the event log

more than they affect resting visible liquidity.
