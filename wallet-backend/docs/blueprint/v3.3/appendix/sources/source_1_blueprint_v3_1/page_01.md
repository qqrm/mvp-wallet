# SOURCE_1__Blueprint_v3_1

## Page 01

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 1
Uzum Wallet MVP Blueprint (v3.1)
Production-grade, contract-first MVP plan (Uzbekistan-first) - monolith architecture
Version: 3.1 | Date: 2026-01-26 | Audience: CEO, Product, Engineering, Security, Compliance
- MVP boundaries: Wallet balances + double-entry ledger, internal P2P transfers, FX conversion (quote + execute), admin mint/burn/reverse, transaction history
+ receipts, sandbox spend simulation.
- Explicitly NOT in MVP: KYC, external rails (cards/top-ups/bank withdrawals), merchant checkout integration, chargebacks/disputes automation,
credit/insurance.
- Core guarantee: Immutable journal + atomic postings + strict idempotency on all financial endpoints.
- Persistence: PostgreSQL required for production; SQLite allowed only for dev/tests with migration plan.
