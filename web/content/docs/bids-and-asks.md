---
title: Bids and Asks
description: How to read the two sides of the book and what each side represents.
---

# Bids and Asks

The order book is split into bids and asks.

## Bids

A **bid** is a buy order.

In `SOL/USDC`, a bid means:

> "I want to buy SOL and pay USDC."

Higher bids are more aggressive, because they are willing to pay more.

## Asks

An **ask** is a sell order.

In `SOL/USDC`, an ask means:

> "I want to sell SOL and receive USDC."

Lower asks are more aggressive, because they are willing to sell cheaper.

## Why they matter together

The distance between the best bid and best ask defines the spread.

When bids move up or asks move down, the spread narrows.
When they move apart, the spread widens.

## Reading the ladder

In a terminal-style ladder:

- bid-side shading shows resting buy interest
- ask-side shading shows resting sell interest
- stronger fills often mean larger size or cumulative depth

## Common mistake

Do not confuse bids and asks with executed buys and sells.

Executed trades are not the same thing as resting orders.

- **bids/asks** are what is waiting
- **trades** are what already happened
