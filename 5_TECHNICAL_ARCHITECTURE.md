# TECHNICAL_ARCHITECTURE.md

## Purpose

This document bridges the gap between the product requirements and the implementation details for `solbook-core`.

The goal is to keep the architecture:

- clean
- deterministic
- idiomatic in Rust
- easy to learn from
- strong enough for portfolio review
- extensible without architectural cleanup later

---

## System Role

`solbook-core` is an **exchange-core library**, not a full exchange application.

It is responsible for:

- accepting validated order requests
- assigning deterministic sequence numbers
- matching orders using price-time priority
- maintaining order book state
- emitting structured events
- exposing top-of-book and snapshot queries

It is **not** responsible for:

- networking
- databases
- balances
- settlement
- fees
- margin
- liquidation
- distributed coordination
- blockchain integration

Those may become future layers, but they are out of scope for this core crate.

---

## Architecture Principles

- Keep the matching core single-threaded and deterministic
- Separate validation, matching, state, and events
- Use exact decimal arithmetic for financial values
- Use internal sequence numbers for FIFO ordering
- Prefer explicit, readable code over excessive abstraction
- Make invalid states difficult to construct
- Keep public APIs small and meaningful
- Use tests to validate behavior, not implementation trivia
- Keep the core library focused on domain behavior
- Avoid overbuilding future layers into the core

---

## Recommended Source Tree

```text
src/
├─ lib.rs
├─ errors.rs
├─ events.rs
├─ market_config.rs
├─ matching.rs
├─ order.rs
├─ order_book.rs
├─ price_level.rs
├─ types.rs
└─ validation.rs
