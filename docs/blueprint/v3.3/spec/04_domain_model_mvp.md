# 4. DOMAIN MODEL (MVP)

4.0 Money representation (MVP)
- All money amounts are stored and processed as integers in minor units (no floating point anywhere).
- Each currency has `minor_units` (number of decimal digits).
  - USD: minor_units=2 (cents)
  - UZS: minor_units=0 (MVP default; configurable if you decide to support tiyin)
- Any conversion, fee, or percentage calculation MUST be done with integer arithmetic + explicit rounding rules (see §8.2).



4.1 User
Fields (minimum):
- phone_number (unique, primary user identifier in MVP; also used as `sub` in JWT)
- email (optional, MVP)
- full_name (optional, MVP)
- status: ACTIVE | BLOCKED
- created_at

Notes:
- `user_id` may exist internally, but in MVP it MUST be equivalent to `phone_number`.

4.2 Account
Each user has 1 account per currency in MVP.

Fields:
- account_id
- phone_number (owner)
- currency (ISO code) — MVP: UZS, USD
- status: ACTIVE | CLOSED
- created_at

Rules:
- Uniqueness: (phone_number, currency) MUST be unique.
- Create account is deduplicated by (phone_number, currency): repeated create MUST return the existing account.
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
- rate_scaled (int) — fixed-point FX rate (see below)
- rate_scale (int) — constant scale (MVP: 1_000_000_000)
- fee_minor (optional)
- created_at
- expires_at
- status: CREATED | EXECUTED | EXPIRED | CANCELED
- rate_snapshot (rate_scaled + rate_scale + fee settings snapshot)

FX rate representation (MVP best-practice)
- We store FX rate as fixed-point: `rate = rate_scaled / rate_scale`
- Units: (to_major / from_major)
- Conversion uses currency minor units to avoid implicit decimals.

Rules:
- A quote is immutable once created.
- Quote TTL: expires_at MUST be created_at + 5 minutes (MVP default; configurable).
- Execute only before expires_at.
- Quotes are stored as a snapshot for audit/receipt purposes (what was shown to the user).

4.7 FX Rates (MVP rule)
FX rates are CONFIG-DRIVEN IN MVP via database table `fx_rates`.
No external rate feed dependency required for MVP.

Minimum fx_rates schema:
- from_currency (e.g., UZS)
- to_currency   (e.g., USD)
- rate_scaled   (int)  -- fixed-point rate (to_major/from_major)
- rate_scale    (int)  -- constant (MVP: 1_000_000_000)
- markup_basis_points (optional; future use)
- updated_at



4.8 FX Fee Policy (MVP)
- FX fee is configurable by admin (default: 1% = 100 bps).
- Fee is charged in the FROM currency (currency A) as an extra debit.
- Fee is expressed as `fee_bps` (basis points, 1 bp = 0.01%).
- Fee rounding rule (platform-favoring, deterministic): fee_minor = ceil(amount_from_minor * fee_bps / 10_000).
- Collected fee is credited to a system fee account for that currency (see §8.2).

Minimum storage (MVP):
- Table `fx_fee_policy` (single-row or versioned):
  - fee_bps (int, default 100)
  - updated_at
  - updated_by (admin actor_id)
- Updates MUST be audited (see audit_log epic).

