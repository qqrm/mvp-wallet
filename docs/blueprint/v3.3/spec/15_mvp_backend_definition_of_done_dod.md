# 15. MVP BACKEND DEFINITION OF DONE (DoD)

Release cannot ship until:
- /v1 endpoints implemented: admin accounts create/close, fund, withdraw, reverse, search; transfers; spend simulate; FX quote/execute; history; transaction receipt.
- OpenAPI published and validated in CI; schema matches runtime behavior.
- Idempotency enforced for all money POST endpoints; return 409 IDEMPOTENCY_CONFLICT on payload mismatch.
- Ledger invariants always pass (double-entry; no negative available; immutable postings).
- Integration/E2E tests cover critical scenarios (P2P, idempotency replay/conflict, FX, spend, admin fund/withdraw/reverse).
- Observability: correlation_id present and stored; audit_event written for every money/admin action; dashboards for errors and latency are available.
- Persistence/ops: Postgres for staging/prod; backups + PITR; restore drill executed; migrations gated in CI; runbooks exist for incident and balance discrepancy investigation.
