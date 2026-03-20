---
title: Learning Path
---

# Learning Path

This file is the guided entry point for studying the repository as if it were a
small self-paced course.

## Read these in order

1. `docs/learning/00-start-here.md`
2. `docs/glossary.md`
3. `docs/learning/01-big-picture.md`
4. `docs/learning/02-core-domain.md`
5. `docs/learning/03-matching-flow.md`
6. `docs/learning/06-visual-guide.md`
7. `docs/learning/07-why-this-design.md`
8. `docs/learning/08-performance-bridge.md`
9. `docs/engine-performance.md`
10. `docs/performance.md`
11. `docs/learning/05-guided-exercises.md`

If you ever feel lost, go back one step instead of forcing yourself forward.

## Recommended study loop

1. Read one chapter slowly.
2. Write down unfamiliar words.
3. Look them up in [docs/glossary.md](/Users/joeyalvarado/Developer/solbook-core/docs/glossary.md).
4. Open the referenced source files.
5. Run the related tests or benchmark.
6. Trace one concrete scenario end to end.
7. Write down what state changed and why.

## Suggested pacing

1. Do not try to finish the whole repo in one sitting.
2. Stop at the point where the words stop feeling obvious.
3. Explain the chapter back to yourself in plain English.
4. Only move on when you can describe the chapter without copying the code.

## Best first live exercises

1. Open [src/order.rs](/Users/joeyalvarado/Developer/solbook-core/src/order.rs) and identify the difference between `NewOrderRequest` and `Order`.
2. Run `cargo test matching -- --nocapture`.
3. Open [src/matching.rs](/Users/joeyalvarado/Developer/solbook-core/src/matching.rs) and narrate one submit path.
4. Run `cargo bench --bench throughput submit_resting_limit_orders_minimal_no_invariants -- --sample-size 10`.
5. After that, open the learning terminal in [`web/`](/Users/joeyalvarado/Developer/solbook-core/web) and compare the UI state with the core snapshots and events.

## What this learning path is trying to teach

- what an order book is before asking you to read matching code
- how to read a deterministic engine without getting lost in implementation detail
- how book state changes during submit, match, rest, and cancel
- how to reason about engine hot paths and failed performance experiments
- how to keep a core library clean while still making it usable in applications
- how to extend the project without accidentally smearing responsibilities across layers

## What this learning path is not trying to do

1. Teach every Rust language feature from scratch.
2. Teach networking before the engine itself makes sense.
3. Pretend this is easy to absorb in one pass.
