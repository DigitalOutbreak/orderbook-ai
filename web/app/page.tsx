import { OrderbookTerminal } from "@/components/orderbook/orderbook-terminal"

export default function Page() {
  return (
    <main className="h-svh overflow-hidden bg-[radial-gradient(circle_at_top_left,rgba(168,85,247,0.14),transparent_28%),radial-gradient(circle_at_top_right,rgba(244,114,182,0.1),transparent_24%),linear-gradient(180deg,rgba(10,12,20,1),rgba(14,18,30,1))]">
      <OrderbookTerminal />
    </main>
  )
}
