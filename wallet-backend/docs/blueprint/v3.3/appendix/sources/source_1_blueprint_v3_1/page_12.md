# SOURCE_1__Blueprint_v3_1

## Page 12

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 12
(4) API CONTRACT RECOMMENDATION (CONTRACT-FIRST)
API principles
- Versioning: /v1 path; breaking changes require /v2.
- Auth: Authorization: Bearer <JWT>; ADMIN role enforced via claims.
- Idempotency: Idempotency-Key required on money-mutating POST endpoints.
- Pagination: cursor-based; order by (created_at DESC, tx_id DESC).
- Error envelope: stable JSON error format with machine-readable codes.
Admin API (MVP required)
| Method | Endpoint | Purpose | Idempotent |
| --- | --- | --- | --- |
| POST | /v1/admin/accounts | Create account for user/currency | Yes (recommended) |
| POST | /v1/admin/accounts/{id}/close | Close/suspend account (no hard delete) | Yes |
| POST | /v1/admin/fund | Mint funds to user account via SYSTEM_MINT | Yes (required) |
| POST | /v1/admin/withdraw | Burn funds to SYSTEM_SINK (MVP withdraw) | Yes (required) |
| POST | /v1/admin/reverse | Compensating reversal of existing tx | Yes (required) |
| GET | /v1/admin/search | Lookup accounts/tx by filters | N/A |
User Wallet API
| Method | Endpoint | Purpose | Idempotent |
| --- | --- | --- | --- |
| GET | /v1/profile | User profile + default wallet/account ids | N/A |
| GET | /v1/accounts/{id} | Account details (status, currency) | N/A |
| GET | /v1/accounts/{id}/balance | Balance breakdown: available/blocked/ledger | N/A |
| GET | /v1/accounts/{id}/transactions?cursor=&limit= | Transaction history | N/A |
| GET | /v1/transactions/{tx_id} | Transaction receipt/details | N/A |
| POST | /v1/transfers | P2P transfer between accounts | Yes (required) |
| POST | /v1/spend/simulate | Sandbox spend: transfer to SYSTEM_SPEND | Yes (required) |
| POST | /v1/fx/quote | Create FX quote (2-step) | Yes (recommended) |
