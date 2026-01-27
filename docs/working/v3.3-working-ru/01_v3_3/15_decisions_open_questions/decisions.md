# 15. Решения и открытые вопросы

Примеры вопросов (decision list):
- Какая юр. сущность оперирует леджером (bank vs payment org vs internal feature)?
- Валюты в MVP (UZS только vs UZS+USD)?
- Источник FX rate (в v3.2 зафиксировано: статическая таблица `fx_rates` в БД).
- Модель FX fee (flat/percent/markup-in-rate)?
- Retention policy для journal и audit logs.
- Governance admin операций (1-person vs 4-eyes).
- Аутентификация пользователя (Uzum SSO vs standalone OTP).
- Нужны ли holds в MVP (по умолчанию blocked=0).
- Когда нужен shared/family wallet (schema reserve wallet_group_id).

PDR (v3.2):
- Содержит Q&A с обоснованиями принятых решений (в частности TTL 48h, статический fx_rates, пагинация admin/search и т.п.).

См. первоисточник: `00_source_pdf_text/page_030.md`–`page_031.md` и `page_033.md`–`page_034.md`.
