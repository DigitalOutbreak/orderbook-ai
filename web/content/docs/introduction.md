---
title: Introduction
description: Why this terminal exists and how to use it as a learning system for market structure.
---

# Introduction

This terminal is a learning workspace for understanding how an order book behaves.

It is not meant to be a perfect exchange clone first. It is meant to make the moving parts of a trading system easier to read:

- the chart gives market context
- the order book shows resting liquidity
- the order form creates state changes
- recent trades show what executed
- snapshot and event log explain what changed

## How to use this docs section

Use these docs like internal product documentation:

1. Read a concept page.
2. Go back to the terminal.
3. Trigger the behavior yourself.
4. Compare the visual state with what the docs described.

## What to focus on first

If you are new to order books, read in this order:

1. What Is an Order Book
2. Bids and Asks
3. Spread and Mid Price
4. Limit Orders
5. Market Orders
6. Depth and Liquidity

## What this app is optimizing for

This project is optimizing for **quant-dev learning**, not just pretty UI.

That means the important questions are:

- what is resting on the book?
- what just traded?
- why did the spread change?
- what happened when I submitted this order?
- what is state, and what is event history?

Those are the questions the terminal and docs should answer together.
