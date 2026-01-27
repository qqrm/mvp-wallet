# 9. RECEIPTS (MVP)

A receipt MUST exist for every posted transaction and MUST include:
- tx_id, tx_type, status
- timestamp(s)
- entries breakdown:
  - account label (user-facing)
  - direction (DEBIT/CREDIT)
  - amount + currency
- counterparty label (for transfers)
- memo or merchant_label (if provided)
- FX details (if applicable):
  - rate
  - fee
  - from_amount, to_amount
- correlation_id (optional in UI; always stored)

Receipt MUST NOT expose sensitive internal-only fields beyond tx_id.
(Showing last-4 of account_id is allowed if needed for support UX.)
