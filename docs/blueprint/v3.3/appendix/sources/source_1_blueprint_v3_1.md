# SOURCE 1 — Blueprint v3.1

<a id="p001"></a>
- MVP boundaries: Wallet balances + double-entry ledger, internal P2P transfers, FX conversion (quote + execute), admin mint/burn/reverse, transaction history receipts, sandbox spend simulation.
- Explicitly NOT in MVP: KYC, external rails (cards/top-ups/bank withdrawals), merchant checkout integration, chargebacks/disputes automation, credit/insurance.
- Core guarantee: Immutable journal + atomic postings + strict idempotency on all financial endpoints.
- Persistence: PostgreSQL required for production; SQLite allowed only for dev/tests with migration plan.
---

<a id="p002"></a>
(1) EXECUTIVE SUMMARY PDF v2 is a solid conceptual overview for a multi-currency wallet ledger (double-entry, holds/capture ideas), but it is not implementation-complete for fintech. The largest missing pieces were locked API contracts, a strict admin safety model, and enforcement of invariants at the DB + transaction level. v3.1 converts the concept into a contract-first MVP blueprint aligned to the current backend direction: admin helpers (create/close/fund/withdraw/reverse), P2P transfers, sandbox spend, and FX quote/execute. KYC is explicitly deferred. Biggest risks for MVP
- Ledger correctness & concurrency: without enforced constraints + atomic postings, balances will diverge and cannot be repaired safely.
- Missing idempotency enforcement on all write endpoints leads to duplicate credits/debits under retries (irreversible support debt).
- Admin operations without role boundaries and compensating corrections create insider-risk and audit gaps.
- FX without defined rounding + quote expiry semantics will leak money and break reconciliation.
- SQLite in production for financial ledger is operationally fragile; PostgreSQL migration is mandatory before launch.
Top 10 improvements that move the needle
- Lock OpenAPI contracts + stable error envelope + idempotency semantics for every financial POST endpoint.
- Adopt PostgreSQL schema with DB-enforced invariants (unique keys, foreign keys, check constraints) and transaction isolation rules.
- Implement immutable journal + double-entry postings; forbid destructive deletes for any financial objects.
- Add Admin API with explicit roles and compensating reversal endpoint (no edits to historical ledger rows).
- Introduce state machines (type-state pattern) for Tx lifecycle and FX quote/execute lifecycle.
- Build automated E2E suite for critical journeys + invariant property tests for ledger correctness.
- Add audit log (append-only) for all admin and financial actions with correlation IDs.
- Add minimal fraud/abuse controls: rate limits, device/session controls, admin 4-eyes approval hook (optional).
- Define balance model precisely (available/blocked/ledger) and rounding policy per currency.
- Define a production-ready ops baseline: backups, migrations, alerting, and incident playbook.
Recommended MVP scope boundary IN MVP (ship in ~1 month) OUT OF MVP (explicitly deferred)
---

<a id="p003"></a>
- Ledger + balances
- P2P transfers
- FX quote+execute
- Admin fund/withdraw/reverse
- Sandbox spend simulation
- Transaction history + receipt
- Basic auth/session
- Minimal admin UI
- Observability + audit logs
- KYC/identity verification
- External top-ups/withdrawals/card rails
- Merchant checkout integration
- Disputes/chargebacks automation
- Shared/family wallets
- Lending/insurance
- Advanced fraud scoring
---

<a id="p004"></a>
CHANGELOG (v3 -> v3.1) Only deltas required by the v3.1 upgrade prompt are listed below.
- Added Withdraw (MVP interpretation) as Admin Burn to SYSTEM_SINK (no external rails).
- Replaced account delete with close/suspend; hard delete forbidden.
- Added explicit Admin API endpoint list: create/close/fund/withdraw/reverse/search.
- Added explicit User Wallet API endpoint list: P2P transfers, balance, history, tx receipt.
- Added Mock Spend as first-class endpoint posting to SYSTEM_SPEND.
- Specified FX as 2-step flow: quote + execute, with expiry + rounding rules.
- Resolved DB mismatch: SQLite allowed only for dev/tests; PostgreSQL required for production + migration plan.
- Added compact Error Contract + mandatory error codes + HTTP mapping.
- Added v3.1 Backend Definition of Done with acceptance criteria and test coverage gates.
---

<a id="p005"></a>
(2) PDF v2 AUDIT: GAPS & FIXES (STRICT) Below are high-impact gaps observed in PDF v2 and how v3.1 fixes or requires fixes. Format: Category - Problem - Why it matters - Concrete fix - Priority.

