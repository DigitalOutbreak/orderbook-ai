---
title: Limit Orders
description: How limit orders rest, cross, or partially execute against the existing book.
---

# Limit Orders

A limit order specifies a price.

That price acts like a cap or floor:

- buy limit: do not pay more than this price
- sell limit: do not sell cheaper than this price

## Two outcomes

A limit order can do one of two main things:

1. **Rest on the book**
2. **Cross the spread and execute immediately**

## Resting example

If the best ask is `172.02` and you submit a buy limit at `171.90`, your order does not cross.

It rests on the bid side as liquidity.

## Crossing example

If the best ask is `172.02` and you submit a buy limit at `172.02` or above, your order crosses.

It can execute immediately against the resting ask.

## Partial fills

If your order size is larger than the resting size at the top level:

- part may execute immediately
- the remainder may rest on the book at your limit price

## Why limit orders matter

Limit orders are where the order book comes from.

Without resting limit orders, there is no visible liquidity structure to study.
