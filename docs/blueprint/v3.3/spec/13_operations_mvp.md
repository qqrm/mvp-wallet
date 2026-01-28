# 13. OPERATIONS (MVP)

Database and durability
- Production requires PostgreSQL (SQLite only for dev/tests).
- Daily backups + PITR.
- Restore drills required before launch.
- DB migrations gated in CI.
- Atomic posting: JournalTx + JournalEntry (+ optional balance_cache update) committed in a single DB transaction.

Concurrency / isolation
- Choose and document the strategy (e.g., SERIALIZABLE vs explicit row locks). The choice must be validated under load.

Audit and admin operations
- FX rate and FX fee policy changes are admin operations and must be audited (who/when/what).
- AuditEvent (append-only) is required for: admin fund/burn/reverse/close, P2P, FX execute, spend simulate.
- Minimum audit fields:
  - actor_id, role
  - ip/device
  - correlation_id, request_id
  - affected_account_ids
  - tx_id (when applicable)
  - (optional) before/after balances

Operational metrics (minimum)
- posting latency
- idempotency conflicts
- insufficient funds rate
- FX quote expiry rate
- error codes distribution

Outbox
- Reserve an outbox table for future external integrations; it may remain inactive in MVP.
