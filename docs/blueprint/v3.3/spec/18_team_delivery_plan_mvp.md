# 18. TEAM & DELIVERY PLAN (MVP)

Minimum team (1–2 months)
- Tech Lead / Staff Engineer (ledger correctness, contracts, review gate)
- 2 Backend Engineers (ledger engine, API, Postgres, observability)
- Android Engineer
- iOS Engineer
- QA Engineer (E2E automation + release sign-off)
- 0.5 DevOps/SRE
- 0.25 Legal/Compliance

Estimated person-hours (MVP)
- Backend: 520–700
- Android: 280–360
- iOS: 280–360
- QA/E2E: 220–320
- DevOps: 120–180
- Compliance: 30–60

Critical path
- OpenAPI locked.
- Postgres schema + atomic posting + concurrency verified.
- Idempotency enforced.
- FX rounding + expiry correctness.
- E2E suite green; backups/restore ready.
