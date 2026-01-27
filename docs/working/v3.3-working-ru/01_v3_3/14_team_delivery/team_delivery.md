# 14. Команда и план поставки

Минимальная команда (1–2 месяца):
- Tech Lead / Staff engineer (ledger correctness + contracts + code review gate)
- 2 Backend (ledger engine, API, Postgres, observability)
- Android + iOS
- QA (E2E automation + release sign-off)
- 0.5 DevOps/SRE
- 0.25 Legal/Compliance

Оценка person-hours (MVP):
- Backend: 520–700
- Android: 280–360
- iOS: 280–360
- QA/E2E: 220–320
- DevOps: 120–180
- Compliance: 30–60

Critical path:
- OpenAPI locked
- Postgres schema + atomic posting + concurrency verified
- Idempotency enforced
- FX rounding + expiry correctness
- E2E suite green + backups/restore ready

См. первоисточник: [SOURCE 1 §p026](../../../../blueprint/v3.3/appendix/sources/source_materials.md)–[SOURCE 1 §p027](../../../../blueprint/v3.3/appendix/sources/source_materials.md).
