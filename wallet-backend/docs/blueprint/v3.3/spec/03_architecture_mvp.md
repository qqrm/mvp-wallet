# 3. ARCHITECTURE (MVP)

3.1 Shape
- Contract-first monolith service (Rust)
- PostgreSQL in production (required)
- SQLite allowed only for local dev/tests (never production)

3.2 Consistency
- All money operations execute inside a single DB transaction.
- Ledger posting must be atomic: either fully posted or fully rolled back.

3.3 Observability
- Every request MUST have request_id / correlation_id.
- correlation_id MUST be stored in:
  - journal_tx
  - audit_event
  - idempotency record

3.4 Time
- All timestamps are stored in UTC.
- API may present ISO-8601 timestamps.
