---
title: What Is an Order Book
description: The order book as the live state of resting buy and sell interest.
---

# What Is an Order Book

An order book is the current in-memory state of active buy and sell interest for a market.

It is not a price chart.
It is not trade history.
It is the **resting liquidity** that incoming orders can interact with.

## Two sides

Every order book has two sides:

- **bids**: buyers willing to pay up to some price
- **asks**: sellers willing to sell down to some price

The best bid is the highest buy price.
The best ask is the lowest sell price.

## What the book tells you

The book answers questions like:

- where is the best available liquidity?
- how much size is near the current market?
- how wide is the spread?
- how deep would I have to trade before price moves?

## Resting state vs event history

This distinction matters:

> The order book is **state**. Recent trades are **events**.

If an order arrives and executes immediately, it may show up in recent trades but never meaningfully rest in the visible book.

## Why quants care

For market microstructure work, the book is one of the main objects of study.

It helps explain:

- queue position
- liquidity concentration
- spread behavior
- price impact
- whether a market feels thin or thick

## Mental model

Think of the order book as the market's current posture.

- the tape shows its recent footsteps
- the chart shows broader price context
- the book shows the immediate executable structure
