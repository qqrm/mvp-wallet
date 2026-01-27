# Wallet Backend — Roadmap to Production

This roadmap defines the evolution from MVP
to a **production-grade Wallet Core**.

Each item must be delivered as a small, reviewable PR
with tests and migrations.

---

## Guiding principles

1. Money is critical:
   - atomicity
   - idempotency
   - immutability
   - strict invariants

2. Backward-compatible API when possible,
   but correctness beats convenience.

3. Every PR:
   - has tests,
   - runs on a clean DB,
   - passes build + clippy + test.

4. No build-time downloads.

---

## EPIC A — Money model hardening

### A1. Currency minor-units
- Add currency registry:

currency_code, minor_units

- Validate `AmountMinor` using currency rules.
- Document rounding explicitly.

DoD:
- invalid precision rejected,
- rounding tests.

---

### A2. Ledger & balance invariants
- Formal invariant:

sum(entries per tx_id) == 0

- Projection invariant:

available + hold == wallet-side sum


DoD:
- proptest 1000+ random flows,
- no invariant violations.

---

### A3. Partial capture / refund (optional)
- Allow partial operations.
- Forbid exceeding limits.

DoD:
- state machine tests,
- double-spend prevention.

---

## EPIC B — Bank-grade state machine

### B0. Single-transaction money ops (ACID)
- Reserve HTTP idempotency + apply ledger effects + finalize in ONE DB transaction.
- No partial commits: rollback everything on error.

DoD:
- integration tests cover retry/crash safety,
- no duplicate side effects under retries.


### B1. Explicit FSM in DB
- `ledger_transactions.status`
- DB-enforced transitions.

DoD:
- impossible invalid transitions.

---

### B2. HTTP Idempotency MVP (no response cache)
- Store request hash + status + lease TTL + result_tx_id.
- Replay by loading ledger data using result_tx_id (no JSON response cache).
- Reject key reuse with different payload via 409 Conflict.

DoD:
- same key -> same tx_id + same effect,
- payload mismatch -> 409 Conflict,
- parallel duplicates apply once.

---

### B3. Concurrency & isolation
- Retry on SQLITE_BUSY (SQLite).
- In Postgres: use row-level locking / SERIALIZABLE where needed for hot accounts.

DoD:
- concurrent transfer tests.

---

## EPIC C — Outbox & integrations

### C1. Outbox worker
- Background delivery.
- Retry with backoff.
- Outbox rows MUST be written in the same DB transaction as money ops (even if the worker is disabled).

DoD:
- at-least-once delivery.

---

### C2. Event contracts
- event_type
- version
- aggregate_id
- tx_id
- correlation_id

DoD:
- documented JSON schemas.

---

### C3. Inbox (external events)
- Dedup external events.
- Idempotent consumption.

---

## EPIC D — Security & audit

### D1. Proper auth
- JWT / API keys.
- RBAC roles.

---

### D2. Audit trail
- audit_log table.
- Atomic with money ops.

---

### D3. Rate limiting
- Admin endpoints.
- Abuse prevention.

---

## EPIC E — Production storage

### E1. Postgres support
- Feature flags.
- Docker compose.

---

### E2. Constraints & indexes
- FK, CHECK, UNIQUE.
- Query performance.

---

## EPIC F — Observability

### F1. Structured logs
- correlation_id everywhere.

### F2. Metrics
- ops_total
- ops_failed
- db_latency

### F3. Health
- /healthz
- /readyz

---

## EPIC G — API polish

### G0. API versioning
- All routes are versioned from day one: /v1/...
- Any breaking change -> /v2/... (never silently break mobile clients).

---

### G1. OpenAPI completeness
- examples
- error models

### G2. Error contract + taxonomy
- Replace `{error: string}` with structured `{code, message}`.
- Stable error codes:

INSUFFICIENT_FUNDS
INVALID_STATE
DUPLICATE_REQUEST
IDEMPOTENCY_CONFLICT

- HTTP mapping must be consistent (e.g. 409 for idempotency conflicts).


### G3. README
- how to run
- env vars
- curl examples

---

## Priority order

Strict execution order:

1. EPIC B (FSM + idempotency)
2. EPIC D (security + audit)
3. EPIC C (outbox)
4. EPIC E (postgres)
5. EPIC F (observability)
6. EPIC A (money model)
7. EPIC G (docs & polish)

---

## Definition of Done for every PR

A PR is complete only if:

- build passes
- clippy passes
- tests pass
- migrations are append-only
- invariants preserved
- no build-time downloads
