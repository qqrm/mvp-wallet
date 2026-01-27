# SOURCE_1__Blueprint_v3_1

## Page 22

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 22
- Amount input (from amount)
- PrimaryButton Get quote
States:
- Loading quote
- Quote returned
- Error (rate unavailable / validation)
Key actions:
- Get quote (POST /v1/fx/quote)
- Change currencies/amount
S11 FX quote review + execute
Purpose: Show rate, fee, expiry and allow execute; show posted result receipt.
Components (Uzum UI Kit patterns):
- Quote card (rate, fee, expires_at)
- PrimaryButton Execute
- Countdown/expiry hint
- Result receipt on success
States:
- Expired quote -> disable Execute
- Execution in progress
- Execution error -> show code/details
Key actions:
- Execute (POST /v1/fx/execute with Idempotency-Key)
- View receipt
- Start new quote
S6 Transaction list
