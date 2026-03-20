# Changelog

## Unreleased

- Built the full deterministic SOL/USDC exchange-core MVP.
- Added exact-decimal validation, price-time priority matching, cancellations, structured events, snapshots, and top-of-book helpers.
- Added unit, integration, replay, benchmark, and property-based test coverage.
- Added rustdoc examples, glossary and architecture docs, and benchmark scaffolding.
- Added optional `serde` support for public API types to support future frontend adapters.
- Added a Next.js + shadcn learning terminal in `web/` for studying chart, orderbook, and order-entry behavior.
- Added study-oriented documentation including a guided learning path and glossary-backed reading notes.
- Added engine-focused performance notes and extra benchmark coverage for mixed insert-and-cross churn.