## PDF v2 audit gaps and fixes (structured)

This section is a readable conversion of the original gap table. It preserves the content while improving navigation.

### Gap 1 — API/Contracts (P0)
- Problem: No locked OpenAPI contracts; endpoint semantics implied only.
- Concrete fix: Publish OpenAPI for all /v1 endpoints with schemas, examples, and error model.

### Gap 2 — Domain Model (P0)
- Problem: Ledger is described conceptually but invariants are not enforceable.
- Concrete fix: Define double-entry tables + check constraints (sum debits == sum credits per journal).

### Gap 3 — Reliability/Testing (P0)
- Problem: Idempotency is mentioned but not mandatory for all financial writes.
- Concrete fix: Require Idempotency-Key for all POST financial endpoints; enforce conflict behavior.

### Gap 4 — Security (P0)
- Problem: No admin safety boundary (who can mint/burn/reverse).
- Concrete fix: Role-based authz + admin audit log; optional 4-eyes approval flow.

### Gap 5 — Data (P0)
- Problem: No immutable journal rule; potential updates/deletes.
- Concrete fix: Append-only journal; reversals only via compensating postings.

### Gap 6 — Backend Logic (P0)
- Problem: No explicit transaction lifecycle states.
- Concrete fix: Define Tx state machine: CREATED -> POSTED -> REVERSED; apply type-state.

### Gap 7 — Backend Logic (P0)
- Problem: FX is not contractually specified (rates, rounding, expiry).
- Concrete fix: Implement quote->execute with expiry, rounding policy, and fee calculation.P0 8 Product Withdraw/topup scope is unclear (external rails vs internal admin ops). Define admin fund/burn as internal ledger operations; external rails out of MVP.

### Gap 9 — Data (P0)
- Problem: Balance model (available vs blocked) unclear for MVP.
- Concrete fix: Define available as ledger_posted - blocked; blocked=0 in MVP unless holds feature enabled.

### Gap 10 — NFR/Operations (P1)
- Problem: DB choice not aligned with fintech requirements (SQLite risk).
- Concrete fix: Use PostgreSQL in production; SQLite only for dev/tests; add migration plan.P0 11 Observability No correlation IDs across requests and ledger journal. Require request_id/correlation_id; store in journal + audit logs; propagate in tracing.

### Gap 12 — Compliance (P1)
- Problem: Uzbekistan data localization / personal data obligations not translated to controls.
- Concrete fix: Add data register, retention rules, access logging, and locality assumptions.P1 13 UI/UX Wallet history UX not specified (filters, receipts). Define transaction list filters, receipt layout, and error states.

### Gap 14 — Backend Logic (P0)
- Problem: No admin correction strategy.
- Concrete fix: Add /admin/reverse that posts compensating journal entries referencing original tx.

### Gap 15 — API/Contracts (P0)
- Problem: No stable error envelope; ad-hoc error strings.
- Concrete fix: Define {error:{code,message,details}}; list mandatory codes + HTTP mapping.

### Gap 16 — Reliability/Testing (P1)
- Problem: No property-based invariant tests.
- Concrete fix: Add proptest/quickcheck invariants: no money created, double-entry sum zero, idempotency replay.

### Gap 17 — Roadmap (P1)
- Problem: Shared/family wallets not framed as future capability.
- Concrete fix: Reserve tenant/wallet group abstraction in schema; keep MVP single-owner.P2 18 Security No rate limits / abuse controls for transfer APIs. Implement per-user and per-IP throttles; add velocity limits.

### Gap 19 — Reliability (P2)
- Problem: No job/outbox pattern for async reconciliation.
- Concrete fix: Add outbox table for future integrations; not required to run in MVP.

### Gap 20 — Data (P1)
- Problem: No deterministic ordering/pagination in history endpoints.
- Concrete fix: Cursor-based pagination by (created_at, tx_id).

### Gap 21 — Product (P0)
- Problem: Merchant flow undefined.
- Concrete fix: Explicitly defer merchant integration; keep sandbox spend simulation only. P1 22 API/Contracts Account lifecycle rules missing (close/suspend semantics). Define closed account behavior and allow only admin corrections.

### Gap 23 — Security (P1)
- Problem: No secrets management baseline.
- Concrete fix: Store secrets in vault/KMS; rotate keys; use env var only in dev.

### Gap 24 — NFR/Operations (P0)
- Problem: No backup/restore plan.
- Concrete fix: Daily backups + PITR; restore drills; migration gating in CI.
---

