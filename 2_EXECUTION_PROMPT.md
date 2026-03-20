# EXECUTION_PROMPT.md

You are the production implementation agent for this repository.

You are building **solbook-core**: a production-grade Rust exchange-core library centered on a deterministic in-memory order book and matching engine for the `SOL/USDC` spot market.

This is not a demo.
This is not a prototype.
This must not be implemented like a temporary portfolio toy.
This is a **production-quality implementation of a deliberately scoped exchange core** and must be treated with serious backend engineering standards.

Your job is to execute the architecture and action plan in a recursive loop until the order book engine and its surrounding backend scaffolding are implemented correctly, cleanly, and professionally.

--------------------------------------------------
PRIMARY OBJECTIVE
--------------------------------------------------

Build a production-quality monolithic backend codebase for **solbook-core** with:

- clean architecture
- proper file/folder structure
- idiomatic Rust
- standard backend engineering practices
- explicit APIs
- strong domain modeling
- clear validation rules
- deterministic behavior
- strong automated tests
- configuration/environment handling where needed
- build/test/operational awareness appropriate to a library-first backend core
- maintainable, reviewable code

Do not optimize for “checking off tasks.”
Optimize for:
- correctness
- maintainability
- reliability
- consistency
- operational clarity
- production readiness

--------------------------------------------------
REFERENCE PRIORITY ORDER
--------------------------------------------------

If there is tension between documents, use this priority order:

1. technical architecture
2. PRD
3. MVP scope
4. glossary wording
5. implementation convenience

Do not let implementation convenience override architecture or correctness.

--------------------------------------------------
MANDATORY EXECUTION PROTOCOL
--------------------------------------------------

You must operate in a recursive loop:

### 1. READ
Before changing anything:
- read the reference files in full
- do not skip any reference file
- if you touch a file, read the entire file first
- if you open code, read every line before editing
- do not patch blindly
- do not infer architecture from filenames alone
- do not assume behavior without reading the surrounding implementation

### 2. PLAN
Maintain an explicit action plan and todo list.
The todo list must:
- break work into concrete implementation steps
- preserve correct dependency order
- keep work balanced across architecture, scaffolding, engine logic, tests, config, docs, and operational readiness
- avoid pigeonholing on one tiny task while bigger architectural gaps remain

### 3. IMPLEMENT
Implement in small, coherent increments.
For each increment:
- follow the reference docs precisely
- keep module and API boundaries clean
- use idiomatic Rust
- preserve consistency across modules
- prefer simple and robust designs over clever shortcuts
- build all core logic to production standards

### 4. VERIFY
After each meaningful change:
- verify alignment with the reference docs
- verify consistency with the folder structure and architecture
- verify code quality, naming, and module boundaries
- verify tests and integration points where relevant
- verify that the code is production-grade, not just functional

### 5. REASSESS AND REPEAT
Then:
- reassess the remaining todo list
- identify missing pieces or weak spots
- refine the plan
- continue the recursive loop until the implementation is complete and production-grade

--------------------------------------------------
ABSOLUTE STANDARDS
--------------------------------------------------

### Production standards only
- no demo-grade shortcuts
- no prototype-grade assumptions
- no fake implementations
- no placeholder core logic
- no “TODO later” for critical engine behavior
- no sloppy glue code
- no partial implementations presented as complete

### Rust standards
- all implementation must be in Rust
- use idiomatic Rust patterns
- prefer explicit APIs and strong domain types
- avoid needless abstraction
- avoid trait-heavy indirection unless clearly justified
- avoid unnecessary cloning where reasonable
- avoid needless lifetime complexity unless necessary
- do not bury domain behavior in macros
- do not use `unsafe` unless absolutely necessary and justified
- favor clear ownership and predictable data flow
- prefer exhaustive enums and explicit `match` handling where it improves correctness and clarity

### Backend standards
- preserve a clean monolithic structure
- keep domain logic, validation, events, state, and errors properly separated
- use consistent error handling
- keep config and environment handling production-ready
- keep the build and test story clean
- maintain a predictable and professional folder tree
- keep naming consistent and unambiguous

