"use client"

import * as React from "react"
import {
  ArrowsOutCardinal,
  ChartLine,
  ListBullets,
  StackSimple,
} from "@phosphor-icons/react"

import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardHeader } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import { cn } from "@/lib/utils"
import { MarketChartPanel } from "@/components/market-chart/market-chart-panel"

import { createMockOrderbookSnapshot, formatPrice } from "./mock-orderbook-data"
import { OrderbookDepthView } from "./orderbook-depth-view"
import { OrderbookEventLog } from "./orderbook-event-log"
import { GlossarySheet } from "./glossary-sheet"
import { OrderForm } from "./order-form"
import { OrderbookRightRail } from "./orderbook-right-rail"
import { OrderbookSplitView } from "./orderbook-split-view"
import { OrderbookStackView } from "./orderbook-stack-view"
import type {
  OrderbookDisplayOption,
  OrderbookEvent,
  OrderbookSnapshot,
  OrderbookView,
} from "./orderbook-types"

const displayDefaults: OrderbookDisplayOption[] = ["heat", "totals", "compact"]

function normalizeDisplayOptions(
  value: OrderbookDisplayOption[]
): OrderbookDisplayOption[] {
  const next: OrderbookDisplayOption[] = [...value]
  if (!next.includes("compact")) next.push("compact")
  if (!next.includes("heat")) next.push("heat")
  return next
}

