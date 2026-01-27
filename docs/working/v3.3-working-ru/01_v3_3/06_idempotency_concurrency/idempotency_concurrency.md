# 06. Идемпотентность и конкуренция

Идемпотентность:
- Клиент отправляет заголовок `Idempotency-Key` (UUID рекомендован).
- Сервер сохраняет: key, actor_id, endpoint, request_hash, status_code, response_body, created_at, ttl.
- Повтор с тем же request_hash возвращает сохранённый ответ + `Idempotency-Replayed: true`.
- Повтор с другим payload → `409 IDEMPOTENCY_CONFLICT`.

Retention (v3.2): TTL 48 часов для записей идемпотентности.

Конкурентность (locking/isolation):
- Любая операция, меняющая баланс, должна эксклюзивно блокировать затронутые аккаунты (`SELECT ... FOR UPDATE`) или выполняться в `SERIALIZABLE` транзакциях.
- Для операций с двумя аккаунтами (P2P) — блокировать в согласованном порядке (например, по отсортированному `user_id`/`account_id`) во избежание deadlocks.

См. первоисточник: [SOURCE 1 §p014](../../../../blueprint/v3.3/appendix/sources/source_materials.md) и [SOURCE 1 §p038](../../../../blueprint/v3.3/appendix/sources/source_materials.md).
