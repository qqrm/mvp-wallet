# 7. API CONTRACTS (MVP)

7.1 General
- All endpoints must be defined in OpenAPI (schemas + examples + errors).
- Idempotency-Key is REQUIRED for POST endpoints that commit state changes (money postings + admin ops).
  - Exception: POST /v1/fx/quote is intentionally NOT idempotent (fresh quote per call).

7.2 Error Envelope (mandatory, stable)
All errors MUST be returned as:
```json
{
  "error": {
    "code": "SOME_CODE",
    "message": "Human readable",
    "details": { ... optional ... }
  }
}
```

7.3 Endpoints (MVP)
USER:
- GET  /v1/profile
- GET  /v1/accounts
- GET  /v1/accounts/{account_id}
- GET  /v1/accounts/{account_id}/balance
- GET  /v1/accounts/{account_id}/transactions?before=&limit=   (default limit=100; sorted desc by created_at)
- GET  /v1/transactions/{tx_id}
- POST /v1/transfers
- POST /v1/fx/quote
- POST /v1/fx/execute
- POST /v1/spend/simulate

ADMIN:
- POST /v1/admin/accounts (create)
- POST /v1/admin/accounts/{account_id}/close
- POST /v1/admin/fund
- POST /v1/admin/withdraw
- POST /v1/admin/reverse
- GET  /v1/admin/search
