# 19. CHANGELOG

v3 → v3.1
- Withdraw (MVP): admin burn to SYSTEM_SINK.
- Account delete → close/suspend; hard delete forbidden.
- Admin/user endpoints list.
- Spend simulate as a first-class endpoint.
- FX: quote + execute, expiry + rounding.
- PostgreSQL required for production; SQLite only for dev/tests.
- Error contract + error codes + HTTP mapping.
- Backend Definition of Done.

v3.1 → v3.2
- P2P recipient: phone_number/user_id (in addition to account_id) + lock ordering to prevent deadlocks.
- Admin search filters + limit 100 + frontend pagination.
- FX rate source: static fx_rates table in DB.
- Idempotency TTL: 48 hours.
- Receipt schema: more complete breakdown.
- Added Product Decision Record (PDR).
- Expanded post-MVP roadmap.
