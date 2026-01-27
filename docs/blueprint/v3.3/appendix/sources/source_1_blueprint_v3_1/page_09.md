# SOURCE_1__Blueprint_v3_1

## Page 09

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 9
- P2P transfer: validate ACTIVE accounts + sufficient funds -> post JournalTx (debit sender, credit receiver).
- Admin fund (mint): post JournalTx (debit SYSTEM_MINT, credit user account).
- Withdraw (MVP interpretation): Admin Burn to SYSTEM_SINK (debit user account, credit SYSTEM_SINK).
- Reverse: create compensating JournalTx referencing original tx_id; never edit history.
- Spend simulate: post to SYSTEM_SPEND (debit user, credit SYSTEM_SPEND).
- FX quote: calculate rate + fee; persist FxQuote with expiry.
- FX execute: validate quote active + not expired; post JournalTx with multi-currency legs + fee legs.
3.8 Security architecture
- Authn: JWT/OAuth2 via Uzum identity; refresh tokens; device/session binding (minimal).
- Authz: roles: USER, ADMIN; admin endpoints require ADMIN; audit every admin action.
- Encryption: TLS in transit; at-rest encryption via Postgres disk/KMS; sensitive fields minimal in MVP.
- Rate limits: per-user and per-IP for transfer/fx/spend endpoints.
3.9 Uzbekistan compliance checklist -> engineering requirements
- Personal data processing must align to Uzbekistan Law on Personal Data (registration, protection controls).
- AML/CTF law exists even if KYC is deferred; keep auditability and transaction monitoring hooks.
- Payment services licensing scope depends on whether wallet is operated by a licensed bank/payment org; keep boundaries explicit.
3.10 Observability & audit logs
- AuditEvent append-only log for: admin fund/burn/reverse/close, P2P, FX execute, spend simulate.
- Every log entry includes actor_id, role, ip/device, correlation_id, affected_account_ids, tx_id, before/after balances.
- Metrics: posting latency, idempotency conflicts, insufficient funds rate, FX quote expiry rate, error codes.
3.11 Reliability patterns
- Atomic write transactions: journal + entries + balance cache update in one commit.
- Outbox table reserved for future async integrations (external rails).
- Backups + PITR, schema migrations, and restore drills as release gate.
3.12 Testing strategy
