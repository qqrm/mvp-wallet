# AGENT.md — Wallet Backend

This file defines how the agent must operate.
It is not a product spec.

## Scope
The agent:
- Implements small, incremental PRs.
- Never rewrites architecture unless explicitly instructed.
- Never modifies existing migrations.
- Always preserves backward-compatible API.

## Mandatory quality gate

Before returning any result, the agent MUST:

### 1. Build
Run:
  cargo build

Must succeed without network access.

### 2. Format
Run:
  cargo fmt --check
If fails, apply:
  cargo fmt

### 3. Lints
Run:
  cargo clippy -- -D warnings
No warnings allowed.

### 4. Tests
Run:
  cargo test
All tests must pass.

### 5. Migrations
If DB schema changes:
- Only append new migration files.
- Never edit existing migrations.
- Tests must pass on a fresh database.

## Change policy

- Changes must be minimal and reviewable.
- Prefer adding new code over modifying working logic.
- Never introduce TODOs or commented-out code.
- Never download anything at build time.

## Output format

For each task, the agent returns:
- Summary of changes.
- List of files modified.
- Tests added/updated.
- Confirmation that all quality gates passed.

## Money operations: Typestate Flow (Type-State Pattern) is mandatory

### Policy
All money-moving operations MUST be implemented via typestate-based flow structs (type-state pattern), not by manually composing ad-hoc SQL/updates inside service functions.

This includes:
- topup
- transfer
- hold authorize
- hold capture
- hold cancel
- refund

### Rationale
Typestate flows enforce a strict, reviewable sequence of steps:
1) validate + load required state
2) insert ledger transaction row
3) insert double-entry ledger entries
4) apply balance projection deltas (checked/atomic where required)
5) write outbox event
6) return domain response + result_tx_id

This is critical for fintech correctness (atomicity, invariants, and avoidance of missing steps).

### Rules
- A flow MUST NOT open/commit/rollback a SQL transaction.
- A flow MUST operate on `&mut Transaction<'_, Sqlite>` passed from the orchestrator.
- The orchestrator controls the transaction boundary (e.g. `idempotency_http::execute`).
- Each flow returns `(ResponseBody, ResultTxId)` where `ResultTxId` is the ledger tx identifier to be stored in HTTP idempotency.
- Any change to a money flow MUST include tests validating:
  - idempotency (same key -> same effect)
  - balance invariants (available/hold correctness)
  - ledger invariants (double-entry correctness, zero-sum)
  - concurrency safety where applicable

### Prohibited
- Rewriting typestate flows into manual inline SQL sequences in `service.rs`.
- Splitting money effects across multiple independent transactions.
- Skipping outbox writes or projection updates “temporarily”.

### Review checklist
PRs touching money logic must demonstrate:
- single-transaction atomicity for reserve + money effects + finalize idempotency
- no response caching in MVP idempotency (replay by `result_tx_id`)
- preserved typestate flow boundaries and ordering guarantees
- added/updated unit/integration/e2e tests
