import type { OrderbookEvent, OrderbookSnapshot } from "@/components/orderbook/orderbook-types"

const API_BASE =
  process.env.NEXT_PUBLIC_ENGINE_API_BASE ?? "http://127.0.0.1:8080"

type EngineSubmitOrderInput = {
  side: "buy" | "sell"
  price: string
  quantity: string
}

type EngineSubmitOrderResponse = {
  snapshot: OrderbookSnapshot
  event: OrderbookEvent
}

async function parseError(response: Response) {
  try {
    const data = (await response.json()) as { error?: string }
    return data.error || `Request failed with ${response.status}`
  } catch {
    return `Request failed with ${response.status}`
  }
}

export async function fetchEngineSnapshot(signal?: AbortSignal) {
  const response = await fetch(`${API_BASE}/snapshot`, {
    method: "GET",
    headers: { Accept: "application/json" },
    signal,
    cache: "no-store",
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as OrderbookSnapshot
}

export async function submitEngineLimitOrder(
  input: EngineSubmitOrderInput
) {
  const response = await fetch(`${API_BASE}/orders`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify({
      side: input.side,
      orderType: "limit",
      price: input.price,
      quantity: input.quantity,
    }),
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineSubmitOrderResponse
}
