# 03. Доменная модель и инварианты

Ключевые сущности (v3.1 + уточнения v3.2):
- Tenant (резерв под multi-tenant)
- User (user_id, phone/email, status)
- Account (per-currency, ACTIVE/CLOSED)
- JournalTx (заголовок транзакции; immutable после POSTED)
- JournalEntry (строки двойной записи)
- Balance cache (производная оптимизация чтения)
- FxQuote (quote lifecycle + expiry; снимок условий)
- FxRate (v3.2: статическая таблица курсов в БД)
- IdempotencyRecord (TTL 48 часов в v3.2)
- AuditEvent (append-only аудит)

Инварианты:
1) Неизменяемость: после POSTED любые update/delete запрещены; исправления только через компенсирующие проводки (reversal).
2) Double-entry: для каждого JournalTx и валюты сумма дебетов == сумма кредитов.
3) Доступный баланс не уходит в минус (овердрафт вне MVP).
4) Все изменения денег — внутри одной транзакции БД.
5) Трассировка: correlation_id / request_id обязателен.

См. первоисточник: [SOURCE 1 §p008](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p008) и [SOURCE 1 §p035](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p035)–[SOURCE 1 §p036](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p036).
