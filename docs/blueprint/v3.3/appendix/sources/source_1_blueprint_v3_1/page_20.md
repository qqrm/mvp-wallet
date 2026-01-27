# SOURCE_1__Blueprint_v3_1

## Page 20

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 20
Components (Uzum UI Kit patterns):
- Balance cards per currency (Card + Typography)
- Primary CTA buttons: Transfer, Convert (FX), History
- Inline status banner (for outages/maintenance)
- Pull-to-refresh and skeleton loader
States:
- Loading skeleton
- Normal state
- Error state (retry)
- Empty state (no accounts yet)
Key actions:
- Open account details
- Start transfer
- Start FX quote
- Open transaction history
S8 Transfer form (P2P)
Purpose: Create an internal transfer between accounts.
Components (Uzum UI Kit patterns):
- From account selector (BottomSheet)
- To recipient selector (contact/ID) - MVP can be account_id input
- Amount input with currency display
- Memo input (optional)
- PrimaryButton Continue
States:
- Validation errors (inline)
