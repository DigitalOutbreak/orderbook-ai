export type OrderbookSide = "bid" | "ask"

export type OrderbookView = "split" | "stack" | "depth"

export type OrderbookDisplayOption = "heat" | "totals" | "compact" | "learn"

export type OrderbookEvent = {
  id: string
  time: string
  tone: "buy" | "sell" | "neutral"
  title: string
  detail: string
}

export type OrderbookLevel = {
  id: string
  side: OrderbookSide
  price: number
  size: number
  total: number
}

export type OrderbookTrade = {
  id: string
  side: "buy" | "sell"
  price: number
  size: number
  time: string
  venue: string
}

export type OrderbookStats = {
  symbol: string
  bestBid: number
  bestAsk: number
  spread: number
  midPrice: number
  updatedAt: string
  mode: "mock" | "live"
}

export type OrderbookSnapshot = {
  stats: OrderbookStats
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  trades: OrderbookTrade[]
}
