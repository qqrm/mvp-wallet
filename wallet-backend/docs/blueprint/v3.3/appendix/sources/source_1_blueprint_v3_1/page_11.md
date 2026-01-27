# SOURCE_1__Blueprint_v3_1

## Page 11

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 11
3.16 Persistence Strategy (MVP)
MVP allows SQLite for local development and automated tests, but production must run on PostgreSQL. This is non-negotiable for operational reliability (locking,
backups, PITR, migrations).
- Dev/Test: SQLite (fast iteration, in-memory tests).
- Staging/Prod: PostgreSQL 15+ (required).
- Migration plan: use SQL migrations (e.g., sqlx/sea-orm/diesel migrations) and run a one-time data export/import tool before launch.
- Release gate: no production launch until Postgres E2E suite is green.
3.17 Database schema (MVP - ledger core)
| Table | Purpose | Key fields (minimal) |
| --- | --- | --- |
| tenants | Future multi-tenant boundary | tenant_id, name, status |
| users | Maps Uzum user identity | user_id, tenant_id, phone/email, status |
| accounts | Per-currency ledger account | account_id, user_id, currency, status, created_at |
| journal_tx | Immutable journal header | tx_id, type, status, correlation_id, created_at, reversed_tx_id? |
| journal_entry | Double-entry lines | entry_id, tx_id, account_id, currency, amount, direction, meta_json |
| balance_cache | Fast balances (derived) | account_id, currency, ledger, blocked, available, updated_at |
| idempotency | Idempotency storage | key, actor_id, endpoint, request_hash, response_json, status_code, created_at |
| fx_quote | Quote lifecycle | quote_id, from_account_id, to_account_id, amount_from, rate, fee, expires_at, status |
| audit_event | Append-only audit | event_id, actor_id, role, action, target_ids, tx_id?, created_at, ip/device |
