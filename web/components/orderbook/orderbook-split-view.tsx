import { ScrollArea } from "@/components/ui/scroll-area"
import { OrderbookRow } from "./orderbook-row"
import { getMaxSize, getMaxTotal } from "./mock-orderbook-data"
import type { OrderbookLevel } from "./orderbook-types"

type OrderbookSplitViewProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  showTotals: boolean
  compact: boolean
  heat: boolean
}

export function OrderbookSplitView({
  bids,
  asks,
  showTotals,
  compact,
  heat,
}: OrderbookSplitViewProps) {
  const splitGrid = showTotals
    ? "grid-cols-[minmax(0,1fr)_minmax(0,1fr)_104px_minmax(0,1fr)_minmax(0,1fr)]"
    : "grid-cols-[minmax(0,1fr)_104px_minmax(0,1fr)]"

  const depth = Math.max(bids.length, asks.length)
  const rows = Array.from({ length: depth }, (_, index) => ({
    bid: bids[index],
    ask: asks[index],
  }))

  const maxBidMetric = showTotals ? getMaxTotal(bids) : getMaxSize(bids)
  const maxAskMetric = showTotals ? getMaxTotal(asks) : getMaxSize(asks)

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="border-b border-border/60 pb-2">
        <div
          className={`grid items-center gap-3 px-4 font-mono text-[10px] tracking-[0.2em] text-muted-foreground/80 uppercase ${splitGrid}`}
        >
          {showTotals ? (
            <>
              <span className="text-right text-emerald-300">Size</span>
              <span className="text-right text-emerald-300">Total</span>
              <span className="text-center text-muted-foreground">Price</span>
              <span className="text-left text-rose-300">Total</span>
              <span className="text-left text-rose-300">Size</span>
            </>
          ) : (
            <>
              <span className="text-right text-emerald-300">Size</span>
              <span className="text-center text-muted-foreground">Price</span>
              <span className="text-left text-rose-300">Size</span>
            </>
          )}
        </div>
      </div>
      <ScrollArea className="min-h-0 flex-1">
        <div className="flex flex-col gap-1.5 pr-3">
          {rows.map((row, index) => (
            <OrderbookRow
              key={`split-row-${index}`}
              layout="split"
              bid={row.bid}
              ask={row.ask}
              showTotals={showTotals}
              compact={compact}
              heat={heat}
              maxBidMetric={maxBidMetric}
              maxAskMetric={maxAskMetric}
            />
          ))}
        </div>
      </ScrollArea>
    </div>
  )
}
