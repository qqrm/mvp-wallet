# SOURCE_1__Blueprint_v3_1

## Page 06

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 6
25
Reliability
No isolation level decision.
Use SERIALIZABLE or SELECT ... FOR UPDATE on balance rows; document approach.
P0
26
Backend Logic
No spend/authorization model; unclear hold/capture.
For MVP: spend = direct posted debit; holds out-of-scope but reserved.
P2
27
UI/UX
No clear UX states (loading/empty/error).
Specify states for each screen; skeleton loaders; retry actions.
P1
28
Compliance
AML monitoring baseline not discussed.
Add velocity rules, auditability, and reporting readiness (post-MVP KYC).
P2
29
Roadmap
FX fee strategy not explicit.
Define fee/markup model with config + transparency on receipt.
P1
30
Testing
No explicit DoD acceptance criteria.
Add v3.1 Backend DoD: OpenAPI, tests, invariants, metrics, and audit logs.P0
