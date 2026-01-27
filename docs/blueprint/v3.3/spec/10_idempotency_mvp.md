# 10. IDEMPOTENCY (MVP)

10.1 Required
Idempotency-Key is REQUIRED for:
- POST /v1/transfers
- POST /v1/fx/execute
- POST /v1/spend/simulate
- All admin POST endpoints

10.2 Not applicable
- POST /v1/fx/quote is intentionally NOT idempotent: each call produces a fresh quote (new quote_id).
  - Retrying /v1/fx/quote should be done by re-calling it.
  - If the client needs replay semantics, it should persist quote_id client-side and re-use it via /v1/fx/execute.
  - If the product needs to audit “what quote was shown”, the system MUST persist FxQuote snapshots keyed by quote_id (created_at, expires_at, params, rate/fee snapshot). History retrieval should rely on quote_id (and admin search), not Idempotency-Key.

10.3 Conflict behavior
- Same key + same payload => replay the original response
- Same key + different payload => 409 IDEMPOTENCY_CONFLICT

10.4 Retention (TTL)
- Idempotency records retained for 48 hours.
- Records may be purged after TTL safely.

10.5 Storage model (normative)
Idempotency storage MUST be sufficient to:
- detect key reuse with a different payload (conflict),
- replay the original response body + status,
- support audit/debugging (who did what, when, via which endpoint),
- expire records deterministically after TTL.

Minimum fields:
- idempotency_key (TEXT, NOT NULL)
- actor_type (TEXT, NOT NULL)            # "admin" | "user" | "system"
- actor_id (TEXT, NULL)                 # phone_number (user) / admin_id (or null for system)
- endpoint (TEXT, NOT NULL)             # stable route id, e.g. "POST /v1/transfers"
- request_hash (TEXT, NOT NULL)         # hash over canonical request payload (+ critical headers if needed)
- status_code (INTEGER, NOT NULL)
- response_body_json (TEXT, NOT NULL)   # exact response (for replay), JSON string
- created_at (TIMESTAMP, NOT NULL)
- expires_at (TIMESTAMP, NOT NULL)      # created_at + 48h

Uniqueness:
- UNIQUE(actor_type, actor_id, endpoint, idempotency_key)
