---
title: Milestones
---

# Milestones

This repository follows the implementation order defined in the handoff docs.

## Phase 1

Scaffold the crate structure, docs, module tree, dependencies, and README baseline.

## Phase 2

Implement the core domain language:

- market and order identifiers
- sequence numbers
- exact price and quantity types
- order request, stored order, trade, and market config

## Phase 3

Implement typed validation and error handling for:

- market config consistency
- order submission requests
- tick size and lot size alignment
- price and quantity precision

## Phase 4

Implement deterministic book state:

- price levels
- bid and ask side storage
- top-of-book helpers
- snapshots
- invariant checks

## Phase 5

Implement matching behavior:

- resting limit orders
- crossing limit orders
- market order sweeps
- partial fills
- event emission
- structured submission results

## Phase 6

Implement cancellation:

- cancel by order ID
- eager cleanup
- cancellation events
- missing-order handling

## Phase 7

Implement unit and integration tests for matching rules, FIFO, market orders, partial fills, snapshots, and invariants.

## Phase 8

Finish the example flow, doc synchronization, and repository quality gates.

## Current status

The first production baseline in this repository covers all eight phases for the MVP scope, including a benchmark scaffold. Future work should extend the engine only after preserving the documented invariants and deterministic event model.
