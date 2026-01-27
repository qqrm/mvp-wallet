# 16. PDR / Decisions (MVP)

This file records decisions made during spec normalization to keep the spec unambiguous.

## Decided

1) Source of truth
- The Markdown spec in this repo is the only source of truth.
- External PDFs are reference-only for cross-checking.

2) User identity (MVP)
- Primary user identifier in MVP is phone_number (string).
- External SSO/IdP integration is out of scope.

3) Account uniqueness and create dedupe (MVP)
- Each user has 1 account per currency.
- Uniqueness key: (phone_number, currency).
- Admin/user create of an account MUST be deduplicated: repeated create returns the existing account.

4) Transaction status terminology
- Successful final state is POSTED.
- COMPLETED is not used.

5) Transaction history paging
- Default `limit=100`.
- Paging is time-based into the past via `before=<timestamp>` (no cursor in MVP).

6) FX quote vs execute
- `POST /v1/fx/quote` returns a firm quote snapshot with TTL=5 minutes (expires_at).
- Quote requests are NOT idempotent: each call returns a fresh quote.
- Execution uses `POST /v1/fx/execute` referencing `quote_id` (idempotency applies to execute).

7) Idempotency requirements
- Idempotency-Key is REQUIRED for all admin POST endpoints.
- Idempotency-Key is REQUIRED for money-moving POST endpoints, excluding `POST /v1/fx/quote`.

8) Tenant model
- MVP is single-tenant; multi-tenant is out of scope.

9) FX rounding and precision (MVP)
- No floating point. FX rates stored as fixed-point: rate_scaled / rate_scale (rate_scale=1e9).
- amount_to_minor is computed with floor() (rounding in favor of platform; deterministic).

10) FX fee policy (MVP)
- Fee is configurable via admin (fee_bps, default 100 bps = 1%).
- fee_minor is computed with ceil() (platform-favoring; deterministic).

11) Stable error codes
- error.code is part of the public API contract and must be stable.

12) Account details visibility
- /v1/accounts/{account_id} returns different field sets for USER vs ADMIN (see API contracts).

## Open questions (not decided)

- (none)
