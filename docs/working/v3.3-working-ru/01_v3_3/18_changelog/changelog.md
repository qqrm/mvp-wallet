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

См. первоисточник: `00_source_pdf_text/page_004.md` и `page_033.md`–`page_034.md`.
