# HANDOFF_INSTRUCTIONS.md

Give the editor the files in this exact order:

1. `HANDOFF_INSTRUCTIONS.md`
2. `EXECUTION_PROMPT.md`
3. `MVP_SCOPE.md`
4. `PRD.md`
5. `TECHNICAL_ARCHITECTURE.md`
6. `docs_glossary.md`

Recommended handoff message:

Use the attached/reference docs as the source of truth.

Read them in this order:
1. HANDOFF_INSTRUCTIONS.md
2. EXECUTION_PROMPT.md
3. MVP_SCOPE.md
4. PRD.md
5. TECHNICAL_ARCHITECTURE.md
6. docs_glossary.md

Rules:
- Read every file in full before implementation.
- Treat `EXECUTION_PROMPT.md` as the behavioral protocol.
- Treat `TECHNICAL_ARCHITECTURE.md` as the highest-priority implementation spec.
- Use the priority order already defined in the execution prompt if there is tension between docs.
- Start with Phase 1 and proceed in order.
- Do not skip files you touch.
- Keep docs, tests, and implementation aligned as you build.

What to build:
A production-quality implementation of `solbook-core`, a deterministic Rust exchange-core library for `SOL/USDC`, following the architecture and standards in the attached docs.