<a id="p006"></a>
25 Reliability No isolation level decision. Use SERIALIZABLE or SELECT ... FOR UPDATE on balance rows; document approach. P0 26 Backend Logic No spend/authorization model; unclear hold/capture. For MVP: spend = direct posted debit; holds out-of-scope but reserved. P2 27 UI/UX No clear UX states (loading/empty/error). Specify states for each screen; skeleton loaders; retry actions. P1 28 Compliance AML monitoring baseline not discussed. Add velocity rules, auditability, and reporting readiness (post-MVP KYC). P2 29 Roadmap FX fee strategy not explicit. Define fee/markup model with config + transparency on receipt. P1 30 Testing No explicit DoD acceptance criteria. Add v3.1 Backend DoD: OpenAPI, tests, invariants, metrics, and audit logs.P0
---

<a id="p007"></a>
(3) v3.1 SPEC BLUEPRINT (TABLE OF CONTENTS + CONTENT) This blueprint defines what must exist in the v3.1 implementation spec. It is intentionally concise but implementation-ready. Table of contents
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
- 3.15 Open questions / decisions pending 3.1 Product scope & personas
- End-user persona: Uzum ecosystem user who needs a multi-currency balance, internal transfers, and transparent FX conversion with receipts.
- Admin persona: internal operations staff (support/finance) who can create/close accounts, fund (mint), withdraw (burn), and reverse postings safely.
- Merchant persona: deferred. MVP includes only sandbox spend simulation (no real merchant rails).
- Core MVP value: Correct balances + fast internal transfer + predictable FX conversion.
3.2 Ecosystem aggregation model
---

<a id="p008"></a>
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
---

<a id="p009"></a>
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
---

<a id="p010"></a>
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
3.15 Open questions / decisions pending See Section (9) for the decision list required from CEO/business/legal.
---

<a id="p011"></a>
3.16 Persistence Strategy (MVP) MVP allows SQLite for local development and automated tests, but production must run on PostgreSQL. This is non-negotiable for operational reliability (locking, backups, PITR, migrations).
- Dev/Test: SQLite (fast iteration, in-memory tests).
- Staging/Prod: PostgreSQL 15+ (required).
- Migration plan: use SQL migrations (e.g., sqlx/sea-orm/diesel migrations) and run a one-time data export/import tool before launch.
- Release gate: no production launch until Postgres E2E suite is green.
3.17 Database schema (MVP - ledger core)
| Table | Purpose | Key fields (minimal) |
| --- | --- | --- |
| tenants | Future multi-tenant boundary | tenant_id, name, status |
| users | Maps Uzum user identity | user_id, tenant_id, phone/email, status |
| accounts | Per-currency ledger account | account_id, user_id, currency, status, created_at |
| journal_tx | Immutable journal header | tx_id, type, status, correlation_id, created_at, reversed_tx_id? |
| journal_entry | Double-entry lines | entry_id, tx_id, account_id, currency, amount, direction, meta_json |
| balance_cache | Fast balances (derived) | account_id, currency, ledger, blocked, available, updated_at |
| idempotency | Idempotency storage | key, actor_id, endpoint, request_hash, response_json, status_code, created_at |
| fx_quote | Quote lifecycle | quote_id, from_account_id, to_account_id, amount_from, rate, fee, expires_at, status |
| audit_event | Append-only audit | event_id, actor_id, role, action, target_ids, tx_id?, created_at, ip/device |
---

<a id="p012"></a>
(4) API CONTRACT RECOMMENDATION (CONTRACT-FIRST) API principles
- Versioning: /v1 path; breaking changes require /v2.
- Auth: Authorization: Bearer <JWT>; ADMIN role enforced via claims.
- Idempotency: Idempotency-Key required on money-mutating POST endpoints.
- Pagination: cursor-based; order by (created_at DESC, tx_id DESC).
- Error envelope: stable JSON error format with machine-readable codes.
Admin API (MVP required)
| Method | Endpoint | Purpose | Idempotent |
| --- | --- | --- | --- |
| POST | /v1/admin/accounts | Create account for user/currency | Yes (recommended) |
| POST | /v1/admin/accounts/{id}/close | Close/suspend account (no hard delete) | Yes |
| POST | /v1/admin/fund | Mint funds to user account via SYSTEM_MINT | Yes (required) |
| POST | /v1/admin/withdraw | Burn funds to SYSTEM_SINK (MVP withdraw) | Yes (required) |
| POST | /v1/admin/reverse | Compensating reversal of existing tx | Yes (required) |
| GET | /v1/admin/search | Lookup accounts/tx by filters | N/A |
User Wallet API
| Method | Endpoint | Purpose | Idempotent |
| --- | --- | --- | --- |
| GET | /v1/profile | User profile + default wallet/account ids | N/A |
| GET | /v1/accounts/{id} | Account details (status, currency) | N/A |
| GET | /v1/accounts/{id}/balance | Balance breakdown: available/blocked/ledger | N/A |
| GET | /v1/accounts/{id}/transactions?cursor=&limit= | Transaction history | N/A |
| GET | /v1/transactions/{tx_id} | Transaction receipt/details | N/A |
| POST | /v1/transfers | P2P transfer between accounts | Yes (required) |
| POST | /v1/spend/simulate | Sandbox spend: transfer to SYSTEM_SPEND | Yes (required) |
| POST | /v1/fx/quote | Create FX quote (2-step) | Yes (recommended) |
---

