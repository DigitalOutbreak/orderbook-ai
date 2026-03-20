# MVP_SCOPE.md

## Project summary
`solbook-core` is a deterministic, in-memory Rust matching engine for a spot crypto market, starting with **SOL/USDC** as the flagship market.

The MVP is intentionally scoped to be:
- realistic enough to be portfolio-worthy
- small enough to understand deeply
- testable and benchmarkable
- expandable toward future exchange, market data, and Solana-adjacent infrastructure

It should feel like a serious exchange core, not a toy app and not an overbuilt fake production system.

## MVP goals
- Build a correct, deterministic order book and matching core
- Support realistic spot market mechanics
- Include strong automated testing from day one
- Include documentation and glossary support for learning
- Produce a repo that is both educational and impressive
- Use Rust patterns that are clean, idiomatic, and maintainable

## Engineering principles
- Make invalid states hard to represent
- Prefer strong domain types over loose primitives when it improves clarity
- Favor simple, explicit code over clever abstraction
- Prefer composition over premature trait-heavy designs
- Use exact decimal arithmetic, never floats, for prices and quantities
- Keep the matching core deterministic and single-threaded
- Return structured results instead of hiding behavior in side effects
- Keep module boundaries clean and purposeful
- Use documentation and tests as part of the product
- Optimize for correctness first, benchmarkability second, optimization third

## In-scope
### Core engine
- One active market: `SOL/USDC`
- Spot only
- Limit orders
- Market orders
- Price-time priority
- FIFO within each price level
- Deterministic sequencing via internal sequence number
- Partial fills
- Cancel by order ID
- Event emission / execution reports
- In-memory state only
- Deterministic matching behavior
- Top-of-book and snapshot helpers

### Documentation
- README with architecture overview
- Glossary of trading / exchange terms
- MVP / PRD / technical architecture docs in `/docs`
- Clear inline Rust doc comments

### Testing
The MVP must include both unit tests and integration tests.

### Demo / presentation layer
At minimum, include one lightweight demonstration path:
- examples folder or tiny demo binary
- pretty printed order book snapshots

## Out of scope for MVP
- Wallets / balances / custody
- Settlement
- Fees
- Margin
- Liquidations
- Blockchain integration
- RPC calls
- Solana token account logic
- Networking / WebSocket server
- Persistence / database
- Multi-node / distributed architecture
- Multi-market routing in runtime
- Concurrent mutation of the book
- Lock-free internals
- Async runtime requirements

## Architecture constraints
- Library-first crate
- Single-threaded matching core
- No async runtime required in MVP
- Engine remains asset-agnostic internally even though `SOL/USDC` is the flagship market
- Use exact decimal arithmetic, not floats
- Use internal sequence numbers for FIFO, not wall-clock timestamps
- Keep market orders non-resting by definition
- Keep validation separate from matching logic
- Keep public API explicit and structured
- Reject invalid tick/lot inputs instead of auto-normalizing them
- Remove empty price levels eagerly

## Required deliverables
- Clean library crate
- Test suite with both unit and integration tests
- README
- `/docs/glossary.md`
- `/docs/architecture.md`
- `/docs/milestones.md`
- `/docs/technical-architecture.md`
- sample examples or scenario runner
- benchmark scaffold or benchmark-ready structure

## Glossary requirement expansion
The glossary should be written for a beginner and explain not just definitions, but why each term matters in this engine and, where helpful, provide a small example.
