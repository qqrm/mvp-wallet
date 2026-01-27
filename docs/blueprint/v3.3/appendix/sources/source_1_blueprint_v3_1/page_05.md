# SOURCE_1__Blueprint_v3_1

## Page 05

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 5
(2) PDF v2 AUDIT: GAPS & FIXES (STRICT)
Below are high-impact gaps observed in PDF v2 and how v3.1 fixes or requires fixes. Format: Category - Problem - Why it matters - Concrete fix - Priority.

## PDF v2 audit gaps and fixes (structured)

This section is a readable conversion of the original gap table. It preserves the content while improving navigation.

### Gap 1 — API/Contracts (P0)
- Problem: No locked OpenAPI contracts; endpoint semantics implied only.
- Concrete fix: Publish OpenAPI for all /v1 endpoints with schemas, examples, and error model.

### Gap 2 — Domain Model (P0)
- Problem: Ledger is described conceptually but invariants are not enforceable.
- Concrete fix: Define double-entry tables + check constraints (sum debits == sum credits per journal).

### Gap 3 — Reliability/Testing (P0)
- Problem: Idempotency is mentioned but not mandatory for all financial writes.
- Concrete fix: Require Idempotency-Key for all POST financial endpoints; enforce conflict behavior.

### Gap 4 — Security (P0)
- Problem: No admin safety boundary (who can mint/burn/reverse).
- Concrete fix: Role-based authz + admin audit log; optional 4-eyes approval flow.

### Gap 5 — Data (P0)
- Problem: No immutable journal rule; potential updates/deletes.
- Concrete fix: Append-only journal; reversals only via compensating postings.

### Gap 6 — Backend Logic (P0)
- Problem: No explicit transaction lifecycle states.
- Concrete fix: Define Tx state machine: CREATED -> POSTED -> REVERSED; apply type-state.

### Gap 7 — Backend Logic (P0)
- Problem: FX is not contractually specified (rates, rounding, expiry).
- Concrete fix: Implement quote->execute with expiry, rounding policy, and fee calculation.P0 8 Product Withdraw/topup scope is unclear (external rails vs internal admin ops). Define admin fund/burn as internal ledger operations; external rails out of MVP.

### Gap 9 — Data (P0)
- Problem: Balance model (available vs blocked) unclear for MVP.
- Concrete fix: Define available as ledger_posted - blocked; blocked=0 in MVP unless holds feature enabled.

### Gap 10 — NFR/Operations (P1)
- Problem: DB choice not aligned with fintech requirements (SQLite risk).
- Concrete fix: Use PostgreSQL in production; SQLite only for dev/tests; add migration plan.P0 11 Observability No correlation IDs across requests and ledger journal. Require request_id/correlation_id; store in journal + audit logs; propagate in tracing.

### Gap 12 — Compliance (P1)
- Problem: Uzbekistan data localization / personal data obligations not translated to controls.
- Concrete fix: Add data register, retention rules, access logging, and locality assumptions.P1 13 UI/UX Wallet history UX not specified (filters, receipts). Define transaction list filters, receipt layout, and error states.

### Gap 14 — Backend Logic (P0)
- Problem: No admin correction strategy.
- Concrete fix: Add /admin/reverse that posts compensating journal entries referencing original tx.

### Gap 15 — API/Contracts (P0)
- Problem: No stable error envelope; ad-hoc error strings.
- Concrete fix: Define {error:{code,message,details}}; list mandatory codes + HTTP mapping.

### Gap 16 — Reliability/Testing (P1)
- Problem: No property-based invariant tests.
- Concrete fix: Add proptest/quickcheck invariants: no money created, double-entry sum zero, idempotency replay.

### Gap 17 — Roadmap (P1)
- Problem: Shared/family wallets not framed as future capability.
- Concrete fix: Reserve tenant/wallet group abstraction in schema; keep MVP single-owner.P2 18 Security No rate limits / abuse controls for transfer APIs. Implement per-user and per-IP throttles; add velocity limits.

### Gap 19 — Reliability (P2)
- Problem: No job/outbox pattern for async reconciliation.
- Concrete fix: Add outbox table for future integrations; not required to run in MVP.

### Gap 20 — Data (P1)
- Problem: No deterministic ordering/pagination in history endpoints.
- Concrete fix: Cursor-based pagination by (created_at, tx_id).

### Gap 21 — Product (P0)
- Problem: Merchant flow undefined.
- Concrete fix: Explicitly defer merchant integration; keep sandbox spend simulation only. P1 22 API/Contracts Account lifecycle rules missing (close/suspend semantics). Define closed account behavior and allow only admin corrections.

### Gap 23 — Security (P1)
- Problem: No secrets management baseline.
- Concrete fix: Store secrets in vault/KMS; rotate keys; use env var only in dev.

### Gap 24 — NFR/Operations (P0)
- Problem: No backup/restore plan.
- Concrete fix: Daily backups + PITR; restore drills; migration gating in CI.