<a id="p013"></a>
POST /v1/fx/execute Execute FX quote (posts ledger) Yes (required) Error model and codes Error envelope (all errors):
```json
{
  "error": {
    "code": "INSUFFICIENT_FUNDS",
    "message": "Not enough available balance",
    "details": { "account_id": "acc_...", "available": 120000, "required": 200000 }
  }
}
```
Mandatory error codes (MVP):
- INSUFFICIENT_FUNDS
- INVALID_STATE
- ACCOUNT_CLOSED
- IDEMPOTENCY_REPLAY
- IDEMPOTENCY_CONFLICT
- VALIDATION_ERROR
- NOT_FOUND
- UNAUTHORIZED / FORBIDDEN HTTP mapping:
- 400 - VALIDATION_ERROR
- 401 - UNAUTHORIZED
- 403 - FORBIDDEN
- 404 - NOT_FOUND
- 409 - IDEMPOTENCY_CONFLICT (same key, different payload) / INVALID_STATE conflicts
- 422 - INSUFFICIENT_FUNDS / business rule violations (optional; may use 409)
- 500 - INTERNAL_ERROR
---

<a id="p014"></a>
Idempotency design
- Client sends header: Idempotency-Key (UUID recommended).
- Server stores: key, actor_id, endpoint, request_hash, status_code, response_body, created_at, ttl.
- Replay with same hash returns stored response with header Idempotency-Replayed: true.
- Replay with different hash returns 409 IDEMPOTENCY_CONFLICT with stored request hash metadata.
Example schemas (5 key endpoints) POST /v1/transfers Request:
```json
{
  "from_account_id": "acc_sender",
  "to_account_id": "acc_receiver",
  "amount": 150000,
  "currency": "UZS",
  "memo": "Dinner split"
}
```
Response (201):
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "balances": {
    "from": { "available": 850000, "blocked": 0, "ledger": 850000 },
    "to":   { "available": 1150000, "blocked": 0, "ledger": 1150000 }
  }
}
```
POST /v1/admin/fund Request:
```json
{
  "account_id": "acc_user",
  "amount": 500000,
  "currency": "UZS",
  "reason": "Support top-up (admin mint)"
}
```
Response (201): { "tx_id": "tx_01H...", "status": "POSTED",
---

<a id="p015"></a>
 "balances": { "available": 1350000, "blocked": 0, "ledger": 1350000 } } POST /v1/admin/withdraw Request:
```json
{
  "account_id": "acc_user",
  "amount": 200000,
  "currency": "UZS",
  "reason": "Admin burn to SYSTEM_SINK (MVP withdraw)"
}
```
Response (201):
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "balances": { "available": 1150000, "blocked": 0, "ledger": 1150000 }
}
```
POST /v1/fx/quote Request:
```json
{
  "from_account_id": "acc_uzs",
  "to_account_id": "acc_usd",
  "amount_from": 1000000
}
```
Response (200):
```json
{
  "quote_id": "fxq_01H...",
  "from": { "currency": "UZS", "amount": 1000000 },
  "to":   { "currency": "USD", "amount": 82_00 },
  "rate": { "base": "UZS", "quote": "USD", "value": "0.00008200" },
  "fee":  { "currency": "UZS", "amount": 10000 },
  "expires_at": "2026-01-26T12:34:56Z"
}
```
POST /v1/fx/execute Request:
```json
{
  "quote_id": "fxq_01H..."
}
```
Response (201):
---

