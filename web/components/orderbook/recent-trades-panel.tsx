import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"

import { TradeTape } from "./trade-tape"
import type { OrderbookTrade } from "./orderbook-types"

type RecentTradesPanelProps = {
  trades: OrderbookTrade[]
  compactView: boolean
  onCompactViewChange: (next: boolean) => void
  tableView: boolean
  onTableViewChange: (next: boolean) => void
}

export function RecentTradesPanel({
  trades,
  compactView,
  onCompactViewChange,
  tableView,
  onTableViewChange,
}: RecentTradesPanelProps) {
  return (
    <div className="flex h-full min-h-0 flex-col border border-border/60 bg-background/50">
      <div className="flex min-h-0 flex-1 flex-col px-2.5 pt-2.5 pb-2.5">
        <div className="mb-2 flex items-center justify-between gap-2 border-b border-border/60 pb-2">
          <div className="flex items-center gap-2">
            <span className="text-[11px] font-semibold tracking-[0.24em] text-muted-foreground uppercase">
              Recent Trades
            </span>
            <Badge variant="outline" className="font-mono text-[10px]">
              {trades.length}
            </Badge>
          </div>
          <div className="flex items-center gap-1.5">
            <Button
              type="button"
              variant={compactView ? "secondary" : "outline"}
              size="xs"
              onClick={() => {
                const next = !compactView
                onCompactViewChange(next)
                if (next) onTableViewChange(false)
              }}
            >
              Compact
            </Button>
            <Button
              type="button"
              variant={tableView ? "secondary" : "outline"}
              size="xs"
              onClick={() => {
                const next = !tableView
                onTableViewChange(next)
                if (next) onCompactViewChange(false)
              }}
            >
              Table
            </Button>
          </div>
        </div>
        <TradeTape
          trades={trades}
          compact={compactView}
          tableView={tableView}
        />
      </div>
    </div>
  )
}
