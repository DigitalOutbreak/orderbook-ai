import * as RechartsPrimitive from "recharts"

import { Badge } from "@/components/ui/badge"
import { ChartContainer, type ChartConfig } from "@/components/ui/chart"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"

import { formatPrice, formatSize } from "./mock-orderbook-data"
import type { OrderbookLevel } from "./orderbook-types"

type OrderbookDepthViewProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  showTotals: boolean
  compact: boolean
  heat: boolean
  midPrice: number
  condensed?: boolean
}

const bidChartConfig = {
  total: {
    label: "Bid depth",
    color: "rgb(20 184 166)",
  },
} satisfies ChartConfig

const askChartConfig = {
  total: {
    label: "Ask depth",
    color: "rgb(251 113 133)",
  },
} satisfies ChartConfig

export function OrderbookDepthView({
  bids,
  asks,
  showTotals,
  compact,
  heat,
  midPrice,
  condensed = false,
}: OrderbookDepthViewProps) {
  const bidOuterTotal = bids[bids.length - 1]?.total ?? 0
  const askOuterTotal = asks[asks.length - 1]?.total ?? 0

  const bidDepthData = [
    { depth: 0, total: 0 },
    ...bids.map((level, index) => ({
      depth: index + 1,
      total: level.total,
    })),
    { depth: bids.length + 1, total: bidOuterTotal },
  ]

  const askDepthData = [
    { depth: 0, total: 0 },
    ...asks.map((level, index) => ({
      depth: index + 1,
      total: level.total,
    })),
    { depth: asks.length + 1, total: askOuterTotal },
  ]

  const maxDepthTotal = Math.max(
    bidDepthData[bidDepthData.length - 1]?.total ?? 0,
    askDepthData[askDepthData.length - 1]?.total ?? 0,
    1
  )
  const maxBidMetric = Math.max(
    ...bids.map((level) => (showTotals ? level.total : level.size)),
    1
  )
  const maxAskMetric = Math.max(
    ...asks.map((level) => (showTotals ? level.total : level.size)),
    1
  )
  const rowPadding = compact ? "py-1.5" : "py-2"
  const seamWidth = condensed ? 112 : 152
  const seamPriceClass = condensed ? "text-[11px]" : "text-[13px]"
  const lanePaddingClass = condensed ? "px-1.5" : "px-2"

  return (
    <div className="flex h-full min-h-0 flex-col gap-1">
      <div className="flex items-center justify-between px-1">
        <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
          Depth
        </span>
        <Badge variant="outline" className="px-1.5 font-mono text-[10px]">
          {formatPrice(midPrice)}
        </Badge>
      </div>
      <div className="flex min-h-0 flex-1 flex-col">
        <div className="overflow-hidden border-b border-border/60 bg-transparent">
          <div className="relative grid grid-cols-[minmax(0,1fr)_minmax(0,1fr)] items-stretch gap-0">
            <ChartContainer
              config={bidChartConfig}
              className="aspect-auto h-[190px] w-full"
            >
              <RechartsPrimitive.AreaChart
                data={bidDepthData}
                margin={{ top: 18, right: 0, bottom: 18, left: 6 }}
              >
                <defs>
                  <linearGradient
                    id="depth-bid-fill"
                    x1="0"
                    y1="0"
                    x2="0"
                    y2="1"
                  >
                    <stop
                      offset="0%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.18"
                    />
                    <stop
                      offset="65%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.1"
                    />
                    <stop
                      offset="100%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.02"
                    />
                  </linearGradient>
                </defs>
                <RechartsPrimitive.CartesianGrid
                  vertical={false}
                  strokeDasharray="3 3"
                />
                <RechartsPrimitive.XAxis hide dataKey="depth" reversed />
                <RechartsPrimitive.YAxis hide domain={[0, maxDepthTotal]} />
                <RechartsPrimitive.Area
                  type="stepAfter"
                  dataKey="total"
                  stroke="var(--color-total)"
                  fill="url(#depth-bid-fill)"
                  strokeWidth={3}
                  isAnimationActive={false}
                />
              </RechartsPrimitive.AreaChart>
            </ChartContainer>

            <ChartContainer
              config={askChartConfig}
              className="aspect-auto h-[190px] w-full"
            >
              <RechartsPrimitive.AreaChart
                data={askDepthData}
                margin={{ top: 18, right: 6, bottom: 18, left: 0 }}
              >
                <defs>
                  <linearGradient
                    id="depth-ask-fill"
                    x1="0"
                    y1="0"
                    x2="0"
                    y2="1"
                  >
                    <stop
                      offset="0%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.18"
                    />
                    <stop
                      offset="65%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.1"
                    />
                    <stop
                      offset="100%"
                      stopColor="var(--color-total)"
                      stopOpacity="0.02"
                    />
                  </linearGradient>
                </defs>
                <RechartsPrimitive.CartesianGrid
                  vertical={false}
                  strokeDasharray="3 3"
                />
                <RechartsPrimitive.XAxis hide dataKey="depth" />
                <RechartsPrimitive.YAxis hide domain={[0, maxDepthTotal]} />
                <RechartsPrimitive.Area
                  type="stepAfter"
                  dataKey="total"
                  stroke="var(--color-total)"
                  fill="url(#depth-ask-fill)"
                  strokeWidth={3}
                  isAnimationActive={false}
                />
              </RechartsPrimitive.AreaChart>
            </ChartContainer>
          </div>
        </div>

        <div className="flex min-h-0 flex-1 flex-col gap-1 pt-1">
          <div className="grid grid-cols-[1fr_auto_1fr] items-center gap-2 px-3 pb-1 font-mono text-[10px] tracking-[0.18em] text-muted-foreground/85 uppercase">
            <span className="text-left">
              Low {formatPrice(bids[bids.length - 1]?.price ?? 0)}
            </span>
            <div
              className="px-2 py-0.5 text-center"
              style={{ width: seamWidth }}
            >
              Mid {formatPrice(midPrice)}
            </div>
            <span className="text-right">
              High {formatPrice(asks[asks.length - 1]?.price ?? 0)}
            </span>
          </div>
          <div className="grid grid-cols-[1fr_auto_1fr] items-center gap-2 px-3 pb-2 font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
            <span className="text-left">Size</span>
            <div
              className="grid grid-cols-2 px-2 py-0.5 text-center"
              style={{ width: seamWidth }}
            >
              <span className="text-emerald-300">Bid</span>
              <span className="text-rose-300">Ask</span>
            </div>
            <span className="text-right">Size</span>
          </div>
          <ScrollArea className="min-h-0 flex-1">
            <div className="flex flex-col gap-1 pr-3">
              {bids.map((bid, index) => {
                const ask = asks[index]
                const bidMetric = showTotals ? bid.total : bid.size
                const askMetric = ask ? (showTotals ? ask.total : ask.size) : 0
                const bidWidth = (bidMetric / maxBidMetric) * 100
                const askWidth = ask ? (askMetric / maxAskMetric) * 100 : 0
                const sideWidth = "50%"

                return (
                  <div
                    key={`depth-row-${index}`}
                    className="relative grid grid-cols-[1fr_auto_1fr] items-center gap-2 overflow-hidden bg-card/20 px-3"
                  >
                    {heat ? (
                      <>
                        <div
                          className="absolute inset-y-0 left-0 overflow-hidden"
                          style={{ width: sideWidth }}
                        >
                          <div
                            className="absolute inset-y-0 right-0 bg-[linear-gradient(90deg,rgba(52,211,153,0.02),rgba(52,211,153,0.16))]"
                            style={{ width: `${Math.max(bidWidth, 12)}%` }}
                          />
                        </div>
                        <div
                          className="absolute inset-y-0 right-0 overflow-hidden"
                          style={{ width: sideWidth }}
                        >
                          <div
                            className="absolute inset-y-0 left-0 bg-[linear-gradient(270deg,rgba(251,113,133,0.02),rgba(251,113,133,0.16))]"
                            style={{ width: `${Math.max(askWidth, 12)}%` }}
                          />
                        </div>
                      </>
                    ) : null}

                    <span
                      className={cn(
                        "relative font-mono text-[13px] text-foreground/90",
                        rowPadding
                      )}
                    >
                      {formatSize(bidMetric)}
                    </span>
                    <div
                      className={cn(
                        "relative text-center font-mono font-semibold",
                        rowPadding
                      )}
                      style={{ width: seamWidth }}
                    >
                      <div className="grid grid-cols-2 bg-transparent">
                        <div
                          className={cn(
                            "py-0.5 text-center text-emerald-300",
                            seamPriceClass,
                            lanePaddingClass
                          )}
                        >
                          {formatPrice(bid.price)}
                        </div>
                        <div
                          className={cn(
                            "py-0.5 text-center text-rose-300",
                            seamPriceClass,
                            lanePaddingClass
                          )}
                        >
                          {formatPrice(ask?.price ?? 0)}
                        </div>
                      </div>
                    </div>
                    <span
                      className={cn(
                        "relative text-right font-mono text-[13px] text-foreground/90",
                        rowPadding
                      )}
                    >
                      {ask ? formatSize(askMetric) : "-"}
                    </span>
                  </div>
                )
              })}
            </div>
          </ScrollArea>
        </div>
      </div>
    </div>
  )
}
