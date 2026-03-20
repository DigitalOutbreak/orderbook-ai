---
title: Performance
---

# Performance

This document records the benchmark and profiling path for the engine.

For the structural explanation of the matching engine through a performance
lens, read [`docs/engine-performance.md`](/Users/joeyalvarado/Developer/solbook-core/docs/engine-performance.md).

## Benchmark command

Run:

- `cargo bench --bench throughput -- --sample-size 10`
- `cargo bench --bench price_level_prototypes -- --sample-size 10`

For a lightweight local profile wrapper:

- `./scripts/profile_bench.sh`

On macOS the helper uses `/usr/bin/time -l`; on Linux it uses `/usr/bin/time -v`.

## Current baseline

The most recent local benchmark pass produced approximately:

- `submit_resting_limit_orders`: `7.80 ms` to `7.83 ms`
- `cross_seeded_book`: `395.8 µs` to `396.9 µs`
- `cancel_populated_book`: `4.00 ms` to `4.02 ms`
- `sweep_single_price_fifo`: `8.02 ms` to `8.04 ms`

The benchmark suite also includes lean-path comparisons:

- `submit_resting_limit_orders_minimal`
- `submit_resting_limit_orders_minimal_no_invariants`
- `cross_seeded_book_minimal`
- `cross_seeded_book_minimal_no_invariants`
- `cancel_populated_book_minimal`
- `cancel_populated_book_minimal_no_invariants`
- `cancel_deep_single_price_level`
- `cancel_deep_single_price_level_minimal`
- `cancel_deep_single_price_level_minimal_no_invariants`

The repository also includes prototype-only price-level structure comparisons:

- `prototype_enqueue_vecdeque_1000`
- `prototype_enqueue_slab_1000`
- `prototype_enqueue_dense_1000`
- `prototype_fifo_sweep_vecdeque_1000`
- `prototype_fifo_sweep_slab_1000`
- `prototype_fifo_sweep_dense_1000`
- `prototype_deep_cancel_vecdeque_1000`
- `prototype_deep_cancel_slab_1000`
- `prototype_deep_cancel_dense_1000`

## Allocation profile

The most recent `./scripts/profile_bench.sh` run on macOS reported approximately:

- maximum resident set size: `57.1 MB`
- peak memory footprint: `38.7 MB`
- retired instructions: `895M`
- elapsed cycles: `275M`
- wall time for the full benchmark suite: `38.0 s`

These are whole-suite numbers, not per-benchmark isolated allocations. They are useful for regression tracking, not for attributing one code path precisely.

These numbers are environment-specific and should only be used as local regression references, not absolute performance claims.

## Profiling guidance

Use the benchmark suite before attempting engine optimizations.

Suggested order:

1. Run `./scripts/profile_bench.sh`.
2. Compare against the stored Criterion baseline.
3. Only optimize paths that show stable pressure across repeated runs.

Current likely hot paths:

- resting order insertion
- resting order insertion with rich result assembly
- resting order insertion with lean summaries
- post-mutation invariant walks
- best-level matching loops
- FIFO sweep execution
- cancellation on a populated book
- cancellation inside a deep single price level
- mixed insert-and-cross churn

Current read on optimization pressure:

- CPU hot spots appear more important than gross memory pressure at the current workload size.
- The full suite footprint is moderate for a benchmark process and does not yet justify structural redesign.
- If optimization continues, prioritize per-benchmark allocation tracing or CPU sampling around matching loops and event/top-of-book construction.