<a id="p016"></a>
```json
{
  "tx_id": "tx_01H...",
  "status": "POSTED",
  "fx": {
    "from_debit": { "currency": "UZS", "amount": 1000000 },
    "to_credit":  { "currency": "USD", "amount": 82_00 },
    "fee":        { "currency": "UZS", "amount": 10000 }
  }
}
```
LOCK AS-IS vs MIGRATE Recommendation: MIGRATE (minimal). If existing prototype endpoints differ, keep the backend logic but migrate contracts to the v3.1 set with stable resource naming and idempotency enforcement. Migration should be shallow: add new endpoints while keeping old ones behind /v0 for a short compatibility window (1-2 weeks), then remove.
---

<a id="p017"></a>
(5) RELIABILITY & TEST PLAN (ENGINEERING TECHNIQUES) E2E tests (critical journeys)
- e2e_login_and_get_wallet_overview: login -> fetch /profile -> get balance.
- e2e_admin_create_account_then_fund: create account -> fund -> balance increased.
- e2e_p2p_transfer_success: sender funded -> transfer -> sender decreases, receiver increases; tx receipt correct.
- e2e_p2p_insufficient_funds: transfer fails with INSUFFICIENT_FUNDS and no ledger mutation.
- e2e_spend_simulate_posts_once: simulate spend -> exactly one tx posted; balance decreases once.
- e2e_fx_quote_execute_success: quote -> execute -> multi-currency balances updated and fee posted.
- e2e_fx_execute_expired_quote: quote expires -> execute fails with INVALID_STATE.
- e2e_admin_withdraw_burn_to_sink: withdraw -> SYSTEM_SINK credited, user debited.
- e2e_admin_reverse_compensates: post tx -> reverse -> net effect zero; original remains immutable.
- e2e_cursor_pagination_history: create N tx -> paginate -> no gaps/duplicates, stable order.
Contract tests
- openapi_schema_validation_all_endpoints: responses validate against OpenAPI for success + error cases.
- consumer_error_code_stability: each failure mode returns correct error.code and HTTP status.
- pagination_contract: cursor tokens are opaque strings; limit respected; deterministic order.
Type-state pattern (where to apply)
- TxState: Created -> Posted -> Reversed; compile-time guarded transitions for posting and reversing.
- FxQuoteState: Created -> Active -> Expired -> Executed; execute requires Active.
- AccountState: Active -> Closed; posting requires Active unless admin correction flag.
Property-based tests (ledger invariants)
- prop_double_entry_sum_zero: for every tx, sum(entries.signed_amount) == 0 per currency.
- prop_no_negative_available: available >= 0 after any sequence of valid operations.
- prop_idempotency_replay_same_result: same key + same payload returns identical tx_id and balances.
---

<a id="p018"></a>
- prop_idempotency_conflict: same key + different payload returns 409 and no side effects.
- prop_reverse_is_compensating: reversing a tx yields net zero delta across affected accounts.
Integration vs unit tests
- Target ratio: ~60% integration (DB + HTTP) / 40% unit (pure business rules).
- Unit tests: rounding, fee calc, quote expiry logic, validation, state machine transitions.
- Integration tests: DB constraints, locking behavior, idempotency storage, full posting atomicity.
Load/performance tests (minimal MVP plan)
- load_post_transfers_steady: 50 rps transfers for 10 min; p95 < 200ms; no invariant violations.
- load_fx_quote_execute_burst: 20 rps quote + execute; ensure no duplicate postings.
- db_lock_contention_test: concurrent debits on same account -> exactly one succeeds; others fail deterministically.
Security tests (practical checklist)
- auth_required_all_endpoints: 401 without token; 403 for admin endpoints without ADMIN role.
- rate_limit_transfer_abuse: repeated calls hit 429 with stable error code.
- input_validation_fuzz: malformed JSON, negative amounts, overflow values -> 400 VALIDATION_ERROR.
- idempotency_key_enforcement: missing key on financial POST -> 400 VALIDATION_ERROR.
- audit_log_written: every admin action produces AuditEvent with correlation_id.
Monitoring/alerting (metrics + SLO hints)
- SLI: posting_success_rate, transfer_p95_latency, fx_execute_p95_latency.
- Errors: insufficient_funds_rate, idempotency_conflict_rate, db_serialization_fail_rate.
- Audit integrity: audit_event_write_failures must be 0 (P0 incident if non-zero).
- SLO hint: 99.9% of postings succeed without manual intervention; p95 < 300ms in-region.
---

