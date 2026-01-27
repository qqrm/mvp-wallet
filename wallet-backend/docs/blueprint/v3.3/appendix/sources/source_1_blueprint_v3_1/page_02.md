# SOURCE_1__Blueprint_v3_1

## Page 02

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 2
(1) EXECUTIVE SUMMARY
PDF v2 is a solid conceptual overview for a multi-currency wallet ledger (double-entry, holds/capture ideas), but it is not implementation-complete for fintech. The
largest missing pieces were locked API contracts, a strict admin safety model, and enforcement of invariants at the DB + transaction level.
v3.1 converts the concept into a contract-first MVP blueprint aligned to the current backend direction: admin helpers (create/close/fund/withdraw/reverse), P2P
transfers, sandbox spend, and FX quote/execute. KYC is explicitly deferred.
Biggest risks for MVP
- Ledger correctness & concurrency: without enforced constraints + atomic postings, balances will diverge and cannot be repaired safely.
- Missing idempotency enforcement on all write endpoints leads to duplicate credits/debits under retries (irreversible support debt).
- Admin operations without role boundaries and compensating corrections create insider-risk and audit gaps.
- FX without defined rounding + quote expiry semantics will leak money and break reconciliation.
- SQLite in production for financial ledger is operationally fragile; PostgreSQL migration is mandatory before launch.
Top 10 improvements that move the needle
- Lock OpenAPI contracts + stable error envelope + idempotency semantics for every financial POST endpoint.
- Adopt PostgreSQL schema with DB-enforced invariants (unique keys, foreign keys, check constraints) and transaction isolation rules.
- Implement immutable journal + double-entry postings; forbid destructive deletes for any financial objects.
- Add Admin API with explicit roles and compensating reversal endpoint (no edits to historical ledger rows).
- Introduce state machines (type-state pattern) for Tx lifecycle and FX quote/execute lifecycle.
- Build automated E2E suite for critical journeys + invariant property tests for ledger correctness.
- Add audit log (append-only) for all admin and financial actions with correlation IDs.
- Add minimal fraud/abuse controls: rate limits, device/session controls, admin 4-eyes approval hook (optional).
- Define balance model precisely (available/blocked/ledger) and rounding policy per currency.
- Define a production-ready ops baseline: backups, migrations, alerting, and incident playbook.
Recommended MVP scope boundary
IN MVP (ship in ~1 month)
OUT OF MVP (explicitly deferred)
