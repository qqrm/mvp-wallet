# SOURCE_1__Blueprint_v3_1

## Page 08

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 8
- Integrated now (MVP): Wallet ledger + FX engine; can be embedded into Uzum app via API gateway.
- Deferred: real Uzum Bank rails (external deposits/withdrawals), card issuing/acquiring, Uzum Market checkout and settlement.
- Data linking: user identity mapping to Uzum account ID; tenant_id reserved for future multi-tenant.
3.3 Domain model & invariants
- Entities: Tenant, User, Wallet, Account (per currency), JournalTx, JournalEntry (double-entry lines), FxQuote, IdempotencyRecord, AuditEvent.
- Invariant #1: JournalTx is immutable after POSTED. No delete/update of financial rows.
- Invariant #2: For each JournalTx and currency: sum(debits) == sum(credits) == total_amount.
- Invariant #3: Account available balance never goes negative unless overdraft flag enabled (out-of-scope).
- Invariant #4: All financial mutations happen inside a single DB transaction.
- Invariant #5: Every JournalTx must have correlation_id + request_id for traceability.
3.4 Balance model and transaction lifecycle
- Balance components: ledger_posted (immutable sum), blocked (0 in MVP), available = ledger_posted - blocked.
- Tx lifecycle: CREATED -> POSTED -> REVERSED (compensating reversal creates a new JournalTx referencing original).
- Account lifecycle: ACTIVE -> CLOSED. CLOSED forbids new debits/credits except admin correction/reversal.
- System accounts (MVP): SYSTEM_MINT, SYSTEM_SINK, SYSTEM_SPEND, SYSTEM_FX_POOL_[CCY], SYSTEM_FX_FEE_[CCY].
3.5 API specification & contracts
- OpenAPI 3.0 published for /v1 endpoints with schemas, examples, and deterministic error model.
- Stable versioning: /v1 path + semantic version in header (optional).
- All list endpoints use cursor pagination: ?cursor=&limit;= with deterministic ordering.
3.6 Idempotency and concurrency rules
- Idempotency: required on every POST that mutates money state (admin fund/withdraw/reverse, transfer, spend simulate, fx execute).
- If same Idempotency-Key is replayed with same request hash -> return stored response (200/201).
- If same Idempotency-Key is replayed with different payload -> 409 IDEMPOTENCY_CONFLICT.
- Concurrency: per account row lock (SELECT FOR UPDATE) OR SERIALIZABLE transaction when posting to the ledger.
3.7 Backend workflows & state machines
