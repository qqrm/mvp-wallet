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

См. первоисточник: [SOURCE 1 §p009](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p009) и [SOURCE 1 §p018](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p018).
