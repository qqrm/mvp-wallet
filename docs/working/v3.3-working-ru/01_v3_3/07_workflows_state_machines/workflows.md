# 07. Воркфлоу и state machines

P2P transfer:
1) Валидация: ACTIVE счета + достаточный баланс.
2) Постинг JournalTx: debit sender, credit receiver.

Admin fund (mint):
- debit SYSTEM_MINT, credit user.

Admin withdraw (MVP interpretation):
- debit user, credit SYSTEM_SINK.

Reverse:
- создать компенсирующий JournalTx, ссылающийся на оригинальный tx_id.

Spend simulate:
- debit user, credit SYSTEM_SPEND; можно добавить merchant_label/memo.

FX:
- quote: расчёт rate + fee; сохранение FxQuote с expires_at.
- execute: проверка quote active и не истёк; постинг multi-currency legs + fee legs.

State machines (type-state pattern):
- TxState: Created → Posted → Reversed
- FxQuoteState: Created → Active → Expired → Executed
- AccountState: Active → Closed

См. первоисточник: [SOURCE 1 §p009](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p009) и [SOURCE 1 §p017](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p017).
