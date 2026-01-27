# 15. MVP BACKEND DEFINITION OF DONE (DoD)

Release cannot ship until:
- All endpoints implemented under /v1 per section 7.3
- OpenAPI published + validated in CI (schema == runtime)
- Idempotency enforced for all money POST endpoints (409 on payload mismatch)
- Ledger invariants always pass (double-entry, immutable postings)
- Receipt endpoints correct for all flows
- Observability: correlation_id present and stored; audit_event written for every money/admin action
- Persistence: Postgres staging/prod; backups + restore drill executed; migrations gated in CI

END OF MVP SPEC (v3.3)
