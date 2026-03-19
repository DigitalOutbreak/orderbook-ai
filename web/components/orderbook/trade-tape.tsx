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
import { formatPrice, formatSideLabel, formatSize } from "./mock-orderbook-data"
import type { OrderbookTrade } from "./orderbook-types"

type TradeTapeProps = {
  trades: OrderbookTrade[]
  compact?: boolean
  tableView?: boolean
}

export function TradeTape({
  trades,
  compact = false,
  tableView = false,
}: TradeTapeProps) {
  if (tableView) {
    return (
      <ScrollArea className="h-full min-h-0">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Side</TableHead>
              <TableHead>Price</TableHead>
              <TableHead>Size</TableHead>
              <TableHead>Time</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {trades.map((trade) => (
              <TableRow
                key={trade.id}
                className={cn(
                  trade.side === "buy" ? "bg-emerald-500/6" : "bg-rose-500/6"
                )}
              >
                <TableCell>
                  <Badge
                    variant="outline"
                    className={cn(
                      "px-1.5 py-0 font-sans text-[10px] tracking-[0.18em] uppercase",
                      trade.side === "buy"
                        ? "border-emerald-500/40 bg-emerald-500/12 text-emerald-300"
                        : "border-rose-500/40 bg-rose-500/12 text-rose-300"
                    )}
                  >
                    {formatSideLabel(trade.side)}
                  </Badge>
                </TableCell>
                <TableCell
                  className={cn(
                    "font-mono",
                    trade.side === "buy" ? "text-emerald-200" : "text-rose-200"
                  )}
                >
                  {formatPrice(trade.price)}
                </TableCell>
                <TableCell className="font-mono">
                  {formatSize(trade.size)}
                </TableCell>
                <TableCell className="font-mono text-muted-foreground">
                  {trade.time}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </ScrollArea>
    )
  }

  return (
    <ScrollArea className="h-full min-h-0">
      <div className="flex flex-col gap-1">
        {trades.map((trade) => (
          <div
            key={trade.id}
            className={cn(
              "grid grid-cols-[auto_1fr_auto_auto] items-center gap-2 border border-border/60 bg-background/60",
              compact ? "px-2.5 py-1.5" : "px-2.5 py-2"
            )}
          >
            <Badge
              variant="outline"
              className={cn(
                "px-1.5 py-0 font-sans text-[10px] tracking-[0.18em] uppercase",
                trade.side === "buy"
                  ? "border-emerald-500/40 bg-emerald-500/12 text-emerald-300"
                  : "border-rose-500/40 bg-rose-500/12 text-rose-300"
              )}
            >
              {formatSideLabel(trade.side)}
            </Badge>
            <div className="flex flex-col">
              <span
                className={cn(
                  "font-mono text-[13px] font-semibold",
                  trade.side === "buy" ? "text-emerald-300" : "text-rose-300"
                )}
              >
                {formatPrice(trade.price)}
              </span>
              {compact ? null : (
                <span className="text-[11px] text-muted-foreground">
                  {trade.venue}
                </span>
              )}
            </div>
            <span className="font-mono text-[13px] text-foreground/85">
              {formatSize(trade.size)}
            </span>
            <span className="text-right font-mono text-[11px] text-muted-foreground">
              {trade.time}
            </span>
          </div>
        ))}
      </div>
    </ScrollArea>
  )
}