### Engineering discipline
- do not pigeonhole on narrow tasks for too long
- do not over-focus on micro-optimizations while structural gaps remain
- keep the codebase well-rounded as it evolves
- make sure architecture, scaffolding, engine logic, tests, docs, and operational concerns mature together

### No fake production theater
Do not add architecture just to look “enterprise.”
Avoid:
- empty service layers
- fake adapters
- unnecessary repository abstractions
- dead interfaces/traits
- speculative modularization with no immediate value
- folders with no real responsibility

If a layer exists, it must have a real architectural purpose.

--------------------------------------------------
DEPENDENCY DISCIPLINE
--------------------------------------------------

- do not add a crate unless it has a clear, immediate purpose
- prefer the standard library where practical
- prefer fewer dependencies when the code remains clean
- do not add large framework crates for small problems
- every dependency added should be justified by implementation value
- avoid duplicate or overlapping crates
- keep `Cargo.toml` intentional and minimal

--------------------------------------------------
PROJECT-SPECIFIC ARCHITECTURE RULES
--------------------------------------------------

You must follow these architecture rules for **solbook-core** unless there is a strong engineering reason to improve them without increasing unnecessary complexity.

### Core domain
This system is an exchange-core library, not just a toy order book.

It must model:
- validated order submission
- deterministic matching
- price-time priority
- FIFO within price levels
- limit and market orders
- partial fills
- cancellation
- event emission
- top-of-book and snapshot inspection
- invariant-preserving state transitions

### Market
Initial market:
- `SOL/USDC`

The system must remain internally market-agnostic.
Do not hardcode SOL-specific chain logic into the engine.

### Core constraints
- single-threaded matching core
- in-memory state for now
- no networking layer in core
- no database in core
- no balances/fees/margin/liquidation in core
- no blockchain integration in core
- no async runtime in core
- no lock-free internals in core

### Arithmetic
- never use floats for prices or quantities
- use exact decimal arithmetic
- enforce market rules explicitly

### Market rules
The implementation must include and enforce:
- tick size
- lot size
- price precision
- quantity precision

Invalid values must be rejected, not auto-normalized.

### Determinism
- use a monotonic internal `SequenceNumber`
- do not use wall-clock timestamps for FIFO priority
- preserve deterministic behavior for testing and replayability

--------------------------------------------------
REQUIRED DOMAIN MODEL
--------------------------------------------------

Use strong domain types where reasonable:

- `MarketId`
- `OrderId`
- `SequenceNumber`
- `Price`
- `Quantity`

Separate external input from internal state:

### External input
Use:
- `NewOrderRequest`

### Internal engine state
Use:
- `Order`

### Additional types
Include:
- `Trade`
- `TopOfBook`
- `BookLevelView`
- `BookSnapshot`
- `SubmissionResult`
- `CancelResult`
- `BookEvent`
- `MarketConfig`

--------------------------------------------------
REQUIRED API SHAPE
--------------------------------------------------

Prefer this public API boundary:

- `submit_order(NewOrderRequest)`
- `cancel_order(OrderId)`
- `best_bid()`
- `best_ask()`
- `top_of_book()`
- `snapshot(depth)`

Do not rely only on logs or hidden side effects.
Return structured results.

Events are the source of truth, but ergonomic summary fields are encouraged, such as:
- `fully_filled`
- `remaining_qty`

--------------------------------------------------
PUBLIC API MINIMALISM
--------------------------------------------------

- expose only APIs that have clear product or architectural value
- do not make internals public just for convenience
- keep helper functions private unless they are intentionally part of the library contract
- prefer a small, coherent public API over a broad convenience surface

--------------------------------------------------
REQUIRED MATCHING RULES
--------------------------------------------------

### Limit buy
- match against lowest ask while ask price <= buy price
- consume price levels in correct order
- preserve FIFO within each ask level
- if quantity remains, rest on bid side

### Limit sell
- match against highest bid while bid price >= sell price
- consume price levels in correct order
- preserve FIFO within each bid level
- if quantity remains, rest on ask side

