# 4. DOMAIN MODEL (MVP)

4.1 User
Fields (minimum):
- user_id (stable internal identifier)
- phone_number (unique)
- status: ACTIVE | BLOCKED
- created_at

4.2 Account
Each user has 1 account per currency in MVP.
Fields:
- account_id
- user_id (owner)
- currency (ISO code) — MVP: UZS, USD
- status: ACTIVE | CLOSED
- created_at

Rules:
- CLOSED account cannot be used for new postings.
- No deletes. Ever.

4.3 Ledger JournalTx (transaction header)
Fields:
- tx_id (UUID or ULID)
- tx_type: TRANSFER | FX_CONVERT | ADMIN_FUND | ADMIN_WITHDRAW | ADMIN_REVERSE | SPEND_SIMULATE
- status: CREATED | POSTED | REVERSED | FAILED (FAILED may exist for request logging; no partial journal entries)
- created_at, posted_at
- actor_type: USER | ADMIN | SYSTEM
- actor_id
- correlation_id
- reference fields (external_reference optional, memo optional)

4.4 Ledger JournalEntry (transaction lines)
Fields:
- entry_id
- tx_id
- account_id
- currency
- amount_minor (int, currency minor units)
- direction: DEBIT | CREDIT
- created_at

Invariant:
- For each tx_id and currency, SUM(debit) == SUM(credit).

4.5 Balances
MVP uses AVAILABLE balance only (blocked=0).
Define:
- ledger_balance = sum(credits - debits) posted to account
- blocked_balance = 0 (MVP)
- available_balance = ledger_balance - blocked_balance

4.6 FX Quote
Fields:
- quote_id
- from_account_id (currency A)
- to_account_id (currency B)
- amount_from_minor
- rate (decimal)
- fee_minor (optional)
- expires_at
- status: CREATED | EXECUTED | EXPIRED | CANCELED
- rate_snapshot (the actual rate applied for this quote)
Rules:
- A quote is immutable once created.
- Execute only before expires_at.

4.7 FX Rates (MVP rule)
FX rates are CONFIG-DRIVEN IN MVP via database table `fx_rates`.
No external rate feed dependency required for MVP.

Minimum fx_rates schema:
- pair (e.g., UZS/USD)
- rate
- markup_basis_points (optional)
- updated_at
