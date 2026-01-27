# 7. API CONTRACTS (MVP)

7.1 General
- OpenAPI is the authoritative source for exact request/response schemas (generated from code). This spec defines semantics and stable error codes.
- All money-moving POST endpoints require header: Idempotency-Key (EXCEPT `POST /v1/fx/quote`, which always returns a fresh quote).

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
- GET  /v1/accounts/{account_id}/transactions?before=<RFC3339 timestamp>&limit=<1..100>  (default limit=100; time-based paging into the past)
- GET  /v1/transactions/{tx_id}
- POST /v1/transfers
- POST /v1/fx/quote
- POST /v1/fx/execute
- POST /v1/spend/simulate

ADMIN:
- GET  /v1/admin/fx/fee
- PUT  /v1/admin/fx/fee
- POST /v1/admin/accounts (create)
- POST /v1/admin/accounts/{account_id}/close
- POST /v1/admin/fund
- POST /v1/admin/withdraw
- POST /v1/admin/reverse
- GET  /v1/admin/search

7.4 Error Codes (mandatory, stable)
Codes are part of the public contract. Frontend and tests must rely on codes, not message text.

Common:
- INVALID_ARGUMENT            (400) validation failed / malformed payload
- UNAUTHORIZED                (401) missing/invalid auth
- FORBIDDEN                   (403) authenticated but not allowed
- NOT_FOUND                   (404) resource not found (account/tx/quote)
- CONFLICT                    (409) state conflict (already closed/executed/etc.)
- INTERNAL                    (500) unexpected server error

Money / ledger:
- INSUFFICIENT_FUNDS          (422) not enough available balance
- ACCOUNT_CLOSED              (409) posting to CLOSED account
- ACCOUNT_BLOCKED             (403) user/account blocked
- TX_NOT_REVERSIBLE           (409) cannot reverse (already reversed / not posted / window exceeded)

Idempotency:
- IDEMPOTENCY_KEY_REQUIRED    (400) required header missing
- IDEMPOTENCY_CONFLICT        (409) same key used with different payload

Replay behavior:
- When a request is replayed (same key + same payload), server returns the original success response (same status + body).

FX:
- FX_RATE_UNAVAILABLE         (409) no rate configured for pair
- FX_QUOTE_EXPIRED            (409) quote expired
- FX_QUOTE_ALREADY_EXECUTED   (409) quote already executed
- FX_FEE_POLICY_INVALID       (400) fee_bps out of allowed range

Admin:
- ADMIN_TARGET_NOT_FOUND      (404) target user/account not found
- ADMIN_INVALID_AMOUNT        (400) amount not allowed

Notes:
- Use 422 for “business-rule validation” that depends on current state (insufficient funds, etc.).
- Include machine-readable fields in `error.details` where useful (e.g., required_min_balance, current_balance, quote_id).

7.5 Account details visibility (MVP)

GET /v1/accounts/{account_id}
- For USER:
  - account_id
  - currency
  - status
  - balances: ledger_balance, blocked_balance, available_balance (all in minor units)
  - updated_at (optional)
  - No owner identifiers (phone_number) are returned (user already knows themselves).
- For ADMIN:
  - all USER fields, plus:
  - owner_phone_number
  - created_at
  - any admin-only flags (if present in implementation)

7.6 Admin FX fee endpoints (MVP)

GET /v1/admin/fx/fee
- Returns current fee policy:
  - fee_bps (int)

PUT /v1/admin/fx/fee
- Sets current fee policy (validated range: 0..10_000 bps).
- Request body:
  - fee_bps (int)
- Response: the updated policy.
- Must be audited.