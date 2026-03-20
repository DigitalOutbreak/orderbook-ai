---
title: Engine Performance Notes
---

# Engine Performance Notes

This file explains `solbook-core` as a matching engine, not as an API project.

## What the hot path is

For an accepted order, the hot path is:

1. validate the request
2. assign `OrderId` and `SequenceNumber`
3. inspect the best opposing price
4. pop the best price level
5. consume makers from the front of that level
6. emit trade events
7. either rest leftover quantity or finish

The most performance-sensitive code is in:

- [src/matching.rs](/Users/joeyalvarado/Developer/solbook-core/src/matching.rs)
- [src/order_book.rs](/Users/joeyalvarado/Developer/solbook-core/src/order_book.rs)
- [src/price_level.rs](/Users/joeyalvarado/Developer/solbook-core/src/price_level.rs)

## Current in-memory design

### Price levels

- bids: `BTreeMap<Price, PriceLevel>`
- asks: `BTreeMap<Price, PriceLevel>`

Why:

- sorted maps make best-price discovery straightforward
- deterministic ordering is easy to reason about
- insertion and removal by price are `O(log L)` where `L` is number of price levels

This is a good learning structure because it makes price-time priority explicit.

### FIFO within a level

Each `PriceLevel` now stores orders in slot-indexed linked storage.

Why:

- append resting orders at the tail: `O(1)`
- match the oldest maker from the head: `O(1)`
- remove a known resting order by stored slot: `O(1)`

This keeps FIFO explicit while avoiding a linear scan inside one deep level during cancellation.

### Cancel support

`order_locations: HashMap<OrderId, OrderLocator>` maps an order ID to side, price,
and slot.

Why:

- avoids scanning the entire book to find an order
- gives direct access to the correct price level

Weak spot:

- the implementation now pays more bookkeeping cost per level mutation than a raw queue
- whether that tradeoff is worth it depends on the full-engine benchmarks, not just microbenchmarks

## Current complexity

Approximate costs:

- best bid / best ask lookup: `O(1)` over the tree edge
- insert resting order into price map: `O(log L)`
- enqueue inside level: `O(1)`
- match one maker from a level head: `O(1)`
- cross many levels: `O(number of consumed makers + crossed levels * log L)`
- cancel lookup by ID: `O(1)` average for the hash map
- cancel inside one level: `O(1)` by stored slot

That is why the benchmark suite now separates:

- populated-book cancels across multiple price levels
- deep single-price-level cancels
- checked vs unchecked invariant modes
- rich vs minimal result paths

## Where allocations happen

The engine is not allocation-free.

Current allocation sites include:

- `SubmissionResult.events` vector
- `CancelResult.events` vector
- `BookSnapshot` vectors
- `TopOfBook` / `BookLevelView` result objects
- `Trade` event creation per fill
- `MarketId` cloning into `Trade`

Important distinction:

- book mutation itself is mostly pointer/container work
- result construction and observability objects add extra allocation pressure

This is a key learning point: ergonomic result APIs often cost more than the bare matching loop.

## Current clone/copy behavior

Cheap copies:

- `Price`, `Quantity`, `OrderId`, `SequenceNumber`, and `Side` are small copyable values

More expensive owned moves/clones:

- `MarketId` inside `Trade`
- result/event vectors
- snapshots for read APIs

Recent improvement:

- the matcher now reads best prices directly from the `BTreeMap` keys instead of building `BookLevelView` values in the hot loop
- the crate now exposes lean `submit_order_minimal` / `cancel_order_minimal` paths so you can measure matching and cancel behavior without the richer result assembly path
- the book now supports an explicit `InvariantPolicy`, so you can measure mutation cost with and without full post-mutation invariant walks

## What is performance-friendly already

- explicit best-level matching loops
- slot-indexed FIFO behavior
- direct cancel index by `OrderId`
- no hidden async/runtime cost in the engine
- exact arithmetic without float normalization surprises

## What is intentionally simple rather than fast

- `BTreeMap` instead of a denser price-indexed structure
- storing full `Order` objects in per-level linked slots
- event-rich submission/cancel result objects on the same path as matching
- invariant checking after each mutation
- `BTreeMap` price discovery instead of a tighter market-specific price index

These choices are good for learning and correctness, but they would be revisited in a lower-latency design.

One important negative result:

- replacing the original queue with a per-level hash-linked structure improved the unchecked deep-cancel microbenchmark, but regressed the default checked engine broadly

That is a useful lesson by itself. More bookkeeping is not automatically more performance. The current engine now uses a denser slot-indexed design instead of the rejected hash-heavy variant.

To support that next step without destabilizing the engine, the repository now
includes an isolated prototype benchmark in
[`benches/price_level_prototypes.rs`](/Users/joeyalvarado/Developer/solbook-core/benches/price_level_prototypes.rs)
that compares:

- a simple `VecDeque` level
- a slab-style linked level with a hash index
- a denser indexed linked prototype

across:

- enqueue-heavy traffic
- FIFO sweep traffic
- deep cancel traffic

## Bottlenecks you should expect under load

If order volume rises, likely pressure points are:

- per-order event/result allocation
- invariant checks on every successful mutation
- per-level bookkeeping for slot maintenance during cancels and partial fills
- `MarketId` cloning for every emitted trade
- full snapshot construction for frequent read consumers

## Invariant policy

The default engine policy is `InvariantPolicy::Local`.

That means successful mutations do this:

1. mutate the book
2. verify the touched levels and index updates
3. return the result

This keeps correctness checks on the hot path without rescanning the entire book
every time.

The crate also supports:

- `InvariantPolicy::Full` for whole-book verification after every mutation
- `InvariantPolicy::Never` for engine study and benchmarking

Configure either through:

- `OrderBook::with_invariant_policy(...)`
- `OrderBook::set_invariant_policy(...)`

These policies do not change matching rules. They only change how much
post-mutation verification work runs.

## Upgrade paths

Reasonable future evolutions:

- separate a lean internal matching core from richer event/result assembly
- gate invariant checks behind debug/test builds
- replace per-level linear cancel removal with intrusive indices or slab-backed storage
- store market identity more cheaply in fills, or remove repeated trade-owned copies
- experiment with denser price-level indexing if the market has a bounded tick grid
- use arena/slab storage for orders if stable object locations become important

The important lesson is not “optimize everything now.” It is to understand where the simple design stops scaling and why.
