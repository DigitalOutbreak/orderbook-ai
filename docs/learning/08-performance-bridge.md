---
title: 08 Performance Bridge
---

# 08 Performance Bridge

This chapter is a bridge into the heavier performance notes.

Read this before [docs/engine-performance.md](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md)
if words like "hot path," "allocation," and "throughput" still feel abstract.

## What performance means here

For this repo, performance mostly means:

- how much work the engine does per order
- how much memory it allocates while doing that work
- which data structures help or hurt under load

You do not need advanced math to follow this.

## Three words to know first

### Hot path

The hot path is the code that runs most often or matters most for speed.

In this project, that usually means:

- validate
- inspect best price
- match
- update state
- return results

### Allocation

An allocation means the program asks for heap memory.

Allocations are not always bad.
They just matter because repeated allocations can add overhead.

### Throughput

Throughput means how much work can be completed over time.

For this repo, think:

- how many orders can this path process efficiently?

## What is cheap vs expensive in this engine

Usually cheaper:

- copying small value types like `Price` or `OrderId`
- reading the best price from the edge of a `BTreeMap`
- popping the head of a price level

Usually more expensive:

- creating lots of result objects
- building event vectors
- cloning `MarketId` into trades
- doing more bookkeeping on every mutation
- walking large parts of the book for invariant checks

## Why benchmarks matter

Your intuition will often be wrong.

That is normal.

A change can:

- improve one microcase
- hurt the main engine
- look cleaner but run slower
- look more advanced but allocate more

This project already hit that exact situation.

So the rule is:

measure first, trust the benchmark, then decide.

## The key beginner lesson

Do not think of performance as "make everything faster."

Think of it as:

1. find the expensive path
2. form a concrete hypothesis
3. measure the result
4. keep or reject the change

That is the habit you want to learn from this repo.

## What to focus on when reading the performance docs

Look for answers to these questions:

- What code runs for every accepted order?
- What work happens only for rich results?
- What work happens only because of safety checks?
- Which structure was chosen for clarity, and which for speed?
- Which optimization ideas failed?

## What not to do while learning performance

Do not immediately chase:

- lock-free ideas
- async ideas
- OS tuning
- network tuning
- exotic low-level containers

Those may matter later, but they are not the first lesson of this project.

The first lesson is:

understand the current cost model before trying to replace it.