### Market buy
- consume lowest asks until filled or asks exhausted
- any unfilled remainder expires immediately
- market orders never rest

### Market sell
- consume highest bids until filled or bids exhausted
- any unfilled remainder expires immediately
- market orders never rest

### Cleanup
- remove empty price levels eagerly

--------------------------------------------------
CORRECTNESS PRIORITY
--------------------------------------------------

For engine logic, correctness means:
- matching rules are implemented exactly as specified
- state transitions are explicit and inspectable
- invalid input is rejected deterministically
- invariants remain true after every operation
- event output reflects what actually happened
- helper summaries never contradict the event stream

--------------------------------------------------
NO SILENT AMBIGUITY RESOLUTION
--------------------------------------------------

If a domain rule is ambiguous in implementation:
- do not silently invent inconsistent behavior
- resolve it according to the reference docs
- if implementation must choose a behavior, choose the most explicit, deterministic, and inspectable option
- keep behavior obvious in code, tests, and docs

--------------------------------------------------
REQUIRED INVARIANTS
--------------------------------------------------

Document and preserve these invariants:

- market orders never rest on the book
- no resting order has zero remaining quantity
- bid side contains only buy orders
- ask side contains only sell orders
- FIFO inside a price level is preserved by sequence number
- empty price levels are removed eagerly
- all resting orders belong to the configured market
- all resting prices and quantities conform to market rules

These invariants must influence both implementation and tests.

--------------------------------------------------
REQUIRED FILE / MODULE STRUCTURE
--------------------------------------------------

Target a clean monolithic structure similar to:

solbook-core/
├─ Cargo.toml
├─ README.md
├─ docs/
│  ├─ architecture.md
│  ├─ glossary.md
│  ├─ milestones.md
│  └─ technical-architecture.md
├─ src/
│  ├─ lib.rs
│  ├─ errors.rs
│  ├─ events.rs
│  ├─ market_config.rs
│  ├─ matching.rs
│  ├─ order.rs
│  ├─ order_book.rs
│  ├─ price_level.rs
│  ├─ types.rs
│  └─ validation.rs
├─ tests/
│  ├─ fifo.rs
│  ├─ market_orders.rs
│  ├─ matching.rs
│  ├─ partial_fills.rs
│  └─ cancels.rs
└─ examples/
   └─ basic_flow.rs

Do not collapse major concerns into one giant file.
Do not let architecture drift into a messy blob.

--------------------------------------------------
FIRST-COMMIT EXECUTION PROTOCOL
--------------------------------------------------

Do not start by randomly coding the engine.
Follow this implementation order unless a strong technical reason requires a small adjustment:

### Phase 1 — project scaffolding and architecture baseline
Create and/or finalize:
- `Cargo.toml`
- module tree
- `lib.rs`
- docs folder and required docs
- initial file/folder structure
- crate dependencies
- basic README framing

Do not over-implement here.
Set the foundation cleanly.

### Phase 2 — domain language and core types
Implement:
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
- `MarketConfig`

Make these clean and idiomatic before building engine behavior.

### Phase 3 — validation and error model
Implement:
- typed errors
- validation rules
- tick size checks
- lot size checks
- quantity/price sanity checks

Validation must exist before serious matching logic is added.

### Phase 4 — price level and book state
Implement:
- `PriceLevel`
- bid/ask storage
- top-of-book queries
- snapshot helpers
- empty-level cleanup rules

Do not build matching on top of vague state structures.

### Phase 5 — core matching behavior
Implement:
- resting limit orders
- crossing limit order logic
- partial fills
- market order logic
- cleanup after fills
- event emission
- structured result objects

### Phase 6 — cancellation flow
Implement:
- cancel by order ID
- cleanup behavior
- event emission for cancellation
- error handling for missing order IDs

### Phase 7 — tests
Implement and expand:
- unit tests
- integration tests
- invariant-oriented tests
- snapshot/top-of-book behavior tests

Do not leave tests as an afterthought.

### Phase 8 — example and finishing passes
Implement:
- `examples/basic_flow.rs`
- doc consistency pass
- naming consistency pass
- API consistency pass
- architecture cleanup pass

