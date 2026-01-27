# SOURCE_1__Blueprint_v3_1

## Page 21

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 21
- Insufficient funds pre-check (optional)
- Network error (snackbar)
Key actions:
- Continue to review
- Edit fields
- Cancel
S9 Transfer review + confirm
Purpose: Confirm the transfer before posting; show fees (0 in MVP) and final amount.
Components (Uzum UI Kit patterns):
- Summary card (from/to/amount)
- PrimaryButton Confirm
- SecondaryButton Edit
- Biometric/OTP gate (optional)
States:
- Posting in progress (spinner)
- Success -> show receipt
- Failure -> show error envelope code/message
Key actions:
- Confirm (calls POST /v1/transfers with Idempotency-Key)
- Open receipt
- Repeat transfer
S10 FX convert form (quote)
Purpose: Request an FX quote for conversion between two accounts.
Components (Uzum UI Kit patterns):
- From/To selectors (BottomSheet)
