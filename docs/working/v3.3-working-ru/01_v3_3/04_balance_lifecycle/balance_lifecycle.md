# 04. Балансная модель и жизненный цикл

Компоненты баланса:
- ledger_posted: сумма подтверждённых проводок (immutable)
- blocked: в MVP = 0 (holds вне MVP)
- available = ledger_posted - blocked

Жизненный цикл транзакции (Tx):
- CREATED → POSTED → REVERSED
REVERSED реализуется как новая JournalTx, ссылающаяся на оригинал (reversed_tx_id), без редактирования истории.

Жизненный цикл аккаунта:
- ACTIVE → CLOSED
CLOSED запрещает новые дебеты/кредиты, кроме админ-коррекций/реверсов (по правилам продукта).

Системные счета (MVP):
- SYSTEM_MINT
- SYSTEM_SINK
- SYSTEM_SPEND
- SYSTEM_FX_POOL_[CCY]
- SYSTEM_FX_FEE_[CCY]

См. первоисточник: [SOURCE 1 §p008](../../../../blueprint/v3.3/appendix/sources/source_materials.md)–[SOURCE 1 §p009](../../../../blueprint/v3.3/appendix/sources/source_materials.md) и [SOURCE 1 §p036](../../../../blueprint/v3.3/appendix/sources/source_materials.md).