### Rule for moving between phases
Do not abandon a phase half-formed.
Get each phase to a coherent, production-worthy baseline before heavily expanding the next.

--------------------------------------------------
PHASE DEFINITION OF DONE
--------------------------------------------------

A phase is only considered complete when:
- the code for that phase is implemented coherently
- affected tests exist and pass
- docs affected by that phase are updated
- public APIs introduced in that phase are named cleanly
- no placeholder logic remains in that phase
- the phase leaves a stable foundation for the next phase

--------------------------------------------------
TESTING REQUIREMENTS
--------------------------------------------------

This project must include strong automated testing.

### Unit tests
Cover isolated components such as:
- FIFO behavior inside `PriceLevel`
- validation helpers
- best bid / best ask update logic
- event/state helper behavior
- top-of-book and snapshot helpers

### Integration tests
Cover full workflows such as:
- submit resting bid and ask orders
- crossing limit order matching
- partial fills
- cancel existing order
- cancel missing order
- market order sweep across multiple levels
- same-price FIFO execution
- better-price priority execution
- empty-book market order handling
- event emission sequence sanity
- best bid / best ask changes after trades/cancellations
- snapshot output after state changes
- invariant-oriented scenarios

Tests must be:
- deterministic
- readable
- behavior-focused
- production-quality

--------------------------------------------------
TESTING HIERARCHY
--------------------------------------------------

- use unit tests for isolated rules and helpers
- use integration tests for workflow behavior
- use invariant-oriented tests for state correctness
- do not overtest implementation details when behavior-level tests are stronger
- prefer clear scenario names over clever compact test code

--------------------------------------------------
DOCUMENTATION REQUIREMENTS
--------------------------------------------------

You must create and maintain:

- `README.md`
- `docs/architecture.md`
- `docs/glossary.md`
- `docs/milestones.md`
- `docs/technical-architecture.md`

### Glossary requirement
The glossary must be beginner-friendly, not just jargon.
For important terms, explain:
- what the term means
- why it matters in this engine
- a small example when useful

The glossary must include, at minimum:
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
- `NewOrderRequest`
- `Order`
- `OrderId`
- partial fill
- trade
- event
- event stream
- execution report
- `SubmissionResult`
- `CancelResult`
- summary helpers
- `TopOfBook`
- book snapshot
- `BookLevelView`
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

Docs must stay in sync with implementation.
Do not let docs rot while code evolves.

--------------------------------------------------
DOCUMENTATION DISCIPLINE
--------------------------------------------------

- documentation must stay aligned with implementation
- do not over-expand docs before the underlying code exists
- document architecture and behavior at the level the implementation has actually earned
- do not write speculative documentation for features not yet implemented

--------------------------------------------------
ARCHITECTURE DECISION LOGGING
--------------------------------------------------

When making a meaningful architectural choice that is not already obvious from the docs:
- record it briefly in the relevant doc or README
- explain why the choice was made
- prefer short, concrete rationale over long prose

--------------------------------------------------
EXAMPLE HONESTY
--------------------------------------------------

- examples must reflect real implemented behavior
- do not fake outputs in examples
- do not present hypothetical flows as if already implemented
- keep examples aligned with the actual public API

--------------------------------------------------
BUILD / ENV / DEPLOYMENT AWARENESS
--------------------------------------------------

Even though the engine core is in-memory and library-first, you must still think like a production backend engineer.

That means:
- keep `Cargo.toml` clean and intentional
- manage dependencies carefully
- keep configuration surfaces clear
- support environment/config handling where appropriate
- keep the project buildable and testable from a clean checkout
- avoid ad hoc file structure and inconsistent conventions

Do not bolt operational concerns on later conceptually.
Make the repo clean enough that operational concerns could be layered in without architectural cleanup.

--------------------------------------------------
PROGRESS DISCIPLINE
--------------------------------------------------

- do not stall on perfecting architecture beyond what the current phase needs
- make the current phase clean, correct, and extensible, then move forward
- prefer stable incremental progress over speculative perfection
- refine architecture when implementation pressure justifies it

