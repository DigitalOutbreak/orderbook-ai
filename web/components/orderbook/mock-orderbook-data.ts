import type {
  OrderbookLevel,
  OrderbookSnapshot,
  OrderbookStats,
  OrderbookTrade,
} from "./orderbook-types"

const bidSeed = [
  [171.98, 42.18],
  [171.95, 38.72],
  [171.92, 34.61],
  [171.9, 28.44],
  [171.87, 25.91],
  [171.84, 21.67],
  [171.81, 18.29],
  [171.78, 15.44],
  [171.75, 13.8],
  [171.72, 10.91],
  [171.68, 8.73],
  [171.64, 6.55],
] as const

const askSeed = [
  [172.02, 31.11],
  [172.05, 33.48],
  [172.08, 36.92],
  [172.1, 28.84],
  [172.13, 24.51],
  [172.16, 22.7],
  [172.19, 19.88],
  [172.22, 17.26],
  [172.25, 13.92],
  [172.28, 12.61],
  [172.32, 9.48],
  [172.36, 7.12],
] as const

const tradesSeed = [
  ["sell", 172.01, 3.21, "14:32:08.121", "SIM"],
  ["buy", 172.02, 1.8, "14:32:07.904", "SIM"],
  ["buy", 172.03, 6.44, "14:32:07.502", "SIM"],
  ["sell", 172, 2.36, "14:32:06.990", "SIM"],
  ["sell", 171.99, 4.8, "14:32:06.351", "SIM"],
  ["buy", 172.04, 1.27, "14:32:05.944", "SIM"],
  ["buy", 172.05, 7.11, "14:32:05.510", "SIM"],
  ["sell", 172.01, 5.63, "14:32:04.871", "SIM"],
  ["buy", 172.06, 2.05, "14:32:04.332", "SIM"],
  ["sell", 171.98, 3.66, "14:32:03.900", "SIM"],
] as const

function buildLevels(
  rows: readonly (readonly [number, number])[],
  side: "bid" | "ask"
): OrderbookLevel[] {
  return buildLevelsFromEntries(
    rows.map(([price, size], index) => ({
      id: `${side}-${index}`,
      price,
      size,
      side,
    })),
    side
  )
}

function buildTrades(): OrderbookTrade[] {
  return tradesSeed.map(([side, price, size, time, venue], index) => ({
    id: `trade-${index}`,
    side,
    price,
    size,
    time,
    venue,
  }))
}

export function buildLevelsFromEntries(
  rows: Array<{
    id: string
    price: number
    size: number
    side: "bid" | "ask"
  }>,
  side: "bid" | "ask"
): OrderbookLevel[] {
  const sorted = [...rows].sort((a, b) =>
    side === "bid" ? b.price - a.price : a.price - b.price
  )
  let cumulative = 0

  return sorted.map((row) => {
    cumulative += row.size

    return {
      id: row.id,
      side,
      price: row.price,
      size: Number(row.size.toFixed(2)),
      total: Number(cumulative.toFixed(2)),
    }
  })
}

export function buildOrderbookStats(
  bids: OrderbookLevel[],
  asks: OrderbookLevel[],
  overrides?: Partial<OrderbookStats>
): OrderbookStats {
  const bestBid = bids[0]?.price ?? 0
  const bestAsk = asks[0]?.price ?? 0
  const spread = Number((bestAsk - bestBid).toFixed(2))
  const midPrice = Number(((bestAsk + bestBid) / 2).toFixed(2))

  return {
    symbol: overrides?.symbol ?? "SOL / USDC",
    bestBid,
    bestAsk,
    spread,
    midPrice,
    updatedAt: overrides?.updatedAt ?? "14:32:08.121 UTC",
    mode: overrides?.mode ?? "mock",
  }
}

export function buildOrderbookSnapshot(
  bids: OrderbookLevel[],
  asks: OrderbookLevel[],
  trades: OrderbookTrade[],
  statsOverrides?: Partial<OrderbookStats>
): OrderbookSnapshot {
  return {
    stats: buildOrderbookStats(bids, asks, statsOverrides),
    bids,
    asks,
    trades,
  }
}

export function createMockOrderbookSnapshot(): OrderbookSnapshot {
  const bids = buildLevels(bidSeed, "bid")
  const asks = buildLevels(askSeed, "ask")
  const trades = buildTrades()

  return buildOrderbookSnapshot(bids, asks, trades, {
    updatedAt: "14:32:08.121 UTC",
  })
}

export const mockOrderbookSnapshot: OrderbookSnapshot =
  createMockOrderbookSnapshot()

export function formatPrice(value: number) {
  return value.toFixed(2)
}

export function formatSize(value: number) {
  return value.toFixed(2)
}

export function formatSideLabel(side: "buy" | "sell") {
  return side === "buy" ? "Buy" : "Sell"
}

export function getMaxSize(levels: OrderbookLevel[]) {
  return Math.max(...levels.map((level) => level.size), 1)
}

export function getMaxTotal(levels: OrderbookLevel[]) {
  return Math.max(...levels.map((level) => level.total), 1)
}

export function formatUtcTimestamp(date = new Date()) {
  const iso = date.toISOString().replace("T", " ").replace("Z", "")
  const [day, time] = iso.split(" ")
  return `${time} UTC`
}
