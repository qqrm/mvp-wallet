# 1. PRODUCT PRINCIPLES (MVP)

P1. Ledger correctness is non-negotiable.
    - Double-entry postings only.
    - Immutable journal.
    - Corrections only via compensating transactions.

P2. Every money-moving request MUST be idempotent.
    - Idempotency-Key is mandatory on POST money endpoints.

P3. Every money/admin operation MUST produce audit events and a user-visible receipt.

P4. MVP is contract-first: OpenAPI is the source of truth for API contracts.
