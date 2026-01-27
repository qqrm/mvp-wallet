# SOURCE_1__Blueprint_v3_1

## Page 26

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 26
(7) TEAM / PERSON-HOURS / DELIVERY PLAN (MVP REALITY CHECK)
Minimal team composition (1-2 month MVP)
- 1x Tech Lead / Staff engineer (ledger correctness + contracts + code review gate).
- 2x Backend engineers (ledger engine, API, Postgres migration, observability).
- 1x Android engineer + 1x iOS engineer (Uzum UI Kit, screens, API integration).
- 1x QA engineer (E2E automation + regression + release sign-off).
- 0.5x DevOps/SRE (infra, CI/CD, migrations, monitoring, backups).
- 0.25x Legal/Compliance advisor (requirements review + regulator alignment).
Person-hours estimate (brutal MVP numbers)
| Track | Scope | Person-hours (MVP) |
| --- | --- | --- |
| Backend | Ledger, idempotency, P2P, admin ops, FX quote/execute, history/receipts, audit logs | 520 - 700 |
| Mobile (Android) | Home, transfer, FX, history, settings, error handling, QA fixes | 280 - 360 |
| Mobile (iOS) | Same as Android | 280 - 360 |
| QA/E2E | E2E suite (>=10), contract tests, regression, test data harness | 220 - 320 |
| DevOps | Postgres setup, migrations, secrets, monitoring, backups, staging/prod deploy | 120 - 180 |
| Compliance support | Data/privacy/AML baseline review + documentation | 30 - 60 |
| Critical path (what blocks release) | - OpenAPI locked + client integration stable (no moving targets). | - PostgreSQL ledger schema + atomic posting engine + concurrency strategy verified. |
| - Idempotency middleware enforced on all money endpoints. | - FX rounding + quote expiry correctness (no silent losses). | - E2E suite green on staging with seeded data; rollback + backup/restore ready. |
What we can cut if schedule is impossible
- Cut sandbox spend screen (keep endpoint for internal testing only).