<a id="p019"></a>
(6) MOBILE UI/UX MVP SPEC (UZUM UI KIT-ALIGNED) Information architecture / navigation
- Bottom navigation (3-4 tabs): Home, Transfers, History, Settings.
- Home is the default: balances + quick actions.
- All money actions are confirmable with a review screen + biometric/OTP (optional).
- Transaction receipt is shareable (PDF/image) and contains fx rate/fee when applicable.
MVP screen list (>= 12)
- S1 Splash / Session restore
- S2 Login (phone + OTP) / Sign-in
- S3 Home (wallet overview)
- S4 Accounts list (if multi-currency)
- S5 Account details (balance breakdown)
- S6 Transaction list (filters + pagination)
- S7 Transaction details / receipt
- S8 Transfer form (P2P)
- S9 Transfer review + confirm
- S10 FX convert form (quote)
- S11 FX quote review + execute result
- S12 Spend simulation (sandbox/developer mode)
- S13 Settings & Security
- S14 Limits (read-only in MVP) Screen specifications (implementation-ready) S3 Home (Wallet overview) Purpose: Show total and per-currency balances; provide quick actions.
---

<a id="p020"></a>
Components (Uzum UI Kit patterns):
- Balance cards per currency (Card + Typography)
- Primary CTA buttons: Transfer, Convert (FX), History
- Inline status banner (for outages/maintenance)
- Pull-to-refresh and skeleton loader States:
- Loading skeleton
- Normal state
- Error state (retry)
- Empty state (no accounts yet) Key actions:
- Open account details
- Start transfer
- Start FX quote
- Open transaction history S8 Transfer form (P2P) Purpose: Create an internal transfer between accounts.
Components (Uzum UI Kit patterns):
- From account selector (BottomSheet)
- To recipient selector (contact/ID) - MVP can be account_id input
- Amount input with currency display
- Memo input (optional)
- PrimaryButton Continue States:
- Validation errors (inline)
---

<a id="p021"></a>
- Insufficient funds pre-check (optional)
- Network error (snackbar) Key actions:
- Continue to review
- Edit fields
- Cancel S9 Transfer review + confirm Purpose: Confirm the transfer before posting; show fees (0 in MVP) and final amount.
Components (Uzum UI Kit patterns):
- Summary card (from/to/amount)
- PrimaryButton Confirm
- SecondaryButton Edit
- Biometric/OTP gate (optional) States:
- Posting in progress (spinner)
- Success -> show receipt
- Failure -> show error envelope code/message Key actions:
- Confirm (calls POST /v1/transfers with Idempotency-Key)
- Open receipt
- Repeat transfer S10 FX convert form (quote) Purpose: Request an FX quote for conversion between two accounts.
Components (Uzum UI Kit patterns):
- From/To selectors (BottomSheet)
---

<a id="p022"></a>
- Amount input (from amount)
- PrimaryButton Get quote States:
- Loading quote
- Quote returned
- Error (rate unavailable / validation) Key actions:
- Get quote (POST /v1/fx/quote)
- Change currencies/amount S11 FX quote review + execute Purpose: Show rate, fee, expiry and allow execute; show posted result receipt.
Components (Uzum UI Kit patterns):
- Quote card (rate, fee, expires_at)
- PrimaryButton Execute
- Countdown/expiry hint
- Result receipt on success States:
- Expired quote -> disable Execute
- Execution in progress
- Execution error -> show code/details Key actions:
- Execute (POST /v1/fx/execute with Idempotency-Key)
- View receipt
- Start new quote S6 Transaction list
---

<a id="p023"></a>
Purpose: Provide fast and reliable history view with filters. Components (Uzum UI Kit patterns):
- List with infinite scroll (cursor pagination)
- Filters: type (P2P/FX/Admin/Spend), currency
- Search by tx_id (optional)
- Empty state illustration States:
- Loading skeleton
- Empty
- Error + retry
- Pagination end Key actions:
- Open receipt
- Apply filter
- Pull-to-refresh S7 Transaction details / receipt Purpose: Single source of truth for what happened (used for support).
Components (Uzum UI Kit patterns):
- Receipt header (status, tx_id)
- Entries breakdown (from/to/system accounts)
- FX details (rate, fee) when applicable
- Share button States:
- Normal
- Not found
---

<a id="p024"></a>
- Error Key actions:
- Share receipt
- Copy tx_id
- Report issue (deferred) S12 Spend simulation (sandbox) Purpose: Developer/support-only feature to simulate merchant spend in MVP.
Components (Uzum UI Kit patterns):
- Toggle 'Sandbox mode' in Settings
- Merchant label input
- Amount input
- PrimaryButton Simulate spend States:
- Hidden when sandbox mode disabled
- Posting in progress
- Success shows receipt Key actions:
- Simulate spend (POST /v1/spend/simulate with Idempotency-Key) Critical flows (MVP)
- Onboarding/login -> Home -> balance refresh.
- Admin funds user -> user sees balance updated -> makes P2P transfer.
- FX quote -> review -> execute -> receipt shows rate + fee.
- Spend simulate -> history shows merchant_label and system spend account.
- Settings -> security options (biometric toggle, session logout).
---

