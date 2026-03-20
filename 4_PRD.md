# PRD.md

## Purpose
`solbook-core` is a deterministic, in-memory matching engine for spot crypto markets, beginning with **SOL/USDC** as the initial market. It is intended to demonstrate exchange-core architecture, realistic order lifecycle handling, and strong Rust systems engineering fundamentals.

## Product goals
### Primary goals
- Implement a correct price-time priority matching engine
- Model realistic spot crypto order book behavior
- Build a portfolio-grade Rust systems project
- Keep the core architecture clean and extensible
- Include a built-in learning layer through docs and glossary

### Secondary goals
- Benchmark-readiness
- Scenario-based demonstrations
- Clear, maintainable tests
- Future expansion toward market routing, feeds, and Solana-adjacent infrastructure

## Product principles
- correctness over hype
- deterministic behavior over hidden magic
- explicit state transitions
- type-driven design where it improves clarity
- pragmatic, idiomatic Rust
- architecture that is simple now and extensible later
- documentation and tests as first-class deliverables

## Market model
### Initial market
- `SOL/USDC`

### Internal design principle
The engine should be market-agnostic internally. `SOL/USDC` is the flagship example market, not a hardcoded architectural dependency.

### Market configuration
A market config should support:
- market ID / symbol
- base asset
- quote asset
- tick size
- lot size
- price precision
- quantity precision

## Functional requirements
- Limit orders
- Market orders
- Buy and sell sides
- Price priority first
- FIFO at the same price level
- Internal monotonic sequence numbers for accepted orders
- Accept valid orders
- Reject invalid orders
- Match crossing orders immediately
- Rest unfilled limit order quantity on the book
- Partial fills
- Full fills
- Cancellation by order ID
- Event / execution-report style outputs
- Top-of-book and snapshot helpers

## Validation requirements
The engine must validate:
- positive quantity
- positive price for limit orders
- tick size alignment
- lot size alignment
- market configuration consistency

## Core behavioral requirements
- Market orders never rest on the book
- Better prices execute before worse prices
- Same-price orders execute FIFO
- Empty price levels are removed eagerly

## Data model requirements
The system should include clean domain types such as:
- `MarketId`
- `OrderId`
- `SequenceNumber`
- `Price`
- `Quantity`
- `Side`
- `OrderType`
- `NewOrderRequest`
- `Order`
- `Trade`
- `BookEvent`
- `MarketConfig`
- `TopOfBook`
- `BookSnapshot`
- `BookLevelView`

## Testing requirements
The project must include both unit tests and integration tests.

### Unit tests
- `PriceLevel` FIFO behavior
- order validation
- best bid / best ask updates
- helper methods
- event generation basics
- snapshot helper behavior where isolated

### Integration tests
- submit resting bid / ask
- cross-book limit order matching
- partial fills
- cancellation
- market order sweeps across several price levels
- FIFO at same price
- price priority across multiple levels
- empty-book market behavior
- event sequence expectations
- best bid / best ask updates after trades and cancels
- snapshot output after state changes
- invariant-oriented scenarios

## Documentation requirements
The repo must include:
- `README.md`
- `docs/glossary.md`
- `docs/architecture.md`
- `docs/milestones.md`
- `docs/technical-architecture.md`

The glossary should define at minimum:
- order book
- bid
- ask
- spread
- best bid
- best ask
- top of book
- FIFO
- price level
- queue
- limit order
- market order
- resting order
- crossing order
- maker
- taker
- liquidity
- slippage
- tick size
- lot size
- price precision
- quantity precision
- validation
- rejection
- normalization
- sequence number
- deterministic
- NewOrderRequest
- Order
- Order ID
- partial fill
- trade
- event
- event stream
- execution report
- SubmissionResult
- CancelResult
- summary helpers
- TopOfBook
- book snapshot
- BookLevelView
- market config
- base asset
- quote asset
- invariant
- empty level cleanup
- producer
- consumer
- matching engine
- single-threaded core
- benchmark
- price-time priority

## Demo / portfolio requirements
The repo should include at least one visible demonstration path such as:
- examples folder
- scenario runner
- pretty printed snapshots
- sample order flow output

It should also include beginner-friendly docs so that a new learner can understand why key market rules and engine design choices exist.
