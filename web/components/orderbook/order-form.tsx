"use client"

import * as React from "react"

import type { OrderbookEvent, OrderbookSnapshot } from "./orderbook-types"
import {
  applyMockLimitOrder,
  deriveOrderValues,
  formatEditableDecimal,
  parseDecimal,
  sanitizeDecimalInput,
  type OrderFormField,
  type OrderFormSide,
} from "@/lib/order-calculations"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Separator } from "@/components/ui/separator"
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import { cn } from "@/lib/utils"

type OrderFormProps = {
  snapshot: OrderbookSnapshot
  onSnapshotChange: (snapshot: OrderbookSnapshot) => void
  onOrderEvent?: (event: OrderbookEvent) => void
  assetLabel?: string
}

type OrderFormState = {
  side: OrderFormSide
  price: string
  quantity: string
  total: string
}

function getBaseAsset(symbol: string) {
  return symbol.split("/")[0]?.trim() || "SOL"
}

export function OrderForm({
  snapshot,
  onSnapshotChange,
  onOrderEvent,
  assetLabel,
}: OrderFormProps) {
  const baseAsset = assetLabel ?? getBaseAsset(snapshot.stats.symbol)
  const [state, setState] = React.useState<OrderFormState>({
    side: "buy",
    price: snapshot.stats.bestBid.toFixed(2),
    quantity: "",
    total: "",
  })
  const [lastEdited, setLastEdited] = React.useState<OrderFormField>("quantity")
  const [statusMessage, setStatusMessage] = React.useState<string | null>(null)

  const parsedPrice = parseDecimal(state.price)
  const parsedQuantity = parseDecimal(state.quantity)
  const parsedTotal = parseDecimal(state.total)

  const isValidOrder =
    parsedPrice != null &&
    parsedPrice > 0 &&
    parsedQuantity != null &&
    parsedQuantity > 0 &&
    parsedTotal != null &&
    parsedTotal > 0

  const helperText =
    lastEdited === "total"
      ? "Quantity is derived from limit price and total."
      : "Total is derived from limit price and quantity."

  function updateField(field: OrderFormField, value: string) {
    const sanitized = sanitizeDecimalInput(value)
    const nextRaw = { ...state, [field]: sanitized }
    const derived = deriveOrderValues(
      {
        price: parseDecimal(nextRaw.price),
        quantity: parseDecimal(nextRaw.quantity),
        total: parseDecimal(nextRaw.total),
      },
      field
    )

    setLastEdited(field)
    setStatusMessage(null)
    setState({
      ...nextRaw,
      quantity:
        field === "quantity"
          ? sanitized
          : derived.quantity != null
            ? formatEditableDecimal(derived.quantity)
            : nextRaw.quantity,
      total:
        field === "total"
          ? sanitized
          : derived.total != null
            ? formatEditableDecimal(derived.total)
            : nextRaw.total,
    })
  }

  function handleSideChange(next: string) {
    if (next !== "buy" && next !== "sell") return
    setState((current) => ({
      ...current,
      side: next,
      price:
        next === "buy"
          ? snapshot.stats.bestBid.toFixed(2)
          : snapshot.stats.bestAsk.toFixed(2),
    }))
    setStatusMessage(null)
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (!isValidOrder || parsedPrice == null || parsedQuantity == null) return

    const result = applyMockLimitOrder(snapshot, {
      side: state.side,
      price: parsedPrice,
      quantity: parsedQuantity,
    })

    onSnapshotChange(result.snapshot)
    onOrderEvent?.(result.event)
    setStatusMessage(result.event.detail)
    setState((current) => ({
      ...current,
      quantity: "",
      total: "",
      price:
        current.side === "buy"
          ? result.snapshot.stats.bestBid.toFixed(2)
          : result.snapshot.stats.bestAsk.toFixed(2),
    }))
  }

  return (
    <Card
      size="sm"
      className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
    >
      <CardHeader className="gap-3 border-b border-border/60 pb-3">
        <div className="flex items-center justify-between gap-3">
          <div>
            <div className="flex items-center gap-2">
              <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                Order Entry
              </span>
              <Badge
                variant="secondary"
                className="px-1.5 font-mono text-[10px]"
              >
                {snapshot.stats.symbol}
              </Badge>
            </div>
            <p className="mt-1 font-mono text-xs text-muted-foreground">
              Limit-only mock workflow wired into the local book.
            </p>
          </div>
          <Badge
            variant={state.side === "buy" ? "secondary" : "outline"}
            className={cn(
              "px-1.5 font-mono text-[10px] uppercase",
              state.side === "buy" ? "text-emerald-300" : "text-rose-300"
            )}
          >
            {state.side === "buy" ? "Buy" : "Sell"}
          </Badge>
        </div>

        <div className="flex flex-wrap items-center justify-between gap-2">
          <ToggleGroup
            type="single"
            value={state.side}
            onValueChange={handleSideChange}
            variant="outline"
            size="sm"
          >
            <ToggleGroupItem
              value="buy"
              className="data-[state=on]:border-emerald-500/40 data-[state=on]:bg-emerald-500/10 data-[state=on]:text-emerald-300"
            >
              Buy
            </ToggleGroupItem>
            <ToggleGroupItem
              value="sell"
              className="data-[state=on]:border-rose-500/40 data-[state=on]:bg-rose-500/10 data-[state=on]:text-rose-300"
            >
              Sell
            </ToggleGroupItem>
          </ToggleGroup>

          <ToggleGroup type="single" value="limit" variant="outline" size="sm">
            <ToggleGroupItem value="limit">Limit</ToggleGroupItem>
            <ToggleGroupItem value="market" disabled>
              Market
            </ToggleGroupItem>
          </ToggleGroup>
        </div>
      </CardHeader>

      <CardContent className="pt-3">
        <form className="flex flex-col gap-3" onSubmit={handleSubmit}>
          <div className="grid gap-3 xl:grid-cols-3">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="order-price">Price</Label>
              <Input
                id="order-price"
                inputMode="decimal"
                placeholder="0.00"
                value={state.price}
                onChange={(event) => updateField("price", event.target.value)}
              />
            </div>

            <div className="flex flex-col gap-1.5">
              <Label htmlFor="order-quantity">Quantity</Label>
              <Input
                id="order-quantity"
                inputMode="decimal"
                placeholder="0.00"
                value={state.quantity}
                onChange={(event) =>
                  updateField("quantity", event.target.value)
                }
              />
            </div>

            <div className="flex flex-col gap-1.5">
              <Label htmlFor="order-total">Total</Label>
              <Input
                id="order-total"
                inputMode="decimal"
                placeholder="0.00"
                value={state.total}
                onChange={(event) => updateField("total", event.target.value)}
              />
            </div>
          </div>

          <div className="grid gap-3 xl:grid-cols-[1fr_auto] xl:items-center">
            <div className="flex min-w-0 flex-wrap items-center gap-2 text-[11px] text-muted-foreground">
              <Badge variant="outline" className="font-mono text-[10px]">
                Best Bid {snapshot.stats.bestBid.toFixed(2)}
              </Badge>
              <Badge variant="outline" className="font-mono text-[10px]">
                Best Ask {snapshot.stats.bestAsk.toFixed(2)}
              </Badge>
              <span className="font-mono">{helperText}</span>
            </div>

            <Button
              type="submit"
              variant={state.side === "buy" ? "default" : "secondary"}
              className={cn(
                "min-w-[180px] font-mono",
                state.side === "buy"
                  ? "bg-emerald-500/90 text-black hover:bg-emerald-400"
                  : "bg-rose-500/90 text-black hover:bg-rose-400"
              )}
              disabled={!isValidOrder}
            >
              {state.side === "buy" ? "Buy" : "Sell"} {baseAsset}
            </Button>
          </div>

          <Separator />

          <div className="flex flex-wrap items-center justify-between gap-2 font-mono text-[11px] text-muted-foreground">
            <span>
              Preview {parsedPrice?.toFixed(2) ?? "--"} x{" "}
              {parsedQuantity?.toFixed(2) ?? "--"} ={" "}
              {parsedTotal?.toFixed(2) ?? "--"}
            </span>
            {statusMessage ? (
              <span
                className={cn(
                  state.side === "buy" ? "text-emerald-300" : "text-rose-300"
                )}
              >
                {statusMessage}
              </span>
            ) : (
              <span>Market mode is disabled for now.</span>
            )}
          </div>
        </form>
      </CardContent>
    </Card>
  )
}
