# SOURCE_1__Blueprint_v3_1

## Page 16

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 16
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "fx": {
    "from_debit": { "currency": "UZS", "amount": 1000000 },
    "to_credit":  { "currency": "USD", "amount": 82_00 },
    "fee":        { "currency": "UZS", "amount": 10000 }
  }
}
```
LOCK AS-IS vs MIGRATE
Recommendation: MIGRATE (minimal). If existing prototype endpoints differ, keep the backend logic but migrate contracts to the v3.1 set with stable resource
naming and idempotency enforcement. Migration should be shallow: add new endpoints while keeping old ones behind /v0 for a short compatibility window (1-2
weeks), then remove.
