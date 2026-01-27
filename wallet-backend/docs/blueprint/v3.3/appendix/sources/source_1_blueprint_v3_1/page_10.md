# SOURCE_1__Blueprint_v3_1

## Page 10

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 10
- E2E tests for critical journeys (P2P, admin fund/burn/reverse, spend simulate, FX quote/execute).
- Contract tests validating OpenAPI schemas and error codes.
- Property-based tests for ledger invariants and idempotency behavior.
3.13 MVP backlog and delivery plan
- Week 1: lock OpenAPI + Postgres schema + ledger posting engine + idempotency middleware.
- Week 2: Admin APIs + user P2P + transaction history endpoints + receipt model.
- Week 3: FX quote/execute + spend simulate + observability + audit log.
- Week 4: mobile UX wiring + E2E suite + load tests + hardening + release readiness.
3.14 Post-MVP roadmap
- Short list: merchant checkout rails, holds/capture, KYC, external topups/withdrawals, limits, disputes.
- Long list: shared/family wallets, multi-tenant, lending/BNPL, insurance, international expansion, advanced fraud scoring.
3.15 Open questions / decisions pending
See Section (9) for the decision list required from CEO/business/legal.
