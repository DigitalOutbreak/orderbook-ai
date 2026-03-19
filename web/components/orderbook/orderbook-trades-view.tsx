import { OrderbookSplitView } from "./orderbook-split-view"
import { RecentTradesPanel } from "./recent-trades-panel"
import type { OrderbookLevel, OrderbookTrade } from "./orderbook-types"

type OrderbookTradesViewProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  trades: OrderbookTrade[]
  showTotals: boolean
  compact: boolean
  heat: boolean
}

export function OrderbookTradesView({
  bids,
  asks,
  trades,
  showTotals,
  compact,
  heat,
}: OrderbookTradesViewProps) {
  const compactTape = false

  return (
    <div className="grid h-full gap-3 xl:grid-cols-[minmax(0,1.35fr)_minmax(300px,0.75fr)]">
      <div className="min-h-0 border border-border/60 bg-background/50 p-3">
        <OrderbookSplitView
          bids={bids}
          asks={asks}
          showTotals={showTotals}
          compact={compact}
          heat={heat}
        />
      </div>

      <RecentTradesPanel
        trades={trades}
        compactView={compactTape}
        onCompactViewChange={() => {}}
        tableView={false}
        onTableViewChange={() => {}}
      />
    </div>
  )
}