export function OrderbookTerminal() {
  const [snapshot, setSnapshot] = React.useState<OrderbookSnapshot>(() =>
    createMockOrderbookSnapshot()
  )
  const [events, setEvents] = React.useState<OrderbookEvent[]>([])
  const [view, setView] = React.useState<OrderbookView>("split")
  const [glossaryOpen, setGlossaryOpen] = React.useState(false)
  const [displayOptions, setDisplayOptions] =
    React.useState<OrderbookDisplayOption[]>(displayDefaults)

  const heat = displayOptions.includes("heat")
  const showTotals = displayOptions.includes("totals")
  const compact = displayOptions.includes("compact")
  const showLearn = displayOptions.includes("learn")
  const visibleBids = snapshot.bids
  const visibleAsks = snapshot.asks
  const showRightRail = showLearn

  const workspaceClassName = cn(
    "h-full min-h-0",
    showRightRail
      ? "grid gap-3 xl:grid-cols-[minmax(0,1.35fr)_minmax(280px,0.8fr)]"
      : "flex flex-col"
  )

  const mainPanelClassName =
    "flex h-full min-h-0 flex-col border border-border/60 bg-background/50 p-3"

  return (
    <div className="mx-auto flex h-full w-full max-w-[1680px] overflow-hidden px-4 py-3">
      <div className="grid h-full min-h-0 w-full gap-3 xl:grid-cols-[minmax(0,1.08fr)_minmax(0,0.92fr)]">
        <div className="flex min-h-0 flex-col gap-3">
          <MarketChartPanel className="min-h-0 flex-1" />
          <OrderForm
            snapshot={snapshot}
            onSnapshotChange={setSnapshot}
            onOrderEvent={(event) =>
              setEvents((current) => [event, ...current].slice(0, 24))
            }
          />
        </div>

        <div className="flex min-h-0 flex-col gap-3">
          <Card
            size="sm"
            className="flex min-h-0 flex-1 flex-col border-border/60 bg-card/80 shadow-2xl shadow-black/15"
          >
            <CardHeader className="gap-1.5 border-b border-border/60 pb-3">
              <div className="flex items-center gap-2">
                <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                  Orderbook
                </span>
                <Badge
                  variant="secondary"
                  className="px-1.5 font-mono text-[10px]"
                >
                  {snapshot.stats.symbol}
                </Badge>
              </div>
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex min-w-0 flex-wrap items-center gap-2">
                  <span className="font-mono text-lg font-semibold">
                    {formatPrice(snapshot.stats.midPrice)}
                  </span>
                  <Badge variant="outline" className="px-1.5 text-[10px]">
                    Spread {formatPrice(snapshot.stats.spread)}
                  </Badge>
                </div>

                <div className="flex items-center gap-2">
                  <ToggleGroup
                    type="single"
                    value={view}
                    onValueChange={(value) => {
                      if (value) setView(value as OrderbookView)
                    }}
                    variant="outline"
                    size="sm"
                  >
                    <ToggleGroupItem value="split" aria-label="Split view">
                      <ArrowsOutCardinal data-icon="inline-start" />
                      Split
                    </ToggleGroupItem>
                    <ToggleGroupItem value="stack" aria-label="Stack view">
                      <StackSimple data-icon="inline-start" />
                      Stack
                    </ToggleGroupItem>
                    <ToggleGroupItem value="depth" aria-label="Depth view">
                      <ChartLine data-icon="inline-start" />
                      Depth
                    </ToggleGroupItem>
                  </ToggleGroup>
                  <Separator
                    orientation="vertical"
                    className="hidden h-6 lg:block"
                  />
                  <ToggleGroup
                    type="multiple"
                    value={displayOptions}
                    onValueChange={(value) =>
                      setDisplayOptions(
                        normalizeDisplayOptions(
                          value as OrderbookDisplayOption[]
                        )
                      )
                    }
                    variant="outline"
                    size="sm"
                  >
                    <ToggleGroupItem value="totals" aria-label="Toggle totals">
                      <ListBullets />
                      Totals
                    </ToggleGroupItem>
                    <ToggleGroupItem
                      value="learn"
                      aria-label="Toggle debug panel"
                    >
                      Debug
                    </ToggleGroupItem>
                  </ToggleGroup>
                </div>
              </div>
            </CardHeader>

            <CardContent className="min-h-0 flex-1 p-3">
              {view === "split" ? (
                <div className={workspaceClassName}>
                  <div className={mainPanelClassName}>
                    <OrderbookSplitView
                      bids={visibleBids}
                      asks={visibleAsks}
                      showTotals={showTotals}
                      compact={compact}
                      heat={heat}
                    />
                  </div>
                  {showRightRail ? (
                    <OrderbookRightRail
                      bids={visibleBids}
                      asks={visibleAsks}
                      trades={snapshot.trades}
                      spread={snapshot.stats.spread}
                      midPrice={snapshot.stats.midPrice}
                      updatedAt={snapshot.stats.updatedAt}
                    />
                  ) : null}
                </div>
              ) : null}

              {view === "stack" ? (
                <div className={workspaceClassName}>
                  <div className={mainPanelClassName}>
                    <OrderbookStackView
                      bids={visibleBids}
                      asks={visibleAsks}
                      showTotals={showTotals}
                      compact={compact}
                      heat={heat}
                      spread={snapshot.stats.spread}
                      midPrice={snapshot.stats.midPrice}
                    />
                  </div>
                  {showRightRail ? (
                    <OrderbookRightRail
                      bids={visibleBids}
                      asks={visibleAsks}
                      trades={snapshot.trades}
                      spread={snapshot.stats.spread}
                      midPrice={snapshot.stats.midPrice}
                      updatedAt={snapshot.stats.updatedAt}
                    />
                  ) : null}
                </div>
              ) : null}

              {view === "depth" ? (
                <div className={workspaceClassName}>
                  <div className={mainPanelClassName}>
                    <OrderbookDepthView
                      bids={visibleBids}
                      asks={visibleAsks}
                      showTotals={showTotals}
                      compact={compact}
                      heat={heat}
                      midPrice={snapshot.stats.midPrice}
                      condensed={showRightRail}
                    />
                  </div>
                  {showRightRail ? (
                    <OrderbookRightRail
                      bids={visibleBids}
                      asks={visibleAsks}
                      trades={snapshot.trades}
                      spread={snapshot.stats.spread}
                      midPrice={snapshot.stats.midPrice}
                      updatedAt={snapshot.stats.updatedAt}
                    />
                  ) : null}
                </div>
              ) : null}
            </CardContent>
          </Card>

          <div className="h-[188px] shrink-0">
            <OrderbookEventLog
              events={events}
              onOpenGlossary={() => setGlossaryOpen(true)}
            />
          </div>
        </div>
      </div>
      <GlossarySheet open={glossaryOpen} onOpenChange={setGlossaryOpen} />
    </div>
  )
}
