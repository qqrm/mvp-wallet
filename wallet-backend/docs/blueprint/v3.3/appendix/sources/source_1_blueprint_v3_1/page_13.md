# SOURCE_1__Blueprint_v3_1

## Page 13

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 13
POST
/v1/fx/execute
Execute FX quote (posts ledger)
Yes (required)
Error model and codes
Error envelope (all errors):
```json
{
  "error": {
    "code": "INSUFFICIENT_FUNDS",
    "message": "Not enough available balance",
    "details": { "account_id": "acc_...", "available": 120000, "required": 200000 }
  }
}
```
Mandatory error codes (MVP):
- INSUFFICIENT_FUNDS
- INVALID_STATE
- ACCOUNT_CLOSED
- IDEMPOTENCY_REPLAY
- IDEMPOTENCY_CONFLICT
- VALIDATION_ERROR
- NOT_FOUND
- UNAUTHORIZED / FORBIDDEN
HTTP mapping:
- 400 - VALIDATION_ERROR
- 401 - UNAUTHORIZED
- 403 - FORBIDDEN
- 404 - NOT_FOUND
- 409 - IDEMPOTENCY_CONFLICT (same key, different payload) / INVALID_STATE conflicts
- 422 - INSUFFICIENT_FUNDS / business rule violations (optional; may use 409)
- 500 - INTERNAL_ERROR
