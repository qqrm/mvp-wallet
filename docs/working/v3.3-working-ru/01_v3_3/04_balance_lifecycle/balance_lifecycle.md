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

См. первоисточник: `00_source_pdf_text/page_008.md`–`page_009.md` и `page_036.md`.
