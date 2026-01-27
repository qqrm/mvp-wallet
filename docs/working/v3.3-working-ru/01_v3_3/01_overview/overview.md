# 01. Обзор (Executive Summary)

Этот документ — план MVP Uzum Wallet с акцентом на банковскую корректность: неизменяемый журнал (immutable journal), атомарные проводки (atomic postings) и строгая идемпотентность на всех финансовых POST-эндпоинтах.

Границы MVP:
- Балансы кошелька и бухгалтерская книга двойной записи (double-entry ledger)
- Внутренние P2P переводы
- FX конвертация в два шага: quote + execute
- Админ-операции над леджером: mint / burn / reverse
- История транзакций + квитанции (receipt)
- Песочница: spend simulation

Вне MVP явно:
- KYC
- Внешние рельсы (карты, пополнения/выводы через банки)
- Merchant checkout интеграция
- Автоматизация чарджбеков/диспутов, кредит/страхование

Первичный источник: см. [SOURCE 1 §p001](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p001) и [SOURCE 1 §p002](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p002).
