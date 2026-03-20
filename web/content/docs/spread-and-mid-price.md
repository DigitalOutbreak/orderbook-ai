---
title: Spread and Mid Price
description: What spread and midpoint mean, and why they are useful reference values.
---

# Spread and Mid Price

Two of the most common reference values in an order book UI are the **spread** and the **mid price**.

## Spread

The spread is:

```text
best ask - best bid
```

Example:

| Best Bid | Best Ask | Spread |
| --- | --- | --- |
| 171.98 | 172.02 | 0.04 |

The spread gives a quick sense of how tight or wide the market is.

## Mid price

The mid price is the midpoint between best bid and best ask:

```text
(best bid + best ask) / 2
```

It is a reference value, not necessarily an executable trade price.

## Why the spread matters

The spread often reflects:

- liquidity conditions
- market maker activity
- short-term uncertainty
- how easy it is to cross the market without paying a large penalty

## Why the mid matters

The mid is useful for:

- centering a depth view
- comparing nearby price levels
- measuring relative movement around the touch

## UI note

The mid should usually be visible, but it should not dominate the ladder.

It is a reference line, not the main data surface.
