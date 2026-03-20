---
title: 01 Big Picture
---

# 01 Big Picture

## Goal

Understand what this repository is before worrying about implementation detail.

## The short version

This repository is mainly one thing:

- a deterministic matching engine

It also contains two smaller secondary things:

- a web learning terminal
- long-form docs for studying the engine and trading concepts

If you remember only one sentence from this chapter, remember this:

`solbook-core` is the real project. The `web/` app and docs are learning surfaces around it.

## What the engine is responsible for

The engine is responsible for:

- checking whether an order is valid
- deciding whether it trades immediately
- updating the in-memory book
- preserving price-time priority
- returning events and summary results

## What the engine is not responsible for

The engine is not responsible for:

- HTTP
- databases
- user accounts
- balances
- fees
- blockchain integration
- frontend code

That separation is a professional design choice. It keeps the core logic
independent from delivery mechanisms.

## The two layers in plain English

### `solbook-core`

This is the machine.

It owns the rules and the state.

### `web/`

This is the study interface.

It lets you read docs, inspect orderbook state, submit mock orders, and study
how market structure is visualized in a terminal-style UI.

## Why determinism matters

Deterministic means:

- same valid inputs
- in the same order
- produce the same outputs every time

That matters because matching engines must be explainable and replayable.
If two runs can disagree, debugging and trust both get harder.

## A simple mental model

Think of the engine as a ledger of open interest:

- bids are people willing to buy
- asks are people willing to sell
- when prices overlap, a trade can happen
- when prices do not overlap, the order rests on the book

## The main path through the engine

When a new order comes in, the engine roughly does this:

1. validate the request
2. assign an `OrderId` and `SequenceNumber`
3. compare it against the best prices on the opposite side
4. execute trades if it crosses
5. rest leftover quantity if it is a limit order
6. return events and a summary

## Files to open while reading

- [src/lib.rs](/Users/joeyalvarado/Developer/solbook-core/src/lib.rs)
- [src/order_book.rs](/Users/joeyalvarado/Developer/solbook-core/src/order_book.rs)
- [src/matching.rs](/Users/joeyalvarado/Developer/solbook-core/src/matching.rs)
- [web/components/orderbook/orderbook-terminal.tsx](/Users/joeyalvarado/Developer/solbook-core/web/components/orderbook/orderbook-terminal.tsx)
- [web/components/market-chart/market-chart-panel.tsx](/Users/joeyalvarado/Developer/solbook-core/web/components/market-chart/market-chart-panel.tsx)
- [web/app/docs/[slug]/page.tsx](/Users/joeyalvarado/Developer/solbook-core/web/app/docs/[slug]/page.tsx)

## Words to look up if needed

If any of these feel fuzzy, stop and read them in the glossary:

- [Deterministic](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Order book](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Bid](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Ask](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Price-time priority](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Event stream](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)

## Check yourself

By the end of this chapter, you should be able to answer:

- Why is the engine a library instead of a server?
- Why is the frontend not the main project?
- What does determinism buy us?
- Why should a future frontend consume outputs from the core instead of reimplementing matching?
