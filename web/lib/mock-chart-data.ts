import type { UTCTimestamp } from "lightweight-charts"

export const MARKET_TIMEFRAMES = ["1m", "5m", "15m", "1h", "4h", "1d"] as const

export type MarketTimeframe = (typeof MARKET_TIMEFRAMES)[number]
export type MarketChartType = "candles" | "line"

export type MarketCandle = {
  time: UTCTimestamp
  open: number
  high: number
  low: number
  close: number
  volume: number
}

export type MarketLinePoint = {
  time: UTCTimestamp
  value: number
}

export type MarketChartDataset = {
  timeframe: MarketTimeframe
  candles: MarketCandle[]
  line: MarketLinePoint[]
  price: number
  changePercent: number
  high: number
  low: number
  volume: number
}

const timeframeMinutes: Record<MarketTimeframe, number> = {
  "1m": 1,
  "5m": 5,
  "15m": 15,
  "1h": 60,
  "4h": 240,
  "1d": 1440,
}

export function getTimeframeSeconds(timeframe: MarketTimeframe) {
  return timeframeMinutes[timeframe] * 60
}

function round(value: number) {
  return Number(value.toFixed(2))
}

function buildBaseCandles(count = 720): MarketCandle[] {
  const candles: MarketCandle[] = []
  const start = Math.floor(Date.UTC(2026, 2, 18, 0, 0, 0) / 1000) - count * 60
  let previousClose = 171.8

  for (let index = 0; index < count; index += 1) {
    const time = (start + index * 60) as UTCTimestamp
    const drift = Math.sin(index / 18) * 0.22 + Math.cos(index / 33) * 0.12
    const micro = Math.sin(index / 3.8) * 0.05
    const bias = index > count * 0.58 ? 0.018 : -0.004
    const open = previousClose
    const close = round(open + drift * 0.18 + micro + bias)
    const high = round(Math.max(open, close) + 0.08 + (index % 5) * 0.015)
    const low = round(Math.min(open, close) - 0.08 - (index % 4) * 0.012)
    const volume = round(
      420 +
        (Math.sin(index / 9) + 1.3) * 140 +
        (index % 7) * 18 +
        (index > count * 0.72 ? 110 : 0)
    )

    candles.push({
      time,
      open: round(open),
      high,
      low,
      close,
      volume,
    })

    previousClose = close
  }

  return candles
}

function aggregateCandles(
  candles: MarketCandle[],
  timeframe: MarketTimeframe
): MarketCandle[] {
  const bucketSize = timeframeMinutes[timeframe]

  if (bucketSize === 1) {
    return candles
  }

  const aggregated: MarketCandle[] = []

  for (let index = 0; index < candles.length; index += bucketSize) {
    const bucket = candles.slice(index, index + bucketSize)
    if (bucket.length === 0) continue

    aggregated.push({
      time: bucket[0].time,
      open: bucket[0].open,
      high: round(Math.max(...bucket.map((item) => item.high))),
      low: round(Math.min(...bucket.map((item) => item.low))),
      close: bucket[bucket.length - 1].close,
      volume: round(bucket.reduce((sum, item) => sum + item.volume, 0)),
    })
  }

  return aggregated
}

function buildDataset(timeframe: MarketTimeframe, source: MarketCandle[]) {
  const candles = aggregateCandles(source, timeframe)
  const line = candles.map((candle) => ({
    time: candle.time,
    value: candle.close,
  }))
  const firstOpen = candles[0]?.open ?? 0
  const lastClose = candles[candles.length - 1]?.close ?? 0
  const changePercent = firstOpen
    ? ((lastClose - firstOpen) / firstOpen) * 100
    : 0

  return {
    timeframe,
    candles,
    line,
    price: lastClose,
    changePercent: Number(changePercent.toFixed(2)),
    high: round(Math.max(...candles.map((candle) => candle.high))),
    low: round(Math.min(...candles.map((candle) => candle.low))),
    volume: round(candles.reduce((sum, candle) => sum + candle.volume, 0)),
  } satisfies MarketChartDataset
}

const baseCandles = buildBaseCandles()

const datasetCache = Object.fromEntries(
  MARKET_TIMEFRAMES.map((timeframe) => [
    timeframe,
    buildDataset(timeframe, baseCandles),
  ])
) as Record<MarketTimeframe, MarketChartDataset>

export function getMockMarketDataset(timeframe: MarketTimeframe) {
  return datasetCache[timeframe]
}