<a id="p025"></a>
Modern wallet UX principles (Revolut-like)
- Always show available balance and keep it consistent with receipts.
- Every write action has a review step and produces a receipt with a stable tx_id.
- Errors are actionable: show user-friendly text + hidden technical error.code for support.
- Fast history: stable ordering, cursor pagination, cached last page for offline view (optional).
- Transparency on FX: rate, fee, expiry, and final credited amount are visible before execution.
---

<a id="p026"></a>
(7) TEAM / PERSON-HOURS / DELIVERY PLAN (MVP REALITY CHECK) Minimal team composition (1-2 month MVP)
- 1x Tech Lead / Staff engineer (ledger correctness + contracts + code review gate).
- 2x Backend engineers (ledger engine, API, Postgres migration, observability).
- 1x Android engineer + 1x iOS engineer (Uzum UI Kit, screens, API integration).
- 1x QA engineer (E2E automation + regression + release sign-off).
- 0.5x DevOps/SRE (infra, CI/CD, migrations, monitoring, backups).
- 0.25x Legal/Compliance advisor (requirements review + regulator alignment).
Person-hours estimate (brutal MVP numbers)
| Track | Scope | Person-hours (MVP) |
| --- | --- | --- |
| Backend | Ledger, idempotency, P2P, admin ops, FX quote/execute, history/receipts, audit logs | 520 - 700 |
| Mobile (Android) | Home, transfer, FX, history, settings, error handling, QA fixes | 280 - 360 |
| Mobile (iOS) | Same as Android | 280 - 360 |
| QA/E2E | E2E suite (>=10), contract tests, regression, test data harness | 220 - 320 |
| DevOps | Postgres setup, migrations, secrets, monitoring, backups, staging/prod deploy | 120 - 180 |
| Compliance support | Data/privacy/AML baseline review + documentation | 30 - 60 |
| Critical path (what blocks release) | - OpenAPI locked + client integration stable (no moving targets). | - PostgreSQL ledger schema + atomic posting engine + concurrency strategy verified. |
| - Idempotency middleware enforced on all money endpoints. | - FX rounding + quote expiry correctness (no silent losses). | - E2E suite green on staging with seeded data; rollback + backup/restore ready. |
What we can cut if schedule is impossible
- Cut sandbox spend screen (keep endpoint for internal testing only).
---

<a id="p027"></a>
- Reduce FX to 1 pair only (UZS<->USD) with fixed markup; defer multi-hop conversions.
- Ship minimal admin UI as CLI-only (still keep API) to avoid frontend load.
- Limit history filters and keep only basic list + receipt.
- Defer device binding and keep only token/session controls.
---

<a id="p028"></a>
(8) UZBEKISTAN COMPLIANCE RESEARCH (2025–2026) This section lists the most relevant Uzbekistan regulations for a wallet/ledger product and translates them into concrete engineering controls. MVP scope has no KYC and no external payment rails, but correctness, privacy, and auditability are mandatory. Relevant regulators / frameworks
- Central Bank of the Republic of Uzbekistan (CBU): payments ecosystem oversight and regulatory framework references. [R4]
- LexUZ national legislation portal: authoritative legal texts for payments, personal data, AML/CFT, cybersecurity. [R1][R2][R3][R6]
- Uzbekistan personal data regime (operator duties, security and confidentiality obligations). [R2]
- Cybersecurity framework for Critical Information Infrastructure (KII) may apply depending on classification. [R6] KYC/AML requirements baseline (MVP interpretation)
- AML/CFT law defines a compliance environment for financial transactions and related obligations for regulated institutions. [R3]
- MVP engineering requirement: audit-ready ledger (immutable journal, traceability, admin accountability) so KYC/AML can be added post-MVP without redesign.
- Add basic monitoring hooks: velocity limits, repeated failed transfers, suspicious admin actions; store flags in audit_event.
Data retention / privacy expectations (engineering view)
- Personal data must be processed lawfully with confidentiality and protection controls. [R2]
- Maintain a data inventory and minimize sensitive fields; do not store unnecessary documents/PII in MVP.
- Data localization rules and cross-border processing are evolving; keep an in-country deploy option and make cross-border a legal decision. [R2][R8] Reporting / audit requirements (high-level)
- Immutable financial history + receipts support internal finance reconciliation and potential regulator audits.
- Admin actions must be fully attributable (who/when/what/why).
- Retain audit logs with defined retention window (suggestion: >= 5 years for financial journal; policy decision).
Requirement -> Engineering control
| Requirement | Engineering control (MVP) |
| --- | --- |
| Payments legal basis awareness | Keep MVP boundary as internal ledger feature; external rails only via licensed entity integration. [R1][R4] |
| Personal data protection | RBAC, access logs, encryption in transit, minimal PII, retention policy. [R2] |
---

