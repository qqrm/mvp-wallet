# 18. Changelog

v3 → v3.1:
- Withdraw (MVP): admin burn to SYSTEM_SINK
- account delete → close/suspend; hard delete запрещён
- список admin/user endpoints
- spend simulate как first-class endpoint
- FX: quote + execute, expiry + rounding
- Postgres required for production; SQLite только dev/tests
- error contract + error codes + HTTP mapping
- Backend DoD

v3.1 → v3.2:
- P2P recipient: phone_number/user_id (в дополнение к account_id) + порядок lock для предотвращения deadlocks
- Admin search filters + limit 100 + пагинация на фронте
- FX rate source: статическая `fx_rates` таблица в БД
- Idempotency TTL: 48 часов
- Receipt schema: более полный breakdown
- Добавлен Product Decision Record (PDR)
- Расширен post-MVP roadmap

См. первоисточник: [SOURCE 1 §p004](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p004) и [SOURCE 1 §p033](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p033)–[SOURCE 1 §p034](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p034).
