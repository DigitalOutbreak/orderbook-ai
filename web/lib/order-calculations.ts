import {
  buildLevelsFromEntries,
  buildOrderbookSnapshot,
  formatUtcTimestamp,
} from "@/components/orderbook/mock-orderbook-data"
import type {
  OrderbookEvent,
  OrderbookLevel,
  OrderbookSide,
  OrderbookSnapshot,
  OrderbookTrade,
} from "@/components/orderbook/orderbook-types"

export type OrderFormSide = "buy" | "sell"
export type OrderFormField = "price" | "quantity" | "total"

export type ParsedOrderValues = {
  price: number | null
  quantity: number | null
  total: number | null
}

export function sanitizeDecimalInput(value: string) {
  const cleaned = value.replace(/[^0-9.]/g, "")
  const [whole, ...rest] = cleaned.split(".")
  if (rest.length === 0) return cleaned
  return `${whole}.${rest.join("")}`
}

export function parseDecimal(value: string) {
  if (!value.trim()) return null
  const parsed = Number(value)
  if (!Number.isFinite(parsed) || parsed < 0) return null
  return parsed
}

export function formatEditableDecimal(value: number | null, precision = 4) {
  if (value == null || !Number.isFinite(value)) return ""
  const fixed = value.toFixed(precision)
  return fixed.replace(/\.?0+$/, "")
}

export function deriveOrderValues(
  next: ParsedOrderValues,
  lastEdited: OrderFormField
) {
  const { price, quantity, total } = next

  if (price == null || price <= 0) {
    return next
  }

  if (lastEdited === "quantity" && quantity != null) {
    return { price, quantity, total: Number((price * quantity).toFixed(4)) }
  }

  if (lastEdited === "total" && total != null) {
    return { price, quantity: Number((total / price).toFixed(4)), total }
  }

  if (lastEdited === "price") {
    if (quantity != null) {
      return { price, quantity, total: Number((price * quantity).toFixed(4)) }
    }
    if (total != null) {
      return { price, quantity: Number((total / price).toFixed(4)), total }
    }
  }

  return next
}

function levelEntries(levels: OrderbookLevel[]) {
  return levels.map((level) => ({
    id: level.id,
    price: level.price,
    size: level.size,
    side: level.side,
  }))
}

function mergeLimitLevel(
  levels: OrderbookLevel[],
  side: OrderbookSide,
  price: number,
  size: number
) {
  const roundedPrice = Number(price.toFixed(2))
  const roundedSize = Number(size.toFixed(2))
  const entries = levelEntries(levels)
  const existing = entries.find((entry) => entry.price === roundedPrice)

  if (existing) {
    existing.size = Number((existing.size + roundedSize).toFixed(2))
  } else {
    entries.push({
      id: `${side}-user-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
      price: roundedPrice,
      size: roundedSize,
      side,
    })
  }

  return buildLevelsFromEntries(entries, side)
}

function reduceTopLevel(
  levels: OrderbookLevel[],
  side: OrderbookSide,
  quantity: number
) {
  const entries = levelEntries(levels)
  const top = entries[0]
  if (!top) return levels

  top.size = Number(Math.max(top.size - quantity, 0).toFixed(2))
  const filtered = entries.filter((entry) => entry.size > 0)
  return buildLevelsFromEntries(filtered, side)
}

function formatTradeTime(date = new Date()) {
  const parts = new Intl.DateTimeFormat("en-US", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    fractionalSecondDigits: 3,
    hour12: false,
    timeZone: "UTC",
  }).formatToParts(date)

  const values = Object.fromEntries(
    parts.map((part) => [part.type, part.value])
  ) as Record<string, string>

  return `${values.hour}:${values.minute}:${values.second}.${values.fractionalSecond ?? "000"}`
}

function createMockTrade(
  side: OrderFormSide,
  price: number,
  size: number
): OrderbookTrade {
  return {
    id: `trade-user-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
    side: side === "buy" ? "buy" : "sell",
    price: Number(price.toFixed(2)),
    size: Number(size.toFixed(2)),
    time: formatTradeTime(),
    venue: "SIM",
  }
}

function createOrderEvent(input: {
  side: OrderFormSide
  price: number
  quantity: number
  crossed: boolean
  executedSize: number
  remainder: number
  executionPrice?: number
}): OrderbookEvent {
  const tone = input.side === "buy" ? "buy" : "sell"
  const sideLabel = input.side === "buy" ? "Buy" : "Sell"

  if (input.crossed) {
    const remainderText =
      input.remainder > 0
        ? `, ${input.remainder.toFixed(2)} rested at ${input.price.toFixed(2)}`
        : ""

    return {
      id: `event-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
      time: formatTradeTime(),
      tone,
      title: `${sideLabel} crossed the spread`,
      detail: `Executed ${input.executedSize.toFixed(2)} @ ${(input.executionPrice ?? input.price).toFixed(2)}${remainderText}.`,
    }
  }

  return {
    id: `event-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
    time: formatTradeTime(),
    tone,
    title: `${sideLabel} rested on the book`,
    detail: `Added ${input.quantity.toFixed(2)} @ ${input.price.toFixed(2)} to the ${input.side === "buy" ? "bid" : "ask"} side.`,
  }
}

export function applyMockLimitOrder(
  snapshot: OrderbookSnapshot,
  input: {
    side: OrderFormSide
    price: number
    quantity: number
  }
) {
  const bookSide: OrderbookSide = input.side === "buy" ? "bid" : "ask"
  const oppositeTop = bookSide === "bid" ? snapshot.asks[0] : snapshot.bids[0]
  const crossesSpread =
    oppositeTop != null &&
    (bookSide === "bid"
      ? input.price >= oppositeTop.price
      : input.price <= oppositeTop.price)

  let nextBids = snapshot.bids
  let nextAsks = snapshot.asks
  let nextTrades = snapshot.trades
  let executedSize = 0
  let remainder = input.quantity

  if (crossesSpread && oppositeTop) {
    executedSize = Math.min(input.quantity, oppositeTop.size)
    remainder = Number((input.quantity - executedSize).toFixed(2))

    nextTrades = [
      createMockTrade(input.side, oppositeTop.price, executedSize),
      ...snapshot.trades,
    ].slice(0, 20)

    if (bookSide === "bid") {
      nextAsks = reduceTopLevel(snapshot.asks, "ask", executedSize)
      nextBids =
        remainder > 0
          ? mergeLimitLevel(snapshot.bids, "bid", input.price, remainder)
          : snapshot.bids
    } else {
      nextBids = reduceTopLevel(snapshot.bids, "bid", executedSize)
      nextAsks =
        remainder > 0
          ? mergeLimitLevel(snapshot.asks, "ask", input.price, remainder)
          : snapshot.asks
    }
  } else {
    nextBids =
      bookSide === "bid"
        ? mergeLimitLevel(snapshot.bids, "bid", input.price, input.quantity)
        : snapshot.bids
    nextAsks =
      bookSide === "ask"
        ? mergeLimitLevel(snapshot.asks, "ask", input.price, input.quantity)
        : snapshot.asks
  }

  const nextSnapshot = buildOrderbookSnapshot(nextBids, nextAsks, nextTrades, {
    ...snapshot.stats,
    updatedAt: formatUtcTimestamp(),
  })

  return {
    snapshot: nextSnapshot,
    event: createOrderEvent({
      side: input.side,
      price: input.price,
      quantity: input.quantity,
      crossed: crossesSpread,
      executedSize,
      remainder,
      executionPrice: oppositeTop?.price,
    }),
  }
}
