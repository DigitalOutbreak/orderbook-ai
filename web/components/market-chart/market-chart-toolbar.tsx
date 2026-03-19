"use client"

import { ChartBar, ChartLine, TrendUp } from "@phosphor-icons/react"

import type { MarketChartType, MarketTimeframe } from "@/lib/mock-chart-data"
import { MARKET_TIMEFRAMES } from "@/lib/mock-chart-data"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import { cn } from "@/lib/utils"

type MarketChartToolbarProps = {
  assetPair: string
  price: number
  changePercent: number
  timeframe: MarketTimeframe
  onTimeframeChange: (next: MarketTimeframe) => void
  chartType: MarketChartType
  onChartTypeChange: (next: MarketChartType) => void
}

function formatPrice(value: number) {
  return value.toFixed(2)
}

function formatChange(value: number) {
  return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`
}

export function MarketChartToolbar({
  assetPair,
  price,
  changePercent,
  timeframe,
  onTimeframeChange,
  chartType,
  onChartTypeChange,
}: MarketChartToolbarProps) {
  const positive = changePercent >= 0

  return (
    <div className="flex flex-wrap items-center justify-between gap-3">
      <div className="flex min-w-0 items-center gap-3">
        <div className="min-w-0">
          <div className="flex items-center gap-2">
            <span className="font-mono text-[11px] tracking-[0.24em] text-muted-foreground uppercase">
              Market
            </span>
            <Badge variant="secondary" className="px-1.5 font-mono text-[10px]">
              {assetPair}
            </Badge>
          </div>
          <div className="mt-1 flex items-center gap-3">
            <span className="font-mono text-lg font-semibold">
              {formatPrice(price)}
            </span>
            <span
              className={cn(
                "inline-flex items-center gap-1 font-mono text-xs",
                positive ? "text-emerald-300" : "text-rose-300"
              )}
            >
              <TrendUp
                className={cn(!positive && "rotate-180")}
                data-icon="inline-start"
              />
              {formatChange(changePercent)}
            </span>
          </div>
        </div>
      </div>

      <div className="flex flex-wrap items-center justify-end gap-2">
        <ToggleGroup
          type="single"
          value={timeframe}
          onValueChange={(value) => {
            if (value) onTimeframeChange(value as MarketTimeframe)
          }}
          variant="outline"
          size="sm"
        >
          {MARKET_TIMEFRAMES.map((item) => (
            <ToggleGroupItem key={item} value={item} aria-label={`Set ${item}`}>
              {item}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>

        <Separator orientation="vertical" className="hidden h-6 lg:block" />

        <ToggleGroup
          type="single"
          value={chartType}
          onValueChange={(value) => {
            if (value) onChartTypeChange(value as MarketChartType)
          }}
          variant="outline"
          size="sm"
        >
          <ToggleGroupItem value="candles" aria-label="Candlestick chart">
            <ChartBar data-icon="inline-start" />
            Candles
          </ToggleGroupItem>
          <ToggleGroupItem value="line" aria-label="Line chart">
            <ChartLine data-icon="inline-start" />
            Line
          </ToggleGroupItem>
        </ToggleGroup>
      </div>
    </div>
  )
}
