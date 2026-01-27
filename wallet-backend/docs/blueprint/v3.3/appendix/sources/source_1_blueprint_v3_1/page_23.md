# SOURCE_1__Blueprint_v3_1

## Page 23

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 23
Purpose: Provide fast and reliable history view with filters.
Components (Uzum UI Kit patterns):
- List with infinite scroll (cursor pagination)
- Filters: type (P2P/FX/Admin/Spend), currency
- Search by tx_id (optional)
- Empty state illustration
States:
- Loading skeleton
- Empty
- Error + retry
- Pagination end
Key actions:
- Open receipt
- Apply filter
- Pull-to-refresh
S7 Transaction details / receipt
Purpose: Single source of truth for what happened (used for support).
Components (Uzum UI Kit patterns):
- Receipt header (status, tx_id)
- Entries breakdown (from/to/system accounts)
- FX details (rate, fee) when applicable
- Share button
States:
- Normal
- Not found
