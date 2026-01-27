# SOURCE_1__Blueprint_v3_1

## Page 32

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 32
v3.1 Backend Definition of Done (DoD)
Release cannot ship until all items below are true.
- Endpoints implemented under /v1: admin accounts create/close, fund, withdraw, reverse, search; user transfers; spend simulate; fx quote/execute; history; tx
receipt.
- OpenAPI published and validated in CI; schema matches runtime behavior.
- Idempotency enforced on all money POST endpoints; 409 IDEMPOTENCY_CONFLICT on payload mismatch.
- Ledger invariants always pass (double-entry sum=0; no negative available).
- Integration tests: P2P success + insufficient funds; idempotency replay + conflict; FX quote/execute + expiry; spend simulate posts once; admin
fund/withdraw/reverse correctness.
- Observability: correlation_id everywhere; audit_event written for every money/admin action; dashboards for errors and posting latency.
- Persistence: PostgreSQL staging/prod; backups + restore drill executed; migrations gated in CI.
References (public sources)
| Ref | Source | URL |
| --- | --- | --- |
| R1 | Law No. ZRU-578 "On Payments and Payment Systems" (LexUZ) | <https://lex.uz/docs/4575788> |
| R2 | Law No. ZRU-547 "On Personal Data" (LexUZ) | <https://lex.uz/docs/4831939> |
| R3 | Law No. 660-II AML/CFT (LexUZ) | <https://lex.uz/en/docs/6808153> |
| R4 | Central Bank of Uzbekistan - Interbank payment system overview | <https://cbu.uz/en/payment-systems/interbank/> |
| R5 | Presidential Decree PD-381 (gov.uz news on payment org/operator changes) | <https://gov.uz/en/davaktiv/news/view/11556> |
| R6 | Presidential Resolution PP-167 on Critical Information Infrastructure cybersecurity (LexUZ) | <https://lex.uz/en/docs/7556383> |
| R7 | CBU payment systems department (official portal entry) | <https://cbu.uz/en/payments/payment-systems/> |
| R8 | Draft amendments discussion for Personal Data Law (UzDaily, 2026-01) | <https://www.uzdaily.uz/en/uzbekistan-considers-amendments-to-personal-data-law-to-promote-digital-pa> |
