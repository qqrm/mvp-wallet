# SOURCE_1__Blueprint_v3_1

## Page 15

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 15
  "balances": { "available": 1350000, "blocked": 0, "ledger": 1350000 }
}
POST /v1/admin/withdraw
Request:
```json
{
  "account_id": "acc_user",
  "amount": 200000,
  "currency": "UZS",
  "reason": "Admin burn to SYSTEM_SINK (MVP withdraw)"
}
```
Response (201):
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "balances": { "available": 1150000, "blocked": 0, "ledger": 1150000 }
}
```
POST /v1/fx/quote
Request:
```json
{
  "from_account_id": "acc_uzs",
  "to_account_id": "acc_usd",
  "amount_from": 1000000
}
```
Response (200):
```json
{
  "quote_id": "fxq_01H...",
  "from": { "currency": "UZS", "amount": 1000000 },
  "to":   { "currency": "USD", "amount": 82_00 },
  "rate": { "base": "UZS", "quote": "USD", "value": "0.00008200" },
  "fee":  { "currency": "UZS", "amount": 10000 },
  "expires_at": "2026-01-26T12:34:56Z"
}
```
POST /v1/fx/execute
Request:
```json
{
  "quote_id": "fxq_01H..."
}
```
Response (201):
