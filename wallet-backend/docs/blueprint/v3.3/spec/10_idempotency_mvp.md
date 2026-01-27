# 10. IDEMPOTENCY (MVP)

10.1 Required
Idempotency-Key is REQUIRED for:
- POST /v1/transfers
- POST /v1/fx/quote
- POST /v1/fx/execute
- POST /v1/spend/simulate
- All admin POST endpoints

10.2 Conflict behavior
- Same key + same payload => replay the original response
- Same key + different payload => 409 IDEMPOTENCY_CONFLICT

10.3 Retention (TTL)
- Idempotency records retained for 48 hours.
- Records may be purged after TTL safely.
