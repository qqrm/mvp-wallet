# 05. API контракты (contract-first)

Принципы:
- Версионирование: `/v1` (breaking changes → `/v2`)
- Auth: `Authorization: Bearer <JWT>`; ADMIN роль по claims
- Идемпотентность: `Idempotency-Key` обязателен на money-mutating POST
- Пагинация: cursor-based для list-эндпоинтов (в v3.2 есть уточнения по admin/search)

Admin API (MVP):
| Method | Endpoint | Purpose | Idempotent |
|---|---|---|---|
| POST | /v1/admin/accounts | Create account for user/currency | Да (рекомендуется) |
| POST | /v1/admin/accounts/{id}/close | Close/suspend account | Да |
| POST | /v1/admin/fund | Mint funds via SYSTEM_MINT | Да (обязательно) |
| POST | /v1/admin/withdraw | Burn funds to SYSTEM_SINK | Да (обязательно) |
| POST | /v1/admin/reverse | Compensating reversal | Да (обязательно) |
| GET | /v1/admin/search | Lookup by filters | N/A |

User Wallet API:
| Method | Endpoint | Purpose | Idempotent |
|---|---|---|---|
| GET | /v1/profile | Profile + default wallet/account ids | N/A |
| GET | /v1/accounts/{id} | Account details | N/A |
| GET | /v1/accounts/{id}/balance | Balance breakdown | N/A |
| GET | /v1/accounts/{id}/transactions | History (cursor) | N/A |
| GET | /v1/transactions/{tx_id} | Receipt/details | N/A |
| POST | /v1/transfers | P2P transfer | Да (обязательно) |
| POST | /v1/spend/simulate | Sandbox spend | Да (обязательно) |
| POST | /v1/fx/quote | Create FX quote | Да (рекомендуется) |
| POST | /v1/fx/execute | Execute FX quote | Да (обязательно) |

Уточнения v3.2:
- Recipient addressing: recipient может задаваться `phone_number` или `user_id` (в дополнение к `account_id`), с резолвом в `account_id` на момент операции.
- Admin search filters: `user_id`, `account_id`, `tx_id`, `currency`, `time_range`, до 100 результатов; пагинация может быть на фронте (без server-side cursor в MVP).

См. первоисточник: [SOURCE 1 §p012](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p012)–[SOURCE 1 §p015](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p015) и [SOURCE 1 §p033](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p033)–[SOURCE 1 §p037](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p037).
