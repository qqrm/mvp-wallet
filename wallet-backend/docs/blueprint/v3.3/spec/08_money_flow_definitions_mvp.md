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
QUOTE:
- read fx_rates for (from_currency/to_currency)
- compute:
  - amount_to_minor = floor(amount_from_minor * rate) with rounding rules
  - fee_minor (optional)
- store quote with rate + fee snapshot
- expires_at = now + QUOTE_TTL_SECONDS (recommended 60s)

EXECUTE:
- validate quote exists + not expired + not executed
- lock both accounts
- ensure from_account has enough available for amount_from + fee
- posting:
  - DEBIT  from_account_id: amount_from_minor
  - CREDIT to_account_id:   amount_to_minor
  - if fee_minor > 0:
      DEBIT  from_account_id: fee_minor
      CREDIT SYSTEM_FEE_ACCOUNT: fee_minor

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
