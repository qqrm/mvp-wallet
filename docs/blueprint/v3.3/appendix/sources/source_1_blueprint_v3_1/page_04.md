# SOURCE_1__Blueprint_v3_1

## Page 04

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 4
CHANGELOG (v3 -> v3.1)
Only deltas required by the v3.1 upgrade prompt are listed below.
- Added Withdraw (MVP interpretation) as Admin Burn to SYSTEM_SINK (no external rails).
- Replaced account delete with close/suspend; hard delete forbidden.
- Added explicit Admin API endpoint list: create/close/fund/withdraw/reverse/search.
- Added explicit User Wallet API endpoint list: P2P transfers, balance, history, tx receipt.
- Added Mock Spend as first-class endpoint posting to SYSTEM_SPEND.
- Specified FX as 2-step flow: quote + execute, with expiry + rounding rules.
- Resolved DB mismatch: SQLite allowed only for dev/tests; PostgreSQL required for production + migration plan.
- Added compact Error Contract + mandatory error codes + HTTP mapping.
- Added v3.1 Backend Definition of Done with acceptance criteria and test coverage gates.