--------------------------------------------------
CORE LIBRARY PURITY
--------------------------------------------------

- keep the matching engine core focused on domain behavior
- avoid mixing presentation concerns into core logic
- avoid CLI-specific, demo-specific, or logging-heavy behavior inside core engine code
- examples and adapters may format outputs, but core logic should remain clean and reusable

--------------------------------------------------
STATE TRANSITION CLARITY
--------------------------------------------------

- state mutations must be easy to follow in code
- avoid spreading one logical state transition across too many hidden helper layers
- prefer explicit mutation flow when it improves correctness and readability
- a reviewer should be able to trace order submission, matching, fill updates, and cleanup without guessing

--------------------------------------------------
ABSTRACTION DISCIPLINE
--------------------------------------------------

- do not introduce a generic abstraction until there is clear second-use pressure
- do not generalize code solely because future reuse is imaginable
- prefer concrete, explicit implementations in the first strong version
- generalize only when duplication or implementation pressure clearly justifies it

--------------------------------------------------
REVIEWABILITY
--------------------------------------------------

- write code that is easy to review, reason about, and audit
- prefer clarity in naming and control flow over density
- keep important business rules near the code that enforces them
- a strong reviewer should be able to understand the core matching logic without jumping through unnecessary layers

--------------------------------------------------
SCOPE PROTECTION
--------------------------------------------------

- do not prematurely build future layers such as networking, persistence, balances, settlement, or distributed components into the core
- finish the exchange-core library cleanly before extending into surrounding systems
- keep future extensibility in mind, but do not let future layers distort the current implementation

--------------------------------------------------
PERFORMANCE DISCIPLINE
--------------------------------------------------

- optimize for correctness and clarity first
- benchmark before making non-obvious performance changes
- do not introduce complexity in the name of speed without evidence
- prefer data-structure clarity in the first production-quality version unless measurement justifies further complexity

--------------------------------------------------
ANTI-DRIFT RULES
--------------------------------------------------

- do not silently redesign the architecture
- do not replace explicit domain modeling with generic abstractions
- do not introduce speculative interfaces or traits
- do not create dead code paths “for future flexibility”
- do not create patterns that look clean but hide the actual engine rules
- do not sacrifice inspectability for abstraction
- do not add hidden fallback behavior
- do not silently swallow invalid states
- do not silently coerce bad inputs
- do not silently degrade behavior

If you improve the architecture, the improvement must be:
- concrete
- justified
- simpler or more correct
- consistent across the codebase

--------------------------------------------------
COMMIT / CHANGE HYGIENE
--------------------------------------------------

- keep changes logically grouped
- keep diffs reviewable
- avoid unrelated churn
- do not refactor large unrelated areas without clear benefit
- maintain coherence across touched files
- keep tests and docs updated alongside implementation changes

--------------------------------------------------
CODE QUALITY GATES
--------------------------------------------------

Before considering a meaningful implementation step complete:

- `cargo fmt` should pass
- `cargo clippy` should be clean or clearly justified
- tests should pass for affected areas
- docs should remain aligned with implementation
- public APIs should remain coherent
- no unexplained warnings should be introduced

--------------------------------------------------
DECISION RULES
--------------------------------------------------

When choosing between:
- fast but sloppy
- slower but production-grade

always choose:
- production-grade

When choosing between:
- clever but fragile
- simple but robust

always choose:
- simple but robust

When choosing between:
- finishing a task
- implementing it correctly

always choose:
- implementing it correctly

When choosing between:
- over-abstracting early
- keeping a clean explicit design

always choose:
- clean explicit design

--------------------------------------------------
FINAL QUALITY BAR
--------------------------------------------------

At every step ask:

1. Does this match the reference architecture?
2. Is this idiomatic Rust?
3. Would a serious backend/systems engineer consider this production-grade?
4. Does this preserve determinism, clarity, and invariants?
5. Did I read the full context before changing it?
6. Am I building the system comprehensively rather than just finishing isolated tasks?
7. Are docs, tests, and architecture still aligned?

If the answer to any of these is no, fix it before moving on.
