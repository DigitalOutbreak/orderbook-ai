"use client"

import * as React from "react"
import {
  type BarData,
  CandlestickSeries,
  ColorType,
  CrosshairMode,
  LineSeries,
  type LineData,
  type MouseEventParams,
  type UTCTimestamp,
  createChart,
  type IChartApi,
  type IPriceLine,
  type IPriceScaleApi,
  type ISeriesApi,
  type SeriesType,
} from "lightweight-charts"

import type {
  MarketCandle,
  MarketChartType,
  MarketTimeframe,
} from "@/lib/mock-chart-data"
import {
  getMockMarketDataset,
  getTimeframeSeconds,
} from "@/lib/mock-chart-data"
import { cn } from "@/lib/utils"
import {
  CardAction,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"

import { MarketChartToolbar } from "./market-chart-toolbar"

type MarketChartPanelProps = {
  assetPair?: string
  className?: string
  footer?: React.ReactNode
  livePrice?: number | null
}

type MeasureAnchor = {
  price: number
  timeLabel: string
}

type HoverPoint = {
  price: number
  timeLabel: string
}

function getSeriesPrice(
  value: BarData | LineData | undefined,
  type: MarketChartType
) {
  if (!value) return null
  return type === "candles"
    ? "close" in value
      ? value.close
      : null
    : "value" in value
      ? value.value
      : null
}

function zoomVisiblePriceRange(
  priceScale: IPriceScaleApi,
  wheelDeltaY: number
) {
  const currentRange = priceScale.getVisibleRange()
  if (!currentRange) return

  const center = (currentRange.from + currentRange.to) / 2
  const span = currentRange.to - currentRange.from
  if (!Number.isFinite(span) || span <= 0) return

  const zoomFactor = wheelDeltaY < 0 ? 0.88 : 1.14
  const nextSpan = Math.max(span * zoomFactor, 0.01)

  priceScale.setAutoScale(false)
  priceScale.setVisibleRange({
    from: center - nextSpan / 2,
    to: center + nextSpan / 2,
  })
}

export function MarketChartPanel({
  assetPair = "SOL/USDC",
  className,
  footer,
  livePrice,
}: MarketChartPanelProps) {
  const chartHostRef = React.useRef<HTMLDivElement | null>(null)
  const chartRef = React.useRef<IChartApi | null>(null)
  const seriesRef = React.useRef<ISeriesApi<SeriesType> | null>(null)
  const priceLineRef = React.useRef<IPriceLine | null>(null)
  const chartTypeRef = React.useRef<MarketChartType>("candles")

  const [timeframe, setTimeframe] = React.useState<MarketTimeframe>("15m")
  const [chartType, setChartType] = React.useState<MarketChartType>("candles")
  const [measureAnchor, setMeasureAnchor] =
    React.useState<MeasureAnchor | null>(null)
  const [measurePrice, setMeasurePrice] = React.useState<number | null>(null)
  const [hoverPoint, setHoverPoint] = React.useState<HoverPoint | null>(null)
  const [displayPrice, setDisplayPrice] = React.useState<number>(171.8)

  chartTypeRef.current = chartType

  const baseDataset = React.useMemo(
    () => getMockMarketDataset(timeframe),
    [timeframe]
  )
  const [liveCandles, setLiveCandles] = React.useState<MarketCandle[]>(() =>
    baseDataset.candles.slice(-220)
  )

  React.useEffect(() => {
    setLiveCandles(baseDataset.candles.slice(-220))
    setDisplayPrice(baseDataset.price)
  }, [baseDataset])

  React.useEffect(() => {
    if (livePrice == null || !Number.isFinite(livePrice) || livePrice <= 0) {
      return
    }

    setDisplayPrice(Number(livePrice.toFixed(2)))

    setLiveCandles((current) => {
      if (current.length === 0) return current

      const next = [...current]
      const bucketSeconds = getTimeframeSeconds(timeframe)
      const now = Math.floor(Date.now() / 1000)
      const bucketTime = Math.floor(now / bucketSeconds) * bucketSeconds
      const roundedPrice = Number(livePrice.toFixed(2))
      const last = next[next.length - 1]

      if (last && Number(last.time) === bucketTime) {
        next[next.length - 1] = {
          ...last,
          high: Math.max(last.high, roundedPrice),
          low: Math.min(last.low, roundedPrice),
          close: roundedPrice,
          volume: Number((last.volume + 12 + Math.random() * 18).toFixed(2)),
        }
      } else {
        const open = last?.close ?? roundedPrice
        next.push({
          time: bucketTime as UTCTimestamp,
          open,
          high: Math.max(open, roundedPrice),
          low: Math.min(open, roundedPrice),
          close: roundedPrice,
          volume: Number((48 + Math.random() * 36).toFixed(2)),
        })
      }

      return next.slice(-220)
    })
  }, [livePrice, timeframe])

  const dataset = React.useMemo(() => {
    const candles = liveCandles.length > 0 ? liveCandles : baseDataset.candles
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
      price: displayPrice > 0 ? displayPrice : lastClose,
      changePercent: Number(changePercent.toFixed(2)),
      high: Number(
        Math.max(...candles.map((candle) => candle.high)).toFixed(2)
      ),
      low: Number(Math.min(...candles.map((candle) => candle.low)).toFixed(2)),
      volume: Number(
        candles.reduce((sum, candle) => sum + candle.volume, 0).toFixed(2)
      ),
    }
  }, [baseDataset.candles, displayPrice, liveCandles, timeframe])

  React.useEffect(() => {
    const container = chartHostRef.current
    if (!container) return

    const chart = createChart(container, {
      width: container.clientWidth,
      height: container.clientHeight,
      layout: {
        background: { type: ColorType.Solid, color: "transparent" },
        textColor: "rgba(232, 234, 245, 0.68)",
        attributionLogo: false,
      },
      grid: {
        vertLines: { color: "rgba(255,255,255,0.04)" },
        horzLines: { color: "rgba(255,255,255,0.04)" },
      },
      crosshair: {
        mode: CrosshairMode.Normal,
        vertLine: {
          color: "rgba(255,255,255,0.18)",
          width: 1,
          labelBackgroundColor: "rgba(20,24,34,0.96)",
        },
        horzLine: {
          color: "rgba(255,255,255,0.12)",
          width: 1,
          labelBackgroundColor: "rgba(20,24,34,0.96)",
        },
      },
      rightPriceScale: {
        borderColor: "rgba(255,255,255,0.08)",
        scaleMargins: { top: 0.12, bottom: 0.08 },
      },
      timeScale: {
        borderColor: "rgba(255,255,255,0.08)",
        rightOffset: 4,
        barSpacing: 8,
        minBarSpacing: 4,
        timeVisible: timeframe !== "1d",
        secondsVisible: false,
      },
      localization: {
        priceFormatter: (value: number) => value.toFixed(2),
      },
      handleScroll: {
        mouseWheel: true,
        pressedMouseMove: true,
        horzTouchDrag: false,
        vertTouchDrag: false,
      },
      handleScale: {
        axisPressedMouseMove: true,
        mouseWheel: true,
        pinch: false,
      },
    })

    chartRef.current = chart

    const handleCrosshairMove = (param: MouseEventParams) => {
      const series = seriesRef.current
      if (!series) return
      const price = getSeriesPrice(
        param.seriesData.get(series) as BarData | LineData | undefined,
        chartTypeRef.current
      )
      if (price == null) {
        setHoverPoint(null)
        if (measureAnchor) setMeasurePrice(null)
        return
      }

      const timeLabel =
        typeof param.time === "number"
          ? new Date(param.time * 1000).toLocaleString([], {
              month: "short",
              day: "numeric",
              hour: "2-digit",
              minute: "2-digit",
            })
          : "Cursor"

      setHoverPoint({ price, timeLabel })
      if (measureAnchor) setMeasurePrice(price)
    }

    const handleClick = (param: MouseEventParams) => {
      if (!param.sourceEvent?.shiftKey || !hoverPoint) return

      setMeasureAnchor({
        price: hoverPoint.price,
        timeLabel: hoverPoint.timeLabel,
      })
      setMeasurePrice(hoverPoint.price)
    }

    chart.subscribeCrosshairMove(handleCrosshairMove)
    chart.subscribeClick(handleClick)

    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries[0]
      if (!entry) return
      const { width, height } = entry.contentRect
      chart.applyOptions({ width, height })
      chart.timeScale().fitContent()
    })

    resizeObserver.observe(container)

    const handleWheel = (event: WheelEvent) => {
      const chart = chartRef.current
      if (!chart || !container) return

      const bounds = container.getBoundingClientRect()
      const axisHotzone = 72
      const pointerInsidePriceAxis = event.clientX >= bounds.right - axisHotzone

      if (!pointerInsidePriceAxis) return

      event.preventDefault()
      zoomVisiblePriceRange(chart.priceScale("right"), event.deltaY)
    }

    container.addEventListener("wheel", handleWheel, { passive: false })

    return () => {
      chart.unsubscribeCrosshairMove(handleCrosshairMove)
      chart.unsubscribeClick(handleClick)
      resizeObserver.disconnect()
      container.removeEventListener("wheel", handleWheel)
      chart.remove()
      chartRef.current = null
      seriesRef.current = null
      priceLineRef.current = null
    }
  }, [])

  React.useEffect(() => {
    setMeasureAnchor(null)
    setMeasurePrice(null)
    setHoverPoint(null)
  }, [timeframe, chartType])

  React.useEffect(() => {
    const chart = chartRef.current
    if (!chart) return

    chart.applyOptions({
      timeScale: {
        borderColor: "rgba(255,255,255,0.08)",
        rightOffset: 4,
        barSpacing: chartType === "candles" ? 8 : 6,
        minBarSpacing: 4,
        timeVisible: timeframe !== "1d",
        secondsVisible: false,
      },
    })

    if (seriesRef.current) {
      chart.removeSeries(seriesRef.current)
      seriesRef.current = null
      priceLineRef.current = null
    }

    if (chartType === "candles") {
      const candleSeries = chart.addSeries(CandlestickSeries, {
        upColor: "#34d399",
        downColor: "#fb7185",
        borderUpColor: "#34d399",
        borderDownColor: "#fb7185",
        wickUpColor: "#6ee7b7",
        wickDownColor: "#fda4af",
        priceLineVisible: false,
        lastValueVisible: true,
      })

      seriesRef.current = candleSeries
    } else {
      const lineSeries = chart.addSeries(LineSeries, {
        color: dataset.changePercent >= 0 ? "#5eead4" : "#fb7185",
        lineWidth: 2,
        priceLineVisible: false,
        lastValueVisible: true,
        crosshairMarkerVisible: true,
      })

      seriesRef.current = lineSeries
    }

    chart.timeScale().fitContent()
  }, [chartType, timeframe])

  React.useEffect(() => {
    const series = seriesRef.current
    if (!series) return

    if (chartType === "candles") {
      ;(series as ISeriesApi<"Candlestick">).setData(dataset.candles)
    } else {
      ;(series as ISeriesApi<"Line">).setData(dataset.line)
    }

    if (priceLineRef.current) {
      series.removePriceLine(priceLineRef.current)
      priceLineRef.current = null
    }

    priceLineRef.current = series.createPriceLine({
      price: dataset.price,
      color: dataset.changePercent >= 0 ? "#34d399" : "#fb7185",
      lineWidth: 1,
      axisLabelVisible: true,
      title: "Last",
    })
  }, [chartType, dataset])

  return (
    <Card
      size="sm"
      className={cn(
        "border-border/60 bg-card/80 shadow-2xl shadow-black/15",
        className
      )}
    >
      <CardHeader className="gap-3 border-b border-border/60 pb-3">
        <CardTitle className="sr-only">Market Chart</CardTitle>
        {measureAnchor ? (
          <CardAction>
            <Button
              type="button"
              variant="ghost"
              size="xs"
              onClick={() => {
                setMeasureAnchor(null)
                setMeasurePrice(null)
              }}
            >
              Clear Measure
            </Button>
          </CardAction>
        ) : null}
        <MarketChartToolbar
          assetPair={assetPair}
          price={dataset.price}
          changePercent={dataset.changePercent}
          timeframe={timeframe}
          onTimeframeChange={setTimeframe}
          chartType={chartType}
          onChartTypeChange={setChartType}
        />
      </CardHeader>

      <CardContent className="flex min-h-0 flex-1 flex-col pt-3">
        <div className="grid grid-cols-[auto_1fr_auto_auto] items-center gap-3 px-1 pb-2 font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
          <span>{dataset.low.toFixed(2)}</span>
          <span className="text-center">Range</span>
          <span>{dataset.high.toFixed(2)}</span>
          <span className="text-right">{dataset.volume.toFixed(0)} vol</span>
        </div>
        <Separator className="mb-3" />
        <div className="relative min-h-0 flex-1">
          {measureAnchor && measurePrice != null ? (
            <div className="pointer-events-none absolute top-2 left-2 z-10 flex items-center gap-2">
              <Badge variant="outline" className="font-mono text-[10px]">
                {measureAnchor.timeLabel}
              </Badge>
              <Badge
                variant="outline"
                className={cn(
                  "font-mono text-[10px]",
                  measurePrice >= measureAnchor.price
                    ? "text-emerald-300"
                    : "text-rose-300"
                )}
              >
                {(
                  ((measurePrice - measureAnchor.price) / measureAnchor.price) *
                  100
                ).toFixed(2)}
                %
              </Badge>
            </div>
          ) : (
            <div className="pointer-events-none absolute top-2 left-2 z-10">
              <Badge variant="outline" className="font-mono text-[10px]">
                Shift + click to measure
              </Badge>
            </div>
          )}
          <div
            ref={chartHostRef}
            className="h-full min-h-0 bg-[linear-gradient(180deg,rgba(255,255,255,0.01),transparent_18%)]"
          />
        </div>
      </CardContent>
      {footer ? (
        <div className="border-t border-border/60 px-3 py-2.5">{footer}</div>
      ) : null}
    </Card>
  )
}
