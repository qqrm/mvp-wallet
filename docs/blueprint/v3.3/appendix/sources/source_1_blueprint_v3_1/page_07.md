# SOURCE_1__Blueprint_v3_1

## Page 07

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 7
(3) v3.1 SPEC BLUEPRINT (TABLE OF CONTENTS + CONTENT)
This blueprint defines what must exist in the v3.1 implementation spec. It is intentionally concise but implementation-ready.
Table of contents
- 3.1 Product scope & personas
- 3.2 Ecosystem aggregation model (Uzum Bank/Market integration cut line)
- 3.3 Domain model & invariants
- 3.4 Balance model and transaction lifecycle
- 3.5 API specification & contracts (OpenAPI)
- 3.6 Idempotency and concurrency rules
- 3.7 Backend workflows & state machines (P2P, Admin ops, FX, Spend simulation)
- 3.8 Security architecture
- 3.9 Uzbekistan compliance checklist -> engineering requirements
- 3.10 Observability & audit logs
- 3.11 Reliability patterns
- 3.12 Testing strategy
- 3.13 MVP backlog and delivery plan
- 3.14 Post-MVP roadmap
- 3.15 Open questions / decisions pending
3.1 Product scope & personas
- End-user persona: Uzum ecosystem user who needs a multi-currency balance, internal transfers, and transparent FX conversion with receipts.
- Admin persona: internal operations staff (support/finance) who can create/close accounts, fund (mint), withdraw (burn), and reverse postings safely.
- Merchant persona: deferred. MVP includes only sandbox spend simulation (no real merchant rails).
- Core MVP value: Correct balances + fast internal transfer + predictable FX conversion.
3.2 Ecosystem aggregation model
