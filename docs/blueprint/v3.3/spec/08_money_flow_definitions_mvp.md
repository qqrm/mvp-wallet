# 8. MONEY FLOW DEFINITIONS (MVP)

8.1 P2P Transfer posting
Inputs:
- from_account_id
- recipient (account_id OR user_id OR phone)
- amount_minor
- memo (optional)

Posting (single tx):
- DEBIT  from_account_id: amount_minor
- CREDIT to_account_id:   amount_minor
No fees in MVP for internal P2P (fee=0).

8.2 FX Convert (quote + execute)

System accounts (required for per-currency ledger invariants):
- SYSTEM_FX_POOL_{currency}    (platform liquidity/position per currency)
- SYSTEM_FX_FEE_{currency}     (fee revenue per currency)

QUOTE (not idempotent; always returns a fresh quote):
- read fx_rates for (from_currency, to_currency)
- read current fee policy (fee_bps, default 100 bps = 1%)
- compute `amount_to_minor` using integer fixed-point arithmetic:

  Definitions:
  - from_minor_units = minor_units(from_currency)
  - to_minor_units   = minor_units(to_currency)
  - rate = rate_scaled / rate_scale  (rate_scale = 1_000_000_000)

  Conversion (rounding in favor of platform):
  - numerator   = amount_from_minor * rate_scaled * 10^to_minor_units
  - denominator = rate_scale * 10^from_minor_units
  - amount_to_minor = floor(numerator / denominator)

  Implementation note:
  - Use at least 128-bit integer math for numerator/denominator to avoid overflow.

- fee (optional, MVP default enabled):
  - fee_minor = ceil(amount_from_minor * fee_bps / 10_000)

- store quote with full snapshot:
  - rate_scaled, rate_scale, fee_bps, fee_minor, amount_to_minor
  - expires_at = created_at + 5 minutes

EXECUTE (idempotent; uses quote_id):
- validate quote exists + not expired + not executed
- lock both user accounts involved (from_account_id, to_account_id)
- ensure from_account has enough AVAILABLE for (amount_from_minor + fee_minor)

Posting (single DB tx, append-only ledger):
Currency A (from_currency):
- DEBIT  user from_account_id:              amount_from_minor
- CREDIT SYSTEM_FX_POOL_{from_currency}:    amount_from_minor
- if fee_minor > 0:
    DEBIT  user from_account_id:            fee_minor
    CREDIT SYSTEM_FX_FEE_{from_currency}:   fee_minor

Currency B (to_currency):
- DEBIT  SYSTEM_FX_POOL_{to_currency}:      amount_to_minor
- CREDIT user to_account_id:                amount_to_minor

Notes:
- This structure preserves the invariant: for each (tx_id, currency), SUM(debits) == SUM(credits).
- Amount_to_minor is rounded down (platform-favoring, deterministic, testable).
- Any fractional remainder < 1 minor unit cannot be posted; it is economically retained in SYSTEM_FX_POOL_{to_currency}.

8.3 Admin fund (mint)
- CREDIT user account
- DEBIT  SYSTEM_MINT_ACCOUNT

8.4 Admin withdraw (burn)
- DEBIT  user account
- CREDIT SYSTEM_SINK_ACCOUNT

8.5 Reverse transaction
- Create a compensating transaction referencing original tx_id.
- Must not mutate original journal entries.
- Reversal MUST be idempotent per (original_tx_id).

8.6 Spend simulate (sandbox)
- DEBIT user account
- CREDIT SYSTEM_SPEND_ACCOUNT
- merchant_label required (string)
