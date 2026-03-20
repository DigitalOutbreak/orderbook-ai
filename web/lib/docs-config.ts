export type DocsPage = {
  slug: string
  title: string
  section: string
  description: string
  file: string
  source: "web" | "repo"
}

export const DOCS_PAGES: DocsPage[] = [
  {
    slug: "learning-path",
    title: "Learning Path",
    section: "Orientation",
    description: "The recommended order for studying the engine and terminal.",
    file: "learning-path.md",
    source: "repo",
  },
  {
    slug: "00-start-here",
    title: "00 Start Here",
    section: "Orientation",
    description:
      "How to approach the project without getting lost in the implementation.",
    file: "learning/00-start-here.md",
    source: "repo",
  },
  {
    slug: "01-big-picture",
    title: "01 Big Picture",
    section: "Orientation",
    description:
      "The mental model for how the engine and learning surfaces fit together.",
    file: "learning/01-big-picture.md",
    source: "repo",
  },
  {
    slug: "introduction",
    title: "Introduction",
    section: "Market Structure",
    description: "Why this terminal exists and how to use it for learning.",
    file: "introduction.md",
    source: "web",
  },
  {
    slug: "what-is-an-order-book",
    title: "What Is an Order Book",
    section: "Market Structure",
    description:
      "Understand the book as live resting state rather than trade history.",
    file: "what-is-an-order-book.md",
    source: "web",
  },
  {
    slug: "bids-and-asks",
    title: "Bids and Asks",
    section: "Market Structure",
    description: "Read the two sides of the ladder correctly.",
    file: "bids-and-asks.md",
    source: "web",
  },
  {
    slug: "spread-and-mid-price",
    title: "Spread and Mid Price",
    section: "Market Structure",
    description: "Learn the main reference values near the touch.",
    file: "spread-and-mid-price.md",
    source: "web",
  },
  {
    slug: "limit-orders",
    title: "Limit Orders",
    section: "Market Structure",
    description: "See when an order rests versus crosses.",
    file: "limit-orders.md",
    source: "web",
  },
  {
    slug: "market-orders",
    title: "Market Orders",
    section: "Market Structure",
    description: "Understand liquidity removal and slippage.",
    file: "market-orders.md",
    source: "web",
  },
  {
    slug: "depth-and-liquidity",
    title: "Depth and Liquidity",
    section: "Market Structure",
    description: "Interpret size, cumulative totals, and depth shape.",
    file: "depth-and-liquidity.md",
    source: "web",
  },
  {
    slug: "matching-engine-basics",
    title: "Matching Engine Basics",
    section: "Engine Study",
    description: "The minimum engine concepts needed to read the UI.",
    file: "matching-engine-basics.md",
    source: "web",
  },
  {
    slug: "02-core-domain",
    title: "02 Core Domain",
    section: "Engine Study",
    description:
      "Learn the engine-owned types before trying to follow matching logic.",
    file: "learning/02-core-domain.md",
    source: "repo",
  },
  {
    slug: "03-matching-flow",
    title: "03 Matching Flow",
    section: "Engine Study",
    description:
      "Trace one complete submit path through validation, matching, and resting.",
    file: "learning/03-matching-flow.md",
    source: "repo",
  },
  {
    slug: "05-guided-exercises",
    title: "05 Guided Exercises",
    section: "Engine Study",
    description:
      "Hands-on exercises for turning the docs into real understanding.",
    file: "learning/05-guided-exercises.md",
    source: "repo",
  },
  {
    slug: "06-visual-guide",
    title: "06 Visual Guide",
    section: "Engine Study",
    description:
      "Bridge the engine concepts into the terminal UI and ladder views.",
    file: "learning/06-visual-guide.md",
    source: "repo",
  },
  {
    slug: "07-why-this-design",
    title: "07 Why This Design",
    section: "Engine Study",
    description:
      "Why the project is structured as a deterministic engine first.",
    file: "learning/07-why-this-design.md",
    source: "repo",
  },
  {
    slug: "08-performance-bridge",
    title: "08 Performance Bridge",
    section: "Engine Study",
    description:
      "A gentle transition from matching flow to engine performance thinking.",
    file: "learning/08-performance-bridge.md",
    source: "repo",
  },
  {
    slug: "architecture",
    title: "Architecture",
    section: "Architecture",
    description: "High-level module and state layout of the engine.",
    file: "architecture.md",
    source: "repo",
  },
  {
    slug: "technical-architecture",
    title: "Technical Architecture",
    section: "Architecture",
    description:
      "Implementation-facing summary of invariants, APIs, and boundaries.",
    file: "technical-architecture.md",
    source: "repo",
  },
  {
    slug: "ui-notes-terminal-ideas",
    title: "UI Notes and Terminal Ideas",
    section: "Architecture",
    description: "Design notes for a better trading-learning interface.",
    file: "ui-notes-terminal-ideas.md",
    source: "web",
  },
  {
    slug: "engine-performance",
    title: "Engine Performance Notes",
    section: "Performance",
    description:
      "Understand the hot paths, data structures, and cost model of the engine.",
    file: "engine-performance.md",
    source: "repo",
  },
  {
    slug: "performance",
    title: "Performance",
    section: "Performance",
    description:
      "Benchmark commands, current baselines, and profiling guidance.",
    file: "performance.md",
    source: "repo",
  },
  {
    slug: "milestones",
    title: "Milestones",
    section: "Project",
    description:
      "The implementation phases and where the project stands today.",
    file: "milestones.md",
    source: "repo",
  },
  {
    slug: "glossary",
    title: "Glossary",
    section: "Reference",
    description: "Core orderbook and trading terms used across the project.",
    file: "glossary.md",
    source: "repo",
  },
]
