---
title: 07 Why This Design
---

# 07 Why This Design

This chapter exists for one reason:

to answer "why this design?" before your brain starts inventing five other
designs and pulling you off track.

That instinct is good. It means you are thinking like an engineer.
But early in learning, too many alternatives can make the current design feel
less solid than it really is.

So this chapter is here to anchor you.

## The core rule

The project does not try to be the most advanced matching engine possible.

It tries to be:

- correct
- deterministic
- understandable
- measurable
- good enough to teach real tradeoffs

That is why some structures are simpler than what a production HFT engine might
eventually use.

## Why `BTreeMap` for bids and asks?

Current choice:

- bids: `BTreeMap<Price, PriceLevel>`
- asks: `BTreeMap<Price, PriceLevel>`

Why this was chosen:

- prices stay sorted automatically
- best bid and best ask are easy to find
- the relationship between price priority and the data structure is easy to see
- it is deterministic and readable

Why not a dense price array yet?

- a dense array assumes a bounded and practical price grid
- it is faster only when the market structure makes that worthwhile
- it adds another layer of representation to understand early

Why not a heap?

- heaps are good at finding a best value
- they are worse at full ordered traversal and level management
- they also do not naturally model grouped price levels the way a map does

Beginner takeaway:

`BTreeMap` is not the fastest possible structure.
It is the clearest structure that still behaves like a real engine.

## Why `PriceLevel` instead of storing all orders in one giant list?

Current choice:

- orders are grouped by price first
- then FIFO is handled inside each price level

Why:

- price-time priority is easier to reason about in two layers:
  - best price first
  - then oldest order at that price

Why not one giant sorted structure of all orders?

- it hides the mental model
- it makes price levels less obvious
- it makes “best price, then FIFO within that price” harder to see directly

Beginner takeaway:

Grouping by price is not just an implementation detail.
It matches how the matching rule itself works.

## Why slot-indexed linked storage inside a price level?

Current choice:

- each price level keeps FIFO order
- each order can also be removed later by a known slot

Why:

- matching wants cheap pop-from-front behavior
- cancellation wants cheap removal of a known resting order
- the current design tries to satisfy both

Why not just use `VecDeque<Order>`?

- `VecDeque` is very clean for FIFO
- but cancellation inside one deep level becomes expensive if you have to search
- earlier versions showed that deep cancel was a real weakness

Why not use a hash-heavy linked design?

- we tried that direction in prototype form
- it improved one microcase
- it made broader engine behavior worse

Why this current design instead?

- it keeps the cancel win
- it stays more compact than the rejected hash-heavy variant
- it still makes FIFO visible

Beginner takeaway:

This is an example of a compromise structure:
not the simplest possible, not the fanciest possible, but one that exposes the
right tradeoff for learning.

## Why an `OrderLocator` index?

Current choice:

- `HashMap<OrderId, OrderLocator>`

Why:

- cancel needs a direct path from `OrderId` to the book location
- without it, you would have to scan the book

Why not just search all levels on cancel?

- because that teaches the wrong habit
- it is simple, but too wasteful for a serious engine

Why not store raw pointers everywhere?

- more dangerous
- harder to reason about
- easier to get wrong while learning

Beginner takeaway:

The locator index is a professional compromise:
fast enough to matter, simple enough to understand.

## Why keep matching logic separate from HTTP?

Why:

- matching rules should not depend on transport format
- tests should not need a server
- future wrappers should be replaceable

Why not just build everything inside the API layer?

- because then the engine becomes harder to test, reuse, and reason about
- transport code and matching code start contaminating each other

Beginner takeaway:

Separation is not “extra architecture.” It is what keeps the real logic clean.

## Why keep invariants at all?

Why:

- invariants catch impossible states
- they help you trust the engine while learning
- they make refactors safer

Why not disable them permanently for speed?

- because then you lose an important correctness tool
- and you stop learning where safety costs actually come from

Why not always do full-book invariant walks?

- because measurements showed that was too expensive on the hot path

Why `Local` by default?

- it keeps checks on
- it limits the work to touched structures
- it preserves the learning value without paying the full old cost

Beginner takeaway:

This is a good example of engineering compromise:
not maximum safety cost, not zero safety cost, but a measured middle ground.

## Why not optimize everything right now?

Because optimization without understanding tends to make systems worse.

This repo already proved that:

- one attempted optimization looked good in a narrow case
- benchmarks showed it hurt the engine more broadly
- the right move was to reject it

That is real engineering.

## The right way to use this chapter

When you catch yourself thinking:

- "Why not just use a vector?"
- "Why not use a heap?"
- "Why not use a database?"
- "Why not make it async?"

come back here first.

Most of the time, the answer is:

because the project is trying to teach the matching engine clearly before
chasing every advanced possibility.
