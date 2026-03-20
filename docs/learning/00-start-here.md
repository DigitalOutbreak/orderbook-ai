---
title: 00 Start Here
---

# 00 Start Here

This repo can teach you a lot, but only if you study it in the right order.

If you jump straight into `src/matching.rs`, it will feel dense and mechanical.
That is normal. The code assumes you already know what an order book is, what a
maker and taker are, and why FIFO matters. This chapter exists to remove that
problem.

## What you are looking at

This project has two relevant surfaces:

- `solbook-core`: the real project, a matching engine library
- `web/`: the learning terminal and docs interface built on top of the engine concepts

For learning, the engine comes first.

## What a matching engine does

A matching engine is a program that keeps track of buy orders and sell orders.
When a new order comes in, it decides:

1. is this order valid?
2. can it trade right now?
3. if yes, with which resting orders?
4. if some quantity is left over, should it rest on the book?
5. what events or results should be returned?

That is the heart of this repository.

## What you do not need to know yet

You do not need to understand:

- networking
- async Rust
- TCP details
- web servers
- advanced data-structure theory

Those topics matter later, but they are not required to understand the core
matching flow.

## Words you should learn first

Before reading code, make sure these terms are at least somewhat familiar:

- [Order book](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Bid](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Ask](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Price level](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [FIFO](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Limit order](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Market order](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Maker](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Taker](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Trade](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)
- [Price-time priority](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md)

If any of those are fuzzy, pause and read the glossary first.

## How to study this repo without getting lost

Use this rule:

1. learn the words
2. learn the state
3. learn one order path
4. learn the data structures
5. learn the performance tradeoffs
6. only then study the learning UI and how it visualizes the engine

Do not try to understand the whole repository at once.

## What “understanding” means here

You understand this engine when you can explain, in plain English:

- where bids are stored
- where asks are stored
- how the engine finds the best price
- why FIFO is preserved
- what happens when a limit order crosses
- what happens when a market order cannot fully fill
- how cancel finds the right order

## First source files to open

Start with:

- [src/lib.rs](/Users/joeyalvarado/Developer/solbook-core/src/lib.rs)
- [src/order.rs](/Users/joeyalvarado/Developer/solbook-core/src/order.rs)
- [src/order_book.rs](/Users/joeyalvarado/Developer/solbook-core/src/order_book.rs)

Do not start with the UI.
If you want a visual aid later, use the learning terminal in [`web/`](/Users/joeyalvarado/Developer/solbook-core/web) after the core flow makes sense.

## First mindset to keep

This is not mainly a web app.

It is an in-memory machine that updates book state deterministically.
The frontend is one learning surface around that machine.
