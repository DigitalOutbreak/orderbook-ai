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
  createChart,
  type IChartApi,
  type ISeriesApi,
  type SeriesType,
} from "lightweight-charts"

import type { MarketChartType, MarketTimeframe } from "@/lib/mock-chart-data"
import { getMockMarketDataset } from "@/lib/mock-chart-data"
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

export function MarketChartPanel({
  assetPair = "SOL/USDC",
  className,
}: MarketChartPanelProps) {
  const chartHostRef = React.useRef<HTMLDivElement | null>(null)
  const chartRef = React.useRef<IChartApi | null>(null)
  const seriesRef = React.useRef<ISeriesApi<SeriesType> | null>(null)
  const chartTypeRef = React.useRef<MarketChartType>("candles")

  const [timeframe, setTimeframe] = React.useState<MarketTimeframe>("15m")
  const [chartType, setChartType] = React.useState<MarketChartType>("candles")
  const [measureAnchor, setMeasureAnchor] =
    React.useState<MeasureAnchor | null>(null)
  const [measurePrice, setMeasurePrice] = React.useState<number | null>(null)
  const [hoverPoint, setHoverPoint] = React.useState<HoverPoint | null>(null)

  chartTypeRef.current = chartType

  const dataset = React.useMemo(
    () => getMockMarketDataset(timeframe),
    [timeframe]
  )

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

    return () => {
      chart.unsubscribeCrosshairMove(handleCrosshairMove)
      chart.unsubscribeClick(handleClick)
      resizeObserver.disconnect()
      chart.remove()
      chartRef.current = null
      seriesRef.current = null
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

      candleSeries.setData(dataset.candles)
      candleSeries.createPriceLine({
        price: dataset.price,
        color: dataset.changePercent >= 0 ? "#34d399" : "#fb7185",
        lineWidth: 1,
        axisLabelVisible: true,
        title: "Last",
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

      lineSeries.setData(dataset.line)
      lineSeries.createPriceLine({
        price: dataset.price,
        color: dataset.changePercent >= 0 ? "#5eead4" : "#fb7185",
        lineWidth: 1,
        axisLabelVisible: true,
        title: "Last",
      })
      seriesRef.current = lineSeries
    }

    chart.timeScale().fitContent()
  }, [chartType, dataset, timeframe])

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
    </Card>
  )
}
