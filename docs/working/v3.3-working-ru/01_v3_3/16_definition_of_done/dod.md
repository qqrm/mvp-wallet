# 16. Definition of Done (Backend)

Релиз не может быть выпущен, пока не выполнено:
- Реализованы /v1 endpoints: admin accounts create/close, fund, withdraw, reverse, search; transfers; spend simulate; fx quote/execute; history; tx receipt.
- OpenAPI опубликован и валидируется в CI; schema соответствует runtime.
- Idempotency на всех money POST; 409 IDEMPOTENCY_CONFLICT на payload mismatch.
- Инварианты леджера всегда проходят (double-entry sum=0; no negative available).
- Integration tests покрывают критические сценарии (P2P, idempotency replay/conflict, FX, spend, admin fund/withdraw/reverse).
- Observability: correlation_id; audit_event для каждого money/admin action; дашборды ошибок и latency.
- Persistence: Postgres для staging/prod; backups + restore drill; migrations gated in CI.

См. первоисточник: `00_source_pdf_text/page_032.md`.
