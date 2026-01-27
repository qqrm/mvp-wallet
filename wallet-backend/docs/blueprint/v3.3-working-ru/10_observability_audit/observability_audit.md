# 10. Наблюдаемость и аудит

AuditEvent (append-only) обязателен для:
- admin fund/burn/reverse/close
- P2P
- FX execute
- spend simulate

Минимальный состав audit записи:
- actor_id, role
- ip/device
- correlation_id, request_id
- affected_account_ids
- tx_id (если применимо)
- (желательно) before/after balances

Метрики:
- posting latency
- idempotency conflicts
- insufficient funds rate
- FX quote expiry rate
- error codes distribution

См. первоисточник: `00_source_pdf_text/page_009.md` и `page_018.md`.
