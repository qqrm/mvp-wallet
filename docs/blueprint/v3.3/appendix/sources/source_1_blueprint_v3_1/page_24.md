# SOURCE_1__Blueprint_v3_1

## Page 24

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 24
- Error
Key actions:
- Share receipt
- Copy tx_id
- Report issue (deferred)
S12 Spend simulation (sandbox)
Purpose: Developer/support-only feature to simulate merchant spend in MVP.
Components (Uzum UI Kit patterns):
- Toggle 'Sandbox mode' in Settings
- Merchant label input
- Amount input
- PrimaryButton Simulate spend
States:
- Hidden when sandbox mode disabled
- Posting in progress
- Success shows receipt
Key actions:
- Simulate spend (POST /v1/spend/simulate with Idempotency-Key)
Critical flows (MVP)
- Onboarding/login -> Home -> balance refresh.
- Admin funds user -> user sees balance updated -> makes P2P transfer.
- FX quote -> review -> execute -> receipt shows rate + fee.
- Spend simulate -> history shows merchant_label and system spend account.
- Settings -> security options (biometric toggle, session logout).
