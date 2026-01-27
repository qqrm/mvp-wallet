# 11. Надёжность и эксплуатация

Atomicity:
- JournalTx + JournalEntry + (опц.) balance_cache update — в одном commit.

Outbox:
- таблица зарезервирована для будущих интеграций (external rails), в MVP может быть неактивна.

DB:
- В production обязателен PostgreSQL (SQLite только dev/tests).
- Backups + PITR, restore drills.
- Решение по isolation: SERIALIZABLE или row locks; документировать.

Load/perf (минимум):
- transfers steady (например, 50 rps 10 минут; p95 < 200ms)
- FX burst
- lock contention test

См. первоисточник: `00_source_pdf_text/page_011.md` и `page_039.md`.
