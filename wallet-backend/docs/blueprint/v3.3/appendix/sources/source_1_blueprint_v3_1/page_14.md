# SOURCE_1__Blueprint_v3_1

## Page 14

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 14
Idempotency design
- Client sends header: Idempotency-Key (UUID recommended).
- Server stores: key, actor_id, endpoint, request_hash, status_code, response_body, created_at, ttl.
- Replay with same hash returns stored response with header Idempotency-Replayed: true.
- Replay with different hash returns 409 IDEMPOTENCY_CONFLICT with stored request hash metadata.
Example schemas (5 key endpoints)
POST /v1/transfers
Request:
```json
{
  "from_account_id": "acc_sender",
  "to_account_id": "acc_receiver",
  "amount": 150000,
  "currency": "UZS",
  "memo": "Dinner split"
}
```
Response (201):
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "balances": {
    "from": { "available": 850000, "blocked": 0, "ledger": 850000 },
    "to":   { "available": 1150000, "blocked": 0, "ledger": 1150000 }
  }
}
```
POST /v1/admin/fund
Request:
```json
{
  "account_id": "acc_user",
  "amount": 500000,
  "currency": "UZS",
  "reason": "Support top-up (admin mint)"
}
```
Response (201):
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
