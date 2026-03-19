export type GlossaryEntry = {
  term: string
  category: string
  body: string[]
}

export const GLOSSARY_ENTRIES: GlossaryEntry[] = [
  {
    term: "Order book",
    category: "Core",
    body: [
      "The order book is the engine's in-memory state for active buy and sell interest.",
      "Matching happens by comparing an incoming order with the resting orders already in the book.",
    ],
  },
  {
    term: "Bid",
    category: "Core",
    body: [
      "A bid is a buy order.",
      "In SOL/USDC, a bid means someone wants to buy SOL and pay USDC.",
    ],
  },
  {
    term: "Ask",
    category: "Core",
    body: [
      "An ask is a sell order.",
      "In SOL/USDC, an ask means someone wants to sell SOL for USDC.",
    ],
  },
  {
    term: "Spread",
    category: "Core",
    body: [
      "The spread is the difference between the best ask and the best bid.",
      "Example: best bid 100.00, best ask 100.05, spread 0.05.",
    ],
  },
  {
    term: "Best bid",
    category: "Core",
    body: [
      "The highest resting buy price in the book.",
      "This is the most aggressive bid currently available.",
    ],
  },
  {
    term: "Best ask",
    category: "Core",
    body: [
      "The lowest resting sell price in the book.",
      "This is the cheapest ask currently available.",
    ],
  },
  {
    term: "Top of book",
    category: "Core",
    body: [
      "The compact view containing the current best bid and best ask.",
      "It is the fastest summary of the live market state.",
    ],
  },
  {
    term: "Limit order",
    category: "Orders",
    body: [
      "A limit order includes a price cap or floor.",
      "If it cannot fully execute immediately, any remaining quantity may rest on the book.",
    ],
  },
  {
    term: "Market order",
    category: "Orders",
    body: [
      "A market order has no limit price.",
      "It consumes the best available opposite-side liquidity until it is filled or the book runs out.",
    ],
  },
  {
    term: "Resting order",
    category: "Orders",
    body: [
      "A resting order is an order, or remainder of an order, that stays on the book waiting for a future match.",
    ],
  },
  {
    term: "Crossing order",
    category: "Orders",
    body: [
      "A crossing order is an incoming order whose price is good enough to match the current opposite side immediately.",
      "If the best ask is 100.00, a buy limit at 100.00 or higher is crossing.",
    ],
  },
  {
    term: "Maker",
    category: "Execution",
    body: [
      "The maker is the resting order in a trade.",
      "Makers add liquidity to the book.",
    ],
  },
  {
    term: "Taker",
    category: "Execution",
    body: [
      "The taker is the incoming order that triggers the trade.",
      "Takers remove liquidity from the book.",
    ],
  },
  {
    term: "Liquidity",
    category: "Execution",
    body: [
      "Liquidity is the executable resting interest already available in the book.",
      "More liquidity usually means easier execution with less slippage.",
    ],
  },
  {
    term: "Slippage",
    category: "Execution",
    body: [
      "Slippage is the difference between the expected price and the actual execution prices achieved.",
      "A market order that sweeps multiple levels can experience slippage.",
    ],
  },
  {
    term: "Trade",
    category: "Execution",
    body: [
      "A trade records an execution between a maker and a taker, including price and quantity.",
      "Recent trades answer the question: what just executed?",
    ],
  },
  {
    term: "Book snapshot",
    category: "Views",
    body: [
      "A snapshot is a read-only aggregated depth view of the current order book.",
      "It is not an event history. It is a structured view of what is resting now.",
    ],
  },
  {
    term: "Depth",
    category: "Views",
    body: [
      "Depth shows cumulative liquidity as you move away from the touch.",
      "It helps you see how much size is stacked near or far from the best prices.",
    ],
  },
  {
    term: "Size",
    category: "Views",
    body: [
      "Size means how much quantity is resting at one exact price level.",
    ],
  },
  {
    term: "Total",
    category: "Views",
    body: [
      "Total means cumulative quantity up to that level.",
      "If sizes are 42, 38, and 34, then totals are 42, 80, and 114.",
    ],
  },
  {
    term: "FIFO",
    category: "Structure",
    body: [
      "FIFO means first in, first out.",
      "If two orders rest at the same price, the one accepted earlier must execute first.",
    ],
  },
  {
    term: "Price level",
    category: "Structure",
    body: [
      "A price level is all resting orders at the same price.",
      "Within one level, orders are still executed in FIFO order.",
    ],
  },
  {
    term: "Queue",
    category: "Structure",
    body: [
      "A queue adds items at the back and removes them from the front.",
      "That is the abstract behavior required inside one price level.",
    ],
  },
  {
    term: "Tick size",
    category: "Rules",
    body: [
      "Tick size is the minimum legal price increment for a market.",
      "If the tick size is 0.01, a price like 100.005 is invalid.",
    ],
  },
  {
    term: "Lot size",
    category: "Rules",
    body: [
      "Lot size is the minimum legal quantity increment for a market.",
      "It defines what sizes are valid to submit.",
    ],
  },
  {
    term: "Validation",
    category: "Rules",
    body: [
      "Validation is the pre-matching rule check.",
      "It ensures positive quantities, positive prices, tick alignment, lot alignment, and precision limits.",
    ],
  },
  {
    term: "Rejection",
    category: "Rules",
    body: [
      "A rejection means the engine refuses invalid input.",
      "A strict engine rejects invalid orders explicitly instead of silently adjusting them.",
    ],
  },
  {
    term: "Venue",
    category: "Context",
    body: [
      "Venue means where a trade happened or which system produced it.",
      "In a learning UI, venue helps answer whether a trade came from a mock engine, a simulator, or a live market.",
    ],
  },
  {
    term: "Mock",
    category: "Context",
    body: [
      "Mock means fake data used for learning, testing, or UI development.",
      "It is useful for safely studying behavior before wiring a live feed.",
    ],
  },
  {
    term: "Simulated",
    category: "Context",
    body: [
      "Simulated means behavior that imitates a real market or engine without being connected to one.",
      "Simulated often suggests behavior shaped to resemble real execution more closely than a simple mock.",
    ],
  },
  {
    term: "Paper trading",
    category: "Context",
    body: [
      "Paper trading means placing pretend orders and tracking pretend fills without using real money.",
      "It is useful for learning interfaces and workflows, but it is still not live trading.",
    ],
  },
  {
    term: "Live",
    category: "Context",
    body: [
      "Live means connected to real market or production activity.",
      "If a book or trade feed is live, the prices, orders, and trades come from an actual running market.",
    ],
  },
]
