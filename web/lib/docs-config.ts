export type DocsPage = {
  slug: string
  title: string
  section: string
  description: string
  file: string
}

export const DOCS_PAGES: DocsPage[] = [
  {
    slug: "introduction",
    title: "Introduction",
    section: "Getting Started",
    description: "Why this terminal exists and how to use it for learning.",
    file: "introduction.md",
  },
  {
    slug: "what-is-an-order-book",
    title: "What Is an Order Book",
    section: "Core Concepts",
    description: "Understand the book as live resting state rather than trade history.",
    file: "what-is-an-order-book.md",
  },
  {
    slug: "bids-and-asks",
    title: "Bids and Asks",
    section: "Core Concepts",
    description: "Read the two sides of the ladder correctly.",
    file: "bids-and-asks.md",
  },
  {
    slug: "spread-and-mid-price",
    title: "Spread and Mid Price",
    section: "Core Concepts",
    description: "Learn the main reference values near the touch.",
    file: "spread-and-mid-price.md",
  },
  {
    slug: "limit-orders",
    title: "Limit Orders",
    section: "Order Flow",
    description: "See when an order rests versus crosses.",
    file: "limit-orders.md",
  },
  {
    slug: "market-orders",
    title: "Market Orders",
    section: "Order Flow",
    description: "Understand liquidity removal and slippage.",
    file: "market-orders.md",
  },
  {
    slug: "depth-and-liquidity",
    title: "Depth and Liquidity",
    section: "Reading the Book",
    description: "Interpret size, cumulative totals, and depth shape.",
    file: "depth-and-liquidity.md",
  },
  {
    slug: "matching-engine-basics",
    title: "Matching Engine Basics",
    section: "Engine",
    description: "The minimum engine concepts needed to read the UI.",
    file: "matching-engine-basics.md",
  },
  {
    slug: "ui-notes-terminal-ideas",
    title: "UI Notes and Terminal Ideas",
    section: "Product Notes",
    description: "Design notes for a better trading-learning interface.",
    file: "ui-notes-terminal-ideas.md",
  },
]