<a id="p029"></a>
Cybersecurity baseline Secure coding + TLS + rate limits; if classified as KII, apply PP-167 controls and assessment readiness. [R6] AML/CFT readiness Audit-ready ledger + monitoring hooks; support extraction of transaction reports. [R3] Minimal compliance MVP checklist
- Scope statement in product/legal docs: MVP is internal wallet ledger; external rails and KYC are deferred.
- Security baseline: RBAC + audit logs for admin endpoints; secrets managed; TLS everywhere.
- Operational baseline: Postgres backups + PITR; restore drill executed; migrations gated in CI.
- Incident readiness: on-call owner; runbook for balance discrepancy investigation using tx_id receipts.
---

<a id="p030"></a>
(9) OPEN QUESTIONS (DECISION LIST)

Q: What legal entity operates the wallet ledger (bank vs payment org vs internal feature)?
Why it matters: Determines licensing/compliance scope and whether external rails can be added.
What changes depending on answer: Affects auth model, regulator reporting, and future integrations.
Default assumption: Default: treat as internal feature under Uzum ecosystem licensed entity; external rails out of MVP.

Q: Which currencies are in MVP (UZS only vs UZS+USD vs more)?
Why it matters: FX complexity and rounding/fee logic depends on supported pairs.
What changes depending on answer: Affects UI (account list) and reconciliation scripts.
Default assumption: Default: UZS + USD only.

Q: FX rate source: Central Bank reference, partner provider, or fixed config?
Why it matters: Incorrect rate source causes revenue leakage or user disputes.
What changes depending on answer: Affects quote validity, refresh frequency, and transparency rules.
Default assumption: Default: fixed configurable rate table + markup for MVP; replace later.

Q: Is FX fee a flat fee, percentage, or markup-in-rate?
Why it matters: Changes receipts, product pricing, and ledger postings.
What changes depending on answer: Affects rounding and user-visible breakdown.
Default assumption: Default: small % fee posted to SYSTEM_FX_FEE.

Q: Retention policy for journal + audit logs (months/years)?
Why it matters: Impacts storage costs and compliance posture.
What changes depending on answer: Affects DB partitioning and backup strategy.
Default assumption: Default: financial journal >= 5 years; audit_event >= 2 years (policy confirmation needed).

Q: Admin operations governance: 1-person authority vs 4-eyes approval?
Why it matters: Insider risk and operational safety in fintech.
---

<a id="p031"></a>
What changes depending on answer: Affects admin workflow and audit controls.
Default assumption: Default: 1-person admin in MVP + full audit log; 4-eyes in post-MVP.

Q: User authentication method: Uzum SSO vs standalone OTP?
Why it matters: Impacts onboarding speed and security boundary.
What changes depending on answer: Affects token issuance, session revocation, device binding.
Default assumption: Default: integrate with Uzum identity JWT via gateway.

Q: Do we need 'blocked/holds' in MVP?
Why it matters: Hold/capture introduces complexity but required for real merchant flows.
What changes depending on answer: Affects balance model and APIs.
Default assumption: Default: blocked=0; holds deferred.

Q: Is 'shared/family wallet' on near-term roadmap (Q2) or later?
Why it matters: Impacts data model (wallet groups) and permissions design.
What changes depending on answer: Affects whether to invest in tenant/wallet-group now.
Default assumption: Default: roadmap item only; schema reserves wallet_group_id.
---

<a id="p032"></a>
v3.1 Backend Definition of Done (DoD) Release cannot ship until all items below are true.
- Endpoints implemented under /v1: admin accounts create/close, fund, withdraw, reverse, search; user transfers; spend simulate; fx quote/execute; history; tx receipt.
- OpenAPI published and validated in CI; schema matches runtime behavior.
- Idempotency enforced on all money POST endpoints; 409 IDEMPOTENCY_CONFLICT on payload mismatch.
- Ledger invariants always pass (double-entry sum=0; no negative available).
- Integration tests: P2P success + insufficient funds; idempotency replay + conflict; FX quote/execute + expiry; spend simulate posts once; admin fund/withdraw/reverse correctness.
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
