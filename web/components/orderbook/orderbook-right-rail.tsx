import { OrderbookDebugView } from "./orderbook-debug-view"
import type { OrderbookLevel, OrderbookTrade } from "./orderbook-types"

type OrderbookRightRailProps = {
  bids: OrderbookLevel[]
  asks: OrderbookLevel[]
  trades: OrderbookTrade[]
  spread: number
  midPrice: number
  updatedAt: string
}

export function OrderbookRightRail({
  bids,
  asks,
  trades,
  spread,
  midPrice,
  updatedAt,
}: OrderbookRightRailProps) {
  return (
    <div className="flex h-full min-h-0 flex-col">
      <OrderbookDebugView
        bids={bids}
        asks={asks}
        trades={trades}
        spread={spread}
        midPrice={midPrice}
        updatedAt={updatedAt}
      />
    </div>
  )
}
