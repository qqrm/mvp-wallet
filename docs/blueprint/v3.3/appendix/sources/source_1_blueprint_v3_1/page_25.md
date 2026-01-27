# SOURCE_1__Blueprint_v3_1

## Page 25

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 25
Modern wallet UX principles (Revolut-like)
- Always show available balance and keep it consistent with receipts.
- Every write action has a review step and produces a receipt with a stable tx_id.
- Errors are actionable: show user-friendly text + hidden technical error.code for support.
- Fast history: stable ordering, cursor pagination, cached last page for offline view (optional).
- Transparency on FX: rate, fee, expiry, and final credited amount are visible before execution.
