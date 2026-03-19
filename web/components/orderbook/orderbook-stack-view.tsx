import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { OrderbookRow } from "./orderbook-row"
import { formatPrice, getMaxSize, getMaxTotal } from "./mock-orderbook-data"
import type { OrderbookLevel } from "./orderbook-types"

type OrderbookStackViewProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  showTotals: boolean
  compact: boolean
  heat: boolean
  spread: number
  midPrice: number
}

export function OrderbookStackView({
  bids,
  asks,
  showTotals,
  compact,
  heat,
  spread,
  midPrice,
}: OrderbookStackViewProps) {
  const askMaxMetric = showTotals ? getMaxTotal(asks) : getMaxSize(asks)
  const bidMaxMetric = showTotals ? getMaxTotal(bids) : getMaxSize(bids)
  const stackGrid = showTotals
    ? "grid-cols-[104px_minmax(0,1fr)_minmax(0,1fr)]"
    : "grid-cols-[104px_minmax(0,1fr)]"

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="grid gap-2">
        <div className="flex items-center justify-between text-[10px] font-semibold tracking-[0.24em] uppercase">
          <span className="text-rose-300">Asks</span>
          <span className="text-muted-foreground">Top down</span>
        </div>
        <div
          className={`grid items-center gap-3 px-3 pr-6 font-mono text-[10px] tracking-[0.2em] text-muted-foreground/80 uppercase ${stackGrid}`}
        >
          <span className="text-right">Price</span>
          <span className="text-right">{showTotals ? "Total" : "Size"}</span>
          {showTotals ? <span className="text-right">Size</span> : null}
        </div>
      </div>
      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-1.5 pr-3">
          {asks
            .slice()
            .reverse()
            .map((level) => (
              <OrderbookRow
                key={level.id}
                layout="stack"
                side="ask"
                level={level}
                showTotals={showTotals}
                compact={compact}
                heat={heat}
                maxMetric={askMaxMetric}
              />
            ))}
        </div>
      </ScrollArea>

      <div className="flex items-center justify-between border-y border-border/60 px-3 py-1.5">
        <span className="font-mono text-sm font-semibold text-foreground">
          {formatPrice(midPrice)}
        </span>
        <Badge variant="outline" className="font-mono text-[10px]">
          Spread {formatPrice(spread)}
        </Badge>
      </div>

      <div className="grid gap-2">
        <div className="flex items-center justify-between text-[10px] font-semibold tracking-[0.24em] uppercase">
          <span className="text-emerald-300">Bids</span>
          <span className="text-muted-foreground">Bottom up</span>
        </div>
        <div
          className={`grid items-center gap-3 px-3 pr-6 font-mono text-[10px] tracking-[0.2em] text-muted-foreground/80 uppercase ${stackGrid}`}
        >
          <span className="text-right">Price</span>
          <span className="text-right">{showTotals ? "Total" : "Size"}</span>
          {showTotals ? <span className="text-right">Size</span> : null}
        </div>
      </div>
      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-1.5 pr-3">
          {bids.map((level) => (
            <OrderbookRow
              key={level.id}
              layout="stack"
              side="bid"
              level={level}
              showTotals={showTotals}
              compact={compact}
              heat={heat}
              maxMetric={bidMaxMetric}
            />
          ))}
        </div>
      </ScrollArea>
    </div>
  )
}
