import {
  ClockCounterClockwise,
  CurrencyCircleDollar,
  DotOutline,
  Pulse,
  TrendDown,
  TrendUp,
} from "@phosphor-icons/react"

import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { formatPrice } from "./mock-orderbook-data"
import type { OrderbookStats } from "./orderbook-types"

type OrderbookStatsBarProps = {
  stats: OrderbookStats
}

const statItems = (stats: OrderbookStats) => [
  { label: "Symbol", value: stats.symbol, icon: DotOutline },
  { label: "Best Bid", value: formatPrice(stats.bestBid), icon: TrendUp },
  { label: "Best Ask", value: formatPrice(stats.bestAsk), icon: TrendDown },
  {
    label: "Spread",
    value: formatPrice(stats.spread),
    icon: CurrencyCircleDollar,
  },
  { label: "Mid Price", value: formatPrice(stats.midPrice), icon: Pulse },
  { label: "Last Update", value: stats.updatedAt, icon: ClockCounterClockwise },
]

export function OrderbookStatsBar({ stats }: OrderbookStatsBarProps) {
  return (
    <Card className="border-border/60 bg-card/80">
      <CardContent className="flex flex-col gap-3 p-4">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div className="flex items-center gap-3">
            <Badge variant="secondary" className="tracking-[0.24em] uppercase">
              Orderbook Terminal
            </Badge>
            <Badge
              variant={stats.mode === "mock" ? "outline" : "secondary"}
              className="tracking-[0.24em] uppercase"
            >
              {stats.mode === "mock" ? "Mock" : "Live"}
            </Badge>
          </div>
          <div className="flex items-center gap-2 text-xs text-muted-foreground">
            {stats.mode === "mock" ? (
              <ClockCounterClockwise className="size-4" />
            ) : (
              <Pulse className="size-4" />
            )}
            Visualization-first terminal scaffold
          </div>
        </div>

        <Separator />

        <div className="grid gap-3 xl:grid-cols-6">
          {statItems(stats).map((item) => {
            const Icon = item.icon
            return (
              <Tooltip key={item.label}>
                <TooltipTrigger asChild>
                  <div className="flex min-w-0 items-start gap-2.5 border border-border/60 bg-background/50 px-3 py-2.5">
                    <Icon className="mt-0.5 size-3.5 text-primary" />
                    <div className="min-w-0">
                      <p className="text-[10px] font-semibold tracking-[0.24em] text-muted-foreground uppercase">
                        {item.label}
                      </p>
                      <p className="mt-0.5 truncate font-mono text-[13px] font-semibold text-foreground">
                        {item.value}
                      </p>
                    </div>
                  </div>
                </TooltipTrigger>
                <TooltipContent>{item.label}</TooltipContent>
              </Tooltip>
            )
          })}
        </div>
      </CardContent>
    </Card>
  )
}
