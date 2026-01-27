# 10. IDEMPOTENCY (MVP)

10.1 Required
Idempotency-Key is REQUIRED for:
- POST /v1/transfers
- POST /v1/fx/execute
- POST /v1/spend/simulate
- All admin POST endpoints (including POST /v1/admin/accounts)

Idempotency-Key is NOT used for:
- POST /v1/fx/quote (always returns a fresh quote; quote history is tracked via quote_id + stored snapshot)

10.2 Conflict behavior
- Same key + same payload => replay the original response
- Same key + different payload => 409 IDEMPOTENCY_CONFLICT

10.3 Retention (TTL)
- Idempotency records retained for 48 hours.
- Records may be purged after TTL safely.

10.4 Storage model (normative minimum)
The idempotency storage MUST allow safe replay and conflict detection.

Minimum fields:
- idempotency_key (string)
- endpoint (string; normalized route)
- actor_type (USER | ADMIN | SYSTEM)
- actor_id (string; in MVP for USER this is phone_number)
- request_hash (hash of canonical request payload)
- created_at (UTC)
- expires_at (created_at + 48h)
- response_status (int)
- response_body (bytes/json)
- tx_id (optional; if the request created a transaction)

Uniqueness constraint:
- UNIQUE(actor_type, actor_id, endpoint, idempotency_key)