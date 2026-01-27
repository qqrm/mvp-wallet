# 16. PRODUCT DECISION RECORD (PDR)

This file records decisions that affect implementation and future discussions.
The spec is the source of truth; PDR captures *why*.

PDR-001 (2026-01-27): Canonical success status is POSTED
- Decision: Use POSTED as the final success state. COMPLETED is not used in MVP.
- Rationale: Align terminology across ledger/receipts and avoid duplicate “success” states.

PDR-002 (2026-01-27): Add account details endpoint
- Decision: Add GET /v1/accounts/{account_id} for account details (in addition to GET /v1/accounts list).
- Rationale: Details view is needed; list endpoint can remain lightweight.

PDR-003 (2026-01-27): Path parameter naming
- Decision: Use {account_id} (not {id}) in API paths in the spec.
- Rationale: Improves clarity and consistency in OpenAPI/SDKs.

PDR-004 (2026-01-27): FX quote is non-idempotent
- Decision: POST /v1/fx/quote is NOT idempotent; each call produces a fresh quote_id.
- Rationale: Quotes must reflect current rates/fees; replay semantics belong to execute (by re-using quote_id).

PDR-005 (2026-01-27): Idempotency required for admin POST endpoints
- Decision: All admin POST endpoints require Idempotency-Key.
- Rationale: Prevent duplicate postings and accidental repeated admin operations.

PDR-006 (2026-01-27): Account create dedupe rule
- Decision: In MVP, (phone_number, currency) is unique; creating an account for an existing (phone_number, currency) returns the existing account.
- Rationale: Prevent duplicate accounts for the same user/currency while keeping retries safe.

PDR-007 (2026-01-27): Single-tenant MVP
- Decision: MVP is single-tenant; tenant_id/multi-tenant isolation is out of scope.
- Rationale: Avoid premature complexity; revisit when multi-tenant becomes a real requirement.

PDR-008 (2026-01-27): JWT provider out of scope (no Uzum SSO in MVP)
- Decision: JWT is required, but issuer/SSO integration is out of scope. Minimum claim is sub=phone_number (or user_id that equals phone_number in MVP).
- Rationale: Allows MVP to ship with a placeholder identity provider while keeping the contract explicit.

PDR-009 (2026-01-27): Transaction history default limit and time-based paging
- Decision: GET /v1/accounts/{account_id}/transactions returns the latest 100 transactions by default; older history is loaded using a time-based parameter (before=timestamp) with limit.
- Rationale: Simple client UX for “recent activity” while allowing incremental backfill without a server cursor in MVP.
