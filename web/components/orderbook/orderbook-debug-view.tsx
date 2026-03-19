import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { cn } from "@/lib/utils"

import { formatPrice, formatSize } from "./mock-orderbook-data"
import { TradeTape } from "./trade-tape"
import type { OrderbookLevel, OrderbookTrade } from "./orderbook-types"

type OrderbookDebugViewProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  trades: OrderbookTrade[]
  spread: number
  midPrice: number
  updatedAt: string
}

export function OrderbookDebugView({
  bids,
  asks,
  trades,
  updatedAt,
}: OrderbookDebugViewProps) {
  return (
    <div className="flex h-full min-h-0 flex-col gap-3">
      <div className="border border-border/60 bg-background/50 p-2.5">
        <div className="mb-2 flex items-center justify-between gap-2 border-b border-border/60 pb-2">
          <span className="text-[11px] font-semibold tracking-[0.24em] text-muted-foreground uppercase">
            Recent Trades
          </span>
          <Badge variant="outline" className="font-mono text-[10px]">
            {trades.length}
          </Badge>
        </div>
        <div className="h-[220px]">
          <TradeTape trades={trades} tableView />
        </div>
      </div>

      <div className="flex min-h-0 flex-1 flex-col border border-border/60 bg-background/50">
        <div className="flex items-center justify-between gap-2 border-b border-border/60 px-2.5 py-2">
          <span className="text-[11px] font-semibold tracking-[0.24em] text-muted-foreground uppercase">
            Snapshot
          </span>
          <Badge variant="secondary" className="text-[10px]">
            {updatedAt}
          </Badge>
        </div>
        <div className="min-h-0 flex-1">
          <ScrollArea className="h-full">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Side</TableHead>
                  <TableHead>Price</TableHead>
                  <TableHead>Size</TableHead>
                  <TableHead>Total</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {[...asks.slice(0, 4), ...bids.slice(0, 4)].map((level) => (
                  <TableRow
                    key={level.id}
                    className={cn(
                      level.side === "bid"
                        ? "bg-emerald-500/6"
                        : "bg-rose-500/6"
                    )}
                  >
                    <TableCell
                      className={cn(
                        "font-semibold uppercase",
                        level.side === "bid"
                          ? "text-emerald-300"
                          : "text-rose-300"
                      )}
                    >
                      {level.side}
                    </TableCell>
                    <TableCell
                      className={cn(
                        "font-mono",
                        level.side === "bid"
                          ? "text-emerald-200"
                          : "text-rose-200"
                      )}
                    >
                      {formatPrice(level.price)}
                    </TableCell>
                    <TableCell className="font-mono">
                      {formatSize(level.size)}
                    </TableCell>
                    <TableCell className="font-mono">
                      {formatSize(level.total)}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </ScrollArea>
        </div>
      </div>
    </div>
  )
}
