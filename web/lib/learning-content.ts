export type LearningLink = {
  title: string
  href: string
  description: string
}

export type LearningLesson = {
  title: string
  stage: string
  summary: string
  bullets: string[]
}

export type LearningDocSection = {
  title: string
  summary: string
  topics: string[]
}

export const LEARNING_LINKS: LearningLink[] = [
  {
    title: "Docs",
    href: "/docs",
    description:
      "Reference-style concepts for orderbooks, execution, and market structure.",
  },
  {
    title: "Lessons",
    href: "/learn/lessons",
    description:
      "Structured study tracks for understanding the engine and the terminal.",
  },
]

export const LEARNING_DOC_SECTIONS: LearningDocSection[] = [
  {
    title: "Orderbook Core",
    summary: "The minimum concepts you need to read the ladder correctly.",
    topics: [
      "Bid",
      "Ask",
      "Spread",
      "Best bid",
      "Best ask",
      "Top of book",
      "Price level",
    ],
  },
  {
    title: "Execution Mechanics",
    summary:
      "What happens when an incoming order interacts with resting liquidity.",
    topics: [
      "Limit order",
      "Market order",
      "Crossing order",
      "Maker",
      "Taker",
      "Trade",
      "Slippage",
    ],
  },
  {
    title: "Depth and Snapshot",
    summary: "How to interpret current state versus recent executions.",
    topics: ["Book snapshot", "Depth", "Size", "Total", "Liquidity"],
  },
  {
    title: "Engine Rules",
    summary: "Validation and deterministic behavior that keep the book sane.",
    topics: ["FIFO", "Tick size", "Lot size", "Validation", "Rejection"],
  },
]

export const LEARNING_LESSONS: LearningLesson[] = [
  {
    title: "Read the Ladder",
    stage: "Lesson 01",
    summary:
      "Learn how bids, asks, spread, and top of book relate visually inside the terminal.",
    bullets: [
      "Start with split view",
      "Track best bid / ask",
      "Toggle totals to see cumulative depth",
    ],
  },
  {
    title: "Resting vs Crossing",
    stage: "Lesson 02",
    summary:
      "Use the order form to see when an order rests on the book versus crossing the spread.",
    bullets: [
      "Submit a non-crossing limit",
      "Submit a crossing limit",
      "Compare event log output",
    ],
  },
  {
    title: "Depth Intuition",
    stage: "Lesson 03",
    summary:
      "Understand how size and total differ and how the depth chart reflects cumulative liquidity.",
    bullets: [
      "Compare split and depth",
      "Watch low / mid / high move",
      "Relate row fills to cumulative size",
    ],
  },
  {
    title: "Trades vs Snapshot",
    stage: "Lesson 04",
    summary:
      "Separate recent executions from current resting liquidity so the two views stop feeling redundant.",
    bullets: [
      "Use debug rail",
      "Compare recent trades to snapshot",
      "Watch what disappears after execution",
    ],
  },
  {
    title: "Microstructure Workflow",
    stage: "Lesson 05",
    summary:
      "Use chart, orderbook, event log, and glossary together like a learning workstation.",
    bullets: [
      "Anchor context with the chart",
      "Place a limit order",
      "Explain the resulting state transition",
    ],
  },
]
