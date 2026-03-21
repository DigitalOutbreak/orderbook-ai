import type {
  OrderbookEvent,
  OrderbookSnapshot,
} from "@/components/orderbook/orderbook-types"

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

type EngineScenarioResponse = {
  snapshot: OrderbookSnapshot
  event: OrderbookEvent
}

export type EngineScenarioId =
  | "balanced"
  | "trend-up"
  | "trend-down"
  | "range"
  | "bid-wall"
  | "ask-wall"
  | "thin-liquidity"
export type EngineSimulationSpeed = "slow" | "normal" | "fast" | "burst"

type EngineStreamPayload = {
  snapshot: OrderbookSnapshot
  event?: OrderbookEvent | null
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

export async function submitEngineLimitOrder(input: EngineSubmitOrderInput) {
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

export async function resetEngineBook() {
  const response = await fetch(`${API_BASE}/reset`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export async function seedEngineScenario(scenarioId: EngineScenarioId) {
  const response = await fetch(`${API_BASE}/scenarios/${scenarioId}`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export async function simulateCrossOrder(side: "buy" | "sell") {
  const response = await fetch(`${API_BASE}/simulate/cross-${side}`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export async function startEngineSimulation() {
  const response = await fetch(`${API_BASE}/simulation/start`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export async function stopEngineSimulation() {
  const response = await fetch(`${API_BASE}/simulation/stop`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export async function setEngineSimulationSpeed(speed: EngineSimulationSpeed) {
  const response = await fetch(`${API_BASE}/simulation/speed/${speed}`, {
    method: "POST",
    headers: { Accept: "application/json" },
  })

  if (!response.ok) {
    throw new Error(await parseError(response))
  }

  return (await response.json()) as EngineScenarioResponse
}

export function subscribeToEngineStream(handlers: {
  onUpdate: (payload: EngineStreamPayload) => void
  onError?: () => void
}) {
  const eventSource = new EventSource(`${API_BASE}/stream`)

  eventSource.addEventListener("engine-update", (event) => {
    const payload = JSON.parse(
      (event as MessageEvent<string>).data
    ) as EngineStreamPayload
    handlers.onUpdate(payload)
  })

  eventSource.onerror = () => {
    handlers.onError?.()
  }

  return () => {
    eventSource.close()
  }
}
