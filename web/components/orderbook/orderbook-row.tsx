import { cn } from "@/lib/utils"
import { formatPrice, formatSize } from "./mock-orderbook-data"
import type { OrderbookLevel } from "./orderbook-types"

type SplitRowProps = {
  layout: "split"
  bid?: OrderbookLevel
  ask?: OrderbookLevel
  showTotals: boolean
  compact: boolean
  heat: boolean
  maxBidMetric: number
  maxAskMetric: number
}

type StackRowProps = {
  layout: "stack"
  side: "bid" | "ask"
  level: OrderbookLevel
  showTotals: boolean
  compact: boolean
  heat: boolean
  maxMetric: number
}

type OrderbookRowProps = SplitRowProps | StackRowProps

function intensity(value: number, maxValue: number) {
  if (maxValue <= 0) return 0
  return Math.max(0.12, Math.min(value / maxValue, 1))
}

function HeatBar({ side, opacity }: { side: "bid" | "ask"; opacity: number }) {
  return (
    <div
      className={cn(
        "absolute inset-y-0",
        side === "bid" ? "left-0 bg-emerald-400" : "right-0 bg-rose-400"
      )}
      style={{ width: `${opacity * 100}%`, opacity: opacity * 0.22 }}
    />
  )
}

export function OrderbookRow(props: OrderbookRowProps) {
  const rowPadding = props.compact ? "py-1" : "py-2"

  if (props.layout === "stack") {
    const { side, level, showTotals, heat, maxMetric } = props
    const metric = showTotals ? level.total : level.size
    const fill = intensity(metric, maxMetric)
    const stackGrid = showTotals
      ? "grid-cols-[104px_minmax(0,1fr)_minmax(0,1fr)]"
      : "grid-cols-[104px_minmax(0,1fr)]"

    return (
      <div className="relative overflow-hidden border border-border/60 bg-background/70">
        {heat ? <HeatBar side={side} opacity={fill} /> : null}
        <div
          className={cn(
            "relative grid items-center gap-3 px-3 font-mono text-xs",
            stackGrid,
            rowPadding
          )}
        >
          <span
            className={cn(
              "text-right font-semibold tracking-tight",
              side === "bid" ? "text-emerald-300" : "text-rose-300"
            )}
          >
            {formatPrice(level.price)}
          </span>
          <span className="text-right text-foreground/90">
            {showTotals ? formatSize(level.total) : formatSize(level.size)}
          </span>
          {showTotals ? (
            <span className="text-right text-foreground/75">
              {formatSize(level.size)}
            </span>
          ) : null}
        </div>
      </div>
    )
  }

  const { bid, ask, showTotals, compact, heat, maxBidMetric, maxAskMetric } =
    props
  const padding = compact ? "py-1.5" : "py-2.5"
  const splitGrid = showTotals
    ? "grid-cols-[minmax(0,1fr)_minmax(0,1fr)_104px_minmax(0,1fr)_minmax(0,1fr)]"
    : "grid-cols-[minmax(0,1fr)_104px_minmax(0,1fr)]"

  const bidMetric = bid ? (showTotals ? bid.total : bid.size) : 0
  const askMetric = ask ? (showTotals ? ask.total : ask.size) : 0

  return (
    <div className="relative overflow-hidden border border-border/60 bg-background/70">
      {heat && bid ? (
        <div className="absolute inset-y-0 left-0 w-[calc(50%-52px)] overflow-hidden">
          <HeatBar side="bid" opacity={intensity(bidMetric, maxBidMetric)} />
        </div>
      ) : null}
      {heat && ask ? (
        <div className="absolute inset-y-0 right-0 w-[calc(50%-52px)] overflow-hidden">
          <HeatBar side="ask" opacity={intensity(askMetric, maxAskMetric)} />
        </div>
      ) : null}
      <div
        className={cn(
          "relative grid items-center gap-3 px-3 font-mono text-xs",
          splitGrid,
          padding
        )}
      >
        {showTotals ? (
          <>
            <span className="text-right text-foreground/90">
              {bid ? formatSize(bid.size) : " "}
            </span>
            <span className="text-right text-foreground/70">
              {bid ? formatSize(bid.total) : " "}
            </span>
          </>
        ) : (
          <span className="text-right text-foreground/90">
            {bid ? formatSize(bid.size) : " "}
          </span>
        )}

        <div className="flex items-center justify-center px-1">
          <span
            className={cn(
              "w-full text-center font-semibold tracking-tight",
              bid
                ? "text-emerald-300"
                : ask
                  ? "text-rose-300"
                  : "text-foreground"
            )}
          >
            {formatPrice(bid?.price ?? ask?.price ?? 0)}
          </span>
        </div>

        {showTotals ? (
          <>
            <span className="text-left text-foreground/70">
              {ask ? formatSize(ask.total) : " "}
            </span>
            <span className="text-left text-foreground/90">
              {ask ? formatSize(ask.size) : " "}
            </span>
          </>
        ) : (
          <span className="text-left text-foreground/90">
            {ask ? formatSize(ask.size) : " "}
          </span>
        )}
      </div>
    </div>
  )
}
