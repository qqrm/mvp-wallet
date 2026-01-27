# Source materials (reference)

These materials were imported and cleaned to help compile the v3.3 documentation set. They are included only for traceability and historical context; the repository documentation should be treated as the single v3.3 source of truth.

## Input A
_Note: Compiled from earlier drafts (legacy)._


- MVP boundaries: Wallet balances + double-entry ledger, internal P2P transfers, FX conversion (quote + execute), admin mint/burn/reverse, transaction history receipts, sandbox spend simulation.
- Explicitly NOT in MVP: KYC, external rails (cards/top-ups/bank withdrawals), merchant checkout integration, chargebacks/disputes automation, credit/insurance.
- Core guarantee: Immutable journal + atomic postings + strict idempotency on all financial endpoints.
- Persistence: PostgreSQL required for production; SQLite allowed only for dev/tests with migration plan.
---

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

25 Reliability No isolation level decision. Use SERIALIZABLE or SELECT ... FOR UPDATE on balance rows; document approach. P0 26 Backend Logic No spend/authorization model; unclear hold/capture. For MVP: spend = direct posted debit; holds out-of-scope but reserved. P2 27 UI/UX No clear UX states (loading/empty/error). Specify states for each screen; skeleton loaders; retry actions. P1 28 Compliance AML monitoring baseline not discussed. Add velocity rules, auditability, and reporting readiness (post-MVP KYC). P2 29 Roadmap FX fee strategy not explicit. Define fee/markup model with config + transparency on receipt. P1 30 Testing No explicit DoD acceptance criteria. Add v3.1 Backend DoD: OpenAPI, tests, invariants, metrics, and audit logs.P0
---

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

Modern wallet UX principles (Revolut-like)
- Always show available balance and keep it consistent with receipts.
- Every write action has a review step and produces a receipt with a stable tx_id.
- Errors are actionable: show user-friendly text + hidden technical error.code for support.
- Fast history: stable ordering, cursor pagination, cached last page for offline view (optional).
- Transparency on FX: rate, fee, expiry, and final credited amount are visible before execution.
---

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

- Reduce FX to 1 pair only (UZS<->USD) with fixed markup; defer multi-hop conversions.
- Ship minimal admin UI as CLI-only (still keep API) to avoid frontend load.
- Limit history filters and keep only basic list + receipt.
- Defer device binding and keep only token/session controls.
---

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

Cybersecurity baseline Secure coding + TLS + rate limits; if classified as KII, apply PP-167 controls and assessment readiness. [R6] AML/CFT readiness Audit-ready ledger + monitoring hooks; support extraction of transaction reports. [R3] Minimal compliance MVP checklist
- Scope statement in product/legal docs: MVP is internal wallet ledger; external rails and KYC are deferred.
- Security baseline: RBAC + audit logs for admin endpoints; secrets managed; TLS everywhere.
- Operational baseline: Postgres backups + PITR; restore drill executed; migrations gated in CI.
- Incident readiness: on-call owner; runbook for balance discrepancy investigation using tx_id receipts.
---

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

## Input B
_Note: Compiled from earlier drafts (legacy)._


Version 3.2 – Contract-First Monolith Architecture (Uzbekistan) Executive Summary MVP boundaries: Multi-currency wallet with user balances and a double-entry ledger, internal P2P transfers, FX conversion (quote + execute), admin ledger operations (mint, burn, reverse), transaction history with receipts, and a sandbox “spend” simulation Explicitly NOT in MVP: KYC onboarding, external payment rails (no card or bank top-ups/ withdrawals), merchant payment integration, automated dispute/chargeback handling, or credit/ loan features . These are deferred beyond MVP. Core guarantees: Immutable journal (no deletion or alteration of posted transactions) and atomic multi-ledger postings, with strict idempotency enforcement on all money-moving endpoints Every financial request is processed exactly once, or not at all, even under retries. Infrastructure: Back-end is a contract-first monolithic service implemented in Rust (replacing the earlier Go approach) for safety and performance. Data persists in PostgreSQL (required for production) with strong consistency; SQLite can be used for local dev/testing with migrations to Postgres . Full audit logging and basic monitoring are built in. MVP v3.2 updates: No change to the core scope or architecture from v3.1 . The v3.2 iteration focuses on removing ambiguity in the specification and tightening definitions (e.g. how recipients are identified, how FX rates are obtained, idempotency key retention) . It adds a Product Decision Record (Q&A) section addressing these design choices, and expands the post-MVP feature roadmap based on a comparative analysis of leading wallets Changelog (v3.1 → v3.2) P2P Recipient Addressing: Extended internal transfer capability to allow specifying the recipient by phone_number or user_id (in addition to account_id ). The service will resolve such identifiers to the target account_id at transaction time, using the user directory. Note: to prevent deadlocks, the transfer logic will always lock accounts in a consistent order (e.g. by user_id ) Admin Search API Filters: Defined the admin search endpoint ( GET /v1/admin/search ) to accept minimal query filters – user_id , account_id , tx_id, currency, time_range – returning up to 100 matching transactions or accounts. Clarified that pagination can be handled on the frontend (no server-side cursor for MVP) FX Rate Source: Locked FX conversion in MVP to use a static fx_rates table in the database as the source of currency exchange rates. The POST /v1/fx/quote operation will read from this table (with pre-configured rates/markup) . Future integrations with public FX APIs or dynamic feeds are noted as post-MVP enhancements, but for MVP all quotes use the internal table. Each FX quote record captures the rate (and fee) at quote time as an immutable snapshot for transparency. Idempotency Key Retention: Established a standard 48-hour TTL for stored idempotency keys. Idempotency records (used to prevent double-processing of POST requests) will be persisted for 48
---

hours, after which they may be purged/expired. This ensures clients can safely retry for a reasonable window without persistent storage growth Transaction Receipt Schema: Enhanced the transaction receipt model to include a complete breakdown of the transaction details. Receipts (returned by GET /v1/transactions/{tx_id} ) will list all involved accounts with amounts and debit/credit direction, the transaction type and timestamp, and any FX rate and fee applied (for conversion transactions). An optional memo or merchant_label is included for user-friendly context (e.g. user-entered note or merchant name on a spend). The API will exclude internal-only fields, showing only user-meaningful data (e.g. no raw internal IDs besides the transaction ID reference) Product Decision Record (PDR): Added a Q&A style section summarizing key product decisions resolved in v3.2 (e.g. choice of 48h idempotency retention, using a static FX rate table, handling of history pagination) along with brief explanations Extended Post-MVP Roadmap: Appended a feature backlog derived from comparative market analysis, grouping and prioritizing future enhancements in categories (Payments/Rails, Compliance, UX, Wallet Capabilities, Ops & Support). High-priority items include phone-number P2P and QR payments, loyalty rewards integration, installment (BNPL) features, joint/family accounts, virtual/ physical cards, merchant/SME tools, and a potential insurance/investments marketplace, among others . These are not in MVP scope but inform the next phases. Product Scope & Personas End-User Persona: A consumer in the Uzum ecosystem who holds money in local (UZS) and foreign (USD) currency balances. They need to send and receive money easily with other users and perform transparent currency exchange, with an immediate immutable receipt for each transaction. In MVP, users can transfer funds internally to other users by selecting a contact (now by phone number or user ID) or entering an account number . They can also convert money between currencies within their wallet. The experience emphasizes speed (instant transfers), no fees for internal P2P, and clear confirmation of each transaction (balance updates and detailed receipts). Admin/Operations Persona: Back-office staff or support agents who manage the ledger system. They can create or close user accounts, credit or debit user balances (mint or withdraw), and perform compensating reversals to correct errors . They have basic search tools to look up transactions or accounts by various identifiers for customer support and reconciliation. MVP assumes a single-admin authority model (no dual approval) but with full audit logging of all admin actions. Admins use the search API to filter transactions (by user, account, etc.) when investigating issues or assisting users MVP Scope Summary: The MVP covers multi-currency wallet accounts per user (UZS and USD), an internal double-entry ledger for all transfers, and basic wallet operations: internal P2P transfers, FX conversion (quote & execute), viewing transaction history with receipts, admin ledger controls (open/close accounts, mint, burn, reverse), and a sandbox spend simulation . It does not include external top-ups or withdrawals (no card/bank linkages), merchant payments, or KYC onboarding flows – those are deferred to post-MVP phases
---

Domain Model & Data Invariants Core Entities: User – End-user record (with unique user_id), including personal identity info (phone, email, etc.), status flags, and linkage to one or more wallet accounts. Each user belongs to a tenant (Uzum ecosystem tenant, single-tenant for now) and can have one default account per currency. Account – Ledger account representing a wallet balance in a specific currency. Key fields: account_id, user_id (owner), currency, status, created_at. Each user/currency pair has one account (MVP supports UZS and USD accounts per user). Accounts can be ACTIVE or CLOSED (closed accounts are suspended from use, but never physically deleted). JournalTx – A financial transaction header (journal entry) with tx_id (unique), type (e.g. P2P transfer, FX, admin adjustment), status, timestamp, and correlation fields. Every money movement in the system is a JournalTx, which links to one or more JournalEntry lines. Once posted, JournalTx records are immutable (no updates/deletes) as part of the audit trail invariant. JournalEntry – The double-entry line items associated with a JournalTx. Each entry records an account_id, amount, and side (debit or credit). For each JournalTx, total debits equal total credits in each currency, ensuring ledger balance. JournalEntries also carry a reference to the tx_id and perhaps a short description (e.g. “P2P transfer from X to Y”). FxQuote – An FX conversion quote record. Created when a user requests a currency conversion quote. Fields: quote_id, from_account_id, to_account_id, amount_from, rate, fee, expires_at, status. The quote holds a snapshot of the exchange rate (and any fee or markup) offered to the user, and an expiration timestamp (e.g. 60 seconds from quote creation) after which the quote can no longer be executed. In v3.2, quotes use the static FX rate table as source; the rate and fee stored in FxQuote represent the exact terms given to the user at that time. (No separate version ID for the rate is needed, since the quote records the actual rate used.) FxRate – (New in v3.2) A static reference table of currency exchange rates. This table holds the authoritative rates used by the wallet for FX quotes in MVP. Fields might include currency pair (e.g. UZS/USD), current rate, an optional markup or fee percentage, and a last_updated timestamp. The MVP will preload or maintain this table with the applicable market rates (e.g. derived from Central Bank or internal treasury) and possibly a small markup. All FX quote requests will read from this table to determine the rate/fee offered. (In future, this could be replaced or updated via external API feeds – see Product Decision Record and Roadmap for details.) IdempotencyRecord – Record of a past POST request identified by an Idempotency-Key. Contains the key value, the requesting actor_id (user or admin), the endpoint and a hash of the request payload, plus the resultant status code and response body, and a timestamp. In v3.2, an additional field/attribute tracks the expiration of this record (TTL). The system retains idempotency records for 48 hours from creation, after which they may be purged . This prevents indefinite growth of the table while covering typical client retry windows. AuditEvent – Append-only log of significant actions and system events for security and compliance. Each event has an event_id, actor info (who performed the action and their role/permissions), action type (e.g. ADMIN_FUND, USER_TRANSFER), target identifiers (which user/account/tx were affected), a timestamp, and context like IP or device ID. All admin operations and all financial transactions trigger audit events. These are retained long-term (at least 2–5 years as per compliance needs).
---

Data Invariants: Ledger Immutability: Once a JournalTx and its JournalEntries are marked POSTED/COMPLETED, they cannot be altered or deleted. Corrections are done via new transactions (e.g. a reversal) rather than editing history. This guarantees an immutable ledger for audit and reconciliation Double-Entry Balance: For every JournalTx, the sum of debit entries equals sum of credit entries in each currency involved. This ensures no money is created or lost in transit. The system enforces this with a DB check constraint or in-code validation. Consistent Account States: Account statuses and balances follow defined rules. For MVP, blocked balance is always zero (holds are not used in MVP ), so an account’s available balance equals its total ledger balance. If in future “holds” are introduced (for card authorizations, etc.), the schema will adjust, but MVP assumes no pending holds. Closing an account (status = CLOSED) prevents new debits/credits but does not remove the record or its balance history. Tenant and Group Structure: MVP is single-tenant (all users under the same umbrella). The schema has a placeholder for tenant_id on user/accounts for future multi-tenant support. Similarly, while MVP is single-owner per account, the schema reserves a wallet_group_id field to support shared/ joint wallets in the future (e.g. family accounts). No group functionality is active in MVP beyond this placeholder Balance Model & Transaction Lifecycle Account Lifecycle: Accounts start as ACTIVE and can be CLOSED by an admin (closed accounts cannot be used for new transactions, but their history remains) . MVP does not implement account freezes or holds beyond this status change. Internal Transfer (P2P): Validate both source and recipient accounts are ACTIVE and source has sufficient balance, then create a JournalTx with two JournalEntries (debit sender’s account and credit receiver’s account) . The transfer posts atomically; on success, both accounts’ balances update. Admin Fund (Mint): An admin can credit a user’s balance by “minting” money into the system. The JournalTx debits a system mint account (SYSTEM_MINT) and credits the user’s account Admin Withdraw (Burn): An admin can deduct a user’s balance by “burning” money out of circulation. The JournalTx debits the user’s account and credits a system sink account (SYSTEM_SINK) . (This models cash-out in MVP, since external outflows are not integrated.) Admin Reverse: To correct mistakes, an admin can reverse a prior transaction by creating a compensating JournalTx that debits the account that was previously credited and credits the one that was debited . Every reversal is a new transaction with its own tx_id, linked to the original transaction for audit. Spend Simulation: The user can simulate a spend (for testing) which transfers an amount from the user’s account to a designated system spend account (SYSTEM_SPEND), optionally attaching a merchant label or note . This does not involve external parties but helps test the end-to-end flow (debit user, credit system account) and receipt generation. API Specification & Contracts User API Endpoints ( /v1/* ): POST /v1/transfers – Create an internal P2P transfer from one user account to another. Idempotent: Yes (Idempotency-Key required)
---

POST /v1/fx/quote – Request a currency exchange quote (two-step FX flow). Returns a quote record with rate, fee, and an expiration timestamp . Idempotent: Yes (client can safely retry; each attempt generates a new quote). POST /v1/fx/execute – Execute a previously obtained FX quote, performing the debit and credit in respective accounts if the quote is still valid . Idempotent: Yes (only one execution succeeds; duplicate calls return the same result or an error) POST /v1/spend/simulate – Simulate a spend (transfer to a system spend account) for testing or sandbox use . Idempotent: Yes. GET /v1/accounts/{id} – Fetch account details (status, currency, created_at) for a user’s account. GET /v1/accounts/{id}/balance – Fetch the balance breakdown of a user’s account (available and blocked amounts). Blocked will be 0 in MVP (no holds). GET /v1/accounts/{id}/transactions?cursor=&limit= – List transaction history for an account (paginated by cursor). Returns recent transactions with their receipt details. GET /v1/transactions/{tx_id} – Retrieve the full receipt for a specific transaction (if the requesting user has access to it). Includes details of the transaction, amounts, and updated balances Admin API Endpoints ( /v1/admin/* ): POST /v1/admin/accounts/{id}/close – Close (suspend) a user account. Closed accounts cannot be used for new transactions (no deletions in ledger) POST /v1/admin/fund – Mint funds into a user’s account (increase balance, credit user, debit system mint pool) POST /v1/admin/withdraw – Burn funds from a user’s account (decrease balance, debit user, credit system sink) POST /v1/admin/reverse – Reverse a previous transaction by tx_id (create compensating journal entries to negate a specific transaction) GET /v1/admin/search – Search for transactions or accounts by filters (e.g. user_id, account_id, tx_id, currency, time range) . Returns up to 100 results per query (no server-side pagination; refine query to narrow results). Used for ops and support lookups. All POST endpoints that modify financial state require an Idempotency-Key header and implement robust idempotency (duplicate requests with the same key will not be processed twice) . GET endpoints are side-effect-free and do not require idempotency keys. All list endpoints use cursor-based pagination (with limit and cursor parameters) for stable ordering Error Handling: Errors are returned with HTTP appropriate status codes and a JSON body containing an error object. For example, a failed transfer due to insufficient funds might return: { "error": { "code": "INSUFFICIENT_FUNDS", "message": "Not enough available balance", "details": { "account_id": "acc_sender", "available": 120000, "required": 200000 }
---

} } All error codes are predefined (e.g. VALIDATION_ERROR , INSUFFICIENT_FUNDS , IDEMPOTENCY_CONFLICT , etc.) and the error response format is consistent across endpoints Idempotency & Concurrency Every money-moving POST request must include a unique Idempotency-Key. The system stores the outcome of each request by key; a retry with the same key returns the saved result (without re- executing) . If a repeat uses the same key but a different payload, the request is rejected with 409 IDEMPOTENCY_CONFLICT Idempotency records are retained for 48 hours to allow safe client retries, after which they expire and may be purged . This TTL prevents unbounded growth of the idempotency log while covering typical retry windows Concurrency: The service ensures only one transaction modifies a given account’s balance at a time. Each transfer or balance-changing operation acquires a write lock on the affected account rows (e.g. using SELECT ... FOR UPDATE ) or runs in a serialized transaction . For operations involving two accounts (like P2P transfers), accounts are locked in a consistent order (e.g. by sorted user/ account ID) to avoid deadlocks Security Architecture Authentication & Authorization: All endpoints require a valid JWT issued by Uzum’s central identity (SSO) system . User tokens grant access only to their own resources (enforced by checking the token’s user_id against the requested data) . Admin endpoints require an admin-scoped token; all such actions are additionally recorded in audit logs with the acting admin’s identity Access Control: Role-based access is enforced (regular users cannot call admin APIs). Within user- facing APIs, each request is also authorized against the owning user’s ID to prevent data leaks (e.g., a user cannot fetch another user’s account or transactions) Transport Security: All communication is over HTTPS with JWT-based auth as above. The wallet service is assumed to run in a secure network environment (within Uzum’s cloud/VPC) and behind an API gateway that handles TLS termination and basic protections. Rate Limiting: To prevent abuse, the system should enforce basic rate limits on critical endpoints (e.g. transfers, login) per user and per IP. This is planned as part of the deployment configuration (e.g. using an API gateway or middleware) . No fine-grained fraud rules are in MVP beyond these limits and the AuditEvent logging. Sensitive Data: The wallet does not store highly sensitive personal data in MVP (no full KYC info), and payment data stays internal. Standard data protection measures (encryption at rest, GDPR/ Uzbek PDPL compliance) are followed as applicable . In the future, if external integrations are added, additional PCI/PII safeguards will be introduced. Compliance & Regulatory Considerations Licensing & Operational Entity: For MVP, the wallet operates under Uzum Bank’s existing financial license as an internal product feature (closed-loop within Uzum ecosystem) . This avoids needing
---

a separate e-money license at launch. All stored value remains within the regulated bank entity, and external cash-out is simulated via admin operations. Personal Data Law: User data handling aligns with Uzbekistan’s Law on Personal Data Protection. All personal data is stored in approved jurisdictions (onshore as required) and protected per regulation (consent records, etc.) . Although MVP collects minimal PII (only phone/email), we ensure proper data security and will register with authorities if needed. AML/CFT: Even though full KYC is deferred, the system maintains auditability of all transactions to support anti-money-laundering oversight . Unusual activity can be detected via the audit logs. Prior to enabling external transfers, Uzum will integrate formal KYC verification and AML transaction monitoring rules (e.g., sanction list screening, suspicious pattern flags) to meet regulatory requirements. Consumer Protection: All transactions produce immutable receipts and audit trails to resolve disputes. There are no automated dispute reversal workflows in MVP (issues are handled manually by admins), but logs ensure accountability. Limits on transfer amounts and velocity can be imposed by configuration if required by regulation. Observability & Monitoring Audit Logging: Every critical action is recorded in an audit_event log (see AuditEvent entity) with timestamp, actor, action type, and context . This applies to all admin operations and financial transactions. Audit logs are immutable and retained long-term (at least 2–5 years) for compliance and forensic purposes. Metrics & Monitoring: The system exposes basic operational metrics: e.g. transaction throughput, latency percentiles (p95), error rates (insufficient_funds errors, idempotency conflicts, DB serialization retries) . These metrics can be collected and alerted on via the monitoring infrastructure. Unusual events (like repeated failed transfers or rapid balance changes) can be flagged for review (though MVP does not include an automated alerting module beyond what the ops team sets up). Tracing & Correlation: Each request carries a correlation_id and is traceable end-to-end. JournalTx records and audit events include this correlation_id (and the initiating user/admin) to tie together log entries for a single operation, simplifying troubleshooting across distributed components. Reliability & Performance Patterns Atomicity: All ledger updates (creating a JournalTx and its JournalEntry lines, updating balances) occur within a single database transaction that either fully succeeds or rolls back . This guarantees that partial updates cannot occur (no stuck half-posted transfers). Outbox for Integrations: The design includes a placeholder for an outbox table to record events for external integrations (e.g. sending notifications or integrating with external payment networks). MVP does not require external messaging, so the outbox is not actively used, but the schema is prepared for future use (ensuring eventual consistency if integrating with other services) Isolation Level: To maintain consistency under concurrent load, the database transaction isolation level can be set to SERIALIZABLE or we use explicit row locks on all affected accounts during updates . This prevents race conditions like lost updates to balances. In testing, we simulate high concurrency (e.g. 50 transfers/sec for 10 min) to ensure no invariant violations Scalability: The MVP monolith is expected to handle initial load within a single service instance and database. We identified no immediate performance bottlenecks: critical paths (transfers, FX) were
---

profiled to meet sub-200ms response times at moderate throughput . As usage grows, vertical scaling of the DB and read replicas for history queries can be employed, and the service can scale horizontally behind a load balancer if needed (stateless API nodes relying on the shared DB). Testing Strategy The implementation will include comprehensive automated tests: Unit Tests & Property Tests: Core business logic (ledger postings, balance updates, idempotency handling) is covered by unit tests. Property-based tests validate key invariants (e.g. no net money creation, idempotent replays yield same result) Integration Tests: The full stack is tested with in-memory or test DB to ensure end-to-end flows work (e.g. funding -> transfer -> balances update -> receipt correctness) . Each API endpoint has contract tests to verify request/response schemas match the OpenAPI specification. Scenario E2E Tests: Critical user journeys are exercised in sequence. For example: admin funds an account, user transfers funds to another user, user executes an FX conversion, etc., checking final balances and receipts at each step . Error scenarios (insufficient funds, invalid inputs, duplicate requests) are also tested to ensure proper error codes and no side effects. Performance Testing: Basic load tests simulate concurrent usage (e.g. sustained transfers per second, bursts of FX quotes) to verify the system maintains acceptable latency and does not violate consistency. The goal is p95 latency under 200ms for typical operations under expected MVP load . Any deadlock or serialization retry issues observed in testing will be tuned (e.g. adjusting indexing or lock granularity). MVP Delivery Plan Week 1: Finalize the API contracts (OpenAPI spec) and database schema. Implement the core ledger posting engine (journal entries, account balance updates) and the idempotency middleware Week 2: Implement all admin-facing APIs (account create/close, fund, withdraw, reverse) and user- facing P2P transfer endpoints. Build the transaction history endpoint and basic transaction receipt model Week 3: Implement the FX conversion flow (quote and execute) and the sandbox spend simulation. Add observability hooks (logging, metrics) and ensure audit logging is capturing all events. Week 4: Integrate the backend with the mobile app UI (using Uzum UI components). Conduct end- to-end testing of all user flows and perform bug fixes and refinements. Prepare deployment and security review. Post-MVP Feature Roadmap
1. Payments & Rails Enhancements:
External P2P by Phone/Card: Expand peer-to-peer transfers beyond the internal wallet. Allow sending money to any user via phone number or even directly to a card number, integrating with local instant payment networks (e.g. Uzcard/Humo) . This would let users send money to people not yet on Uzum (or to any card), greatly increasing reach and utility. QR Code Merchant Payments: Introduce the ability to pay merchants via QR codes. Users can scan a merchant’s QR in-store to transfer payment from their wallet . This likely involves supporting a
---

national standard (or Uzum’s own QR format) and tying into Uzum Bank’s acquiring business (Uzum Pay). High priority, as QR payments are popular in the region and enable offline acceptance. Bills & Utilities Integration: Let users pay utility bills, top-up mobile airtime, taxes, and other common bills directly from the wallet . This requires integrations with biller aggregators or relevant APIs. Most competitor wallets offer extensive bill-pay options, driving frequent engagement. Uzum Bank already supports some bill payments on web; bringing them into the app would increase usage. Virtual & Physical Cards: Offer wallet-linked payment cards. Users could get a virtual card instantly (for online payments or to add to Apple/Google Pay) and optionally order a physical Visa/Mastercard . Card controls (set PIN, freeze/unfreeze, spending limits) would be provided. This extends wallet usage to any POS or online merchant and leverages Uzum Bank’s card issuance capabilities (noting Uzum has begun issuing cards). High priority for parity with modern wallets.
2. Compliance & Security:
KYC & Identity Verification: Implement tiered Know-Your-Customer verification for wallet users before enabling external money flows or higher limits. This could integrate digital ID checks, document upload, or bank eKYC services . MVP defers full KYC, but it will be mandatory later to comply with regulations when broadening access. Automated AML Transaction Monitoring: As volume grows, deploy rules or machine learning to detect suspicious activities . For example, flag rapid in/out transfers, large transactions, or known-risk accounts. These could feed alerts to compliance officers. This enhancement becomes crucial once external transfers are allowed and volumes increase (medium-high priority post-MVP). Advanced Login Security: Enhance user authentication with features like biometric login (fingerprint/FaceID) and device management . This improves security and UX. Also consider adaptive authentication (step-up verification for high-risk actions) and session management tools (viewing active sessions, remote logout). These are standard in leading fintech apps and would boost user trust. Fraud Prevention Tools: Add safeguards such as transaction OTP confirmations for large payments, velocity limits, and integration with any national anti-fraud systems . Longer-term, incorporate AI- based fraud detection to analyze user behavior and flag anomalies (as some global apps do). These features are planned as the platform scales to protect users and the business.
3. User Experience (UX) Enhancements:
Saved Recipients & Contacts: Allow users to save frequent recipients and import contacts to find other Uzum Wallet users easily . Instead of entering details each time, users can simply select a saved name. This speeds up P2P transfers and leverages social connections (future versions might sync phone contacts to suggest friends who have the wallet ). Enhanced History & Insights: Provide richer transaction history features in the app. For example, add filters (by date, type, amount) and search in the user’s transaction list . Additionally, personal finance insights like categorizing spending, monthly summaries, or budgeting tools can increase engagement. MVP keeps history basic, but these can differentiate the app later. Multi-Language Support: Localize the app UI and receipts into multiple languages (Uzbek, Russian, English, etc.) . As Uzum’s user base is multilingual, providing content in users’ preferred language improves accessibility. This involves translating static text and possibly supporting different currency formats/symbols.
---

UI Personalization: Offer features like Dark Mode, custom themes, and quick action shortcuts for common tasks . While lower priority, these polish the user experience and meet modern user expectations. They can be added incrementally once core features are stable.
4. Wallet Capabilities Expansion:
Unified Loyalty & Rewards: Integrate Uzum ecosystem loyalty programs (e.g. Uzum Market cashback or points) into the wallet . For example, users could earn points or cashback for using Uzum Wallet or shopping in the marketplace, visible in the wallet. This encourages usage across Uzum’s services. Implementing this may involve a points ledger or tracking rewards as a separate balance/currency. Installments & Pay-Later (BNPL): Embed Uzum’s “Nasiya” installment plan or similar buy-now-pay- later features directly into wallet payments . At checkout, users could split a purchase into installments or get a micro-loan in one tap. Many competitors have had success with in-app credit options (e.g. Kaspi’s BNPL). This would leverage Uzum Bank’s lending capabilities and require linking transaction flows with credit approvals. Shared/Family Wallets: Support joint accounts or family wallets where multiple users can share access to funds . For example, a couple could have a shared wallet, or a parent could oversee a teen’s sub-account. This requires adding wallet group management and permission controls, which have been anticipated by reserving wallet_group_id in the schema. It’s a medium priority feature that could differentiate Uzum Wallet in the region. Expanded Multi-Currency: Beyond UZS and USD, consider adding more currency accounts (EUR, RUB, etc.) if user demand arises . Features like real-time FX rate alerts, the ability to hold multiple currencies and exchange at user-chosen times (like Revolut’s model) could attract users with international needs. This positions the wallet closer to a multi-currency finance app for travelers or savers. Merchant & SME Features: Develop a version of the wallet for small business users. This could include quick onboarding for merchants, the ability to receive customer payments (potentially via the QR system above), payout to suppliers or employees, and tools like transaction reports or analytics . Integrating with Uzum Market’s seller platform to instantly pay out sales to the wallet is one idea. This taps into B2B use cases and would likely require additional compliance (business KYC) and feature complexity (medium-term priority). Third-Party Integrations: In the longer term, integrate other financial services like insurance and investments through the wallet interface . For example, offer an insurance marketplace (users can buy travel or phone insurance via the app) or simple investment products (gold, bonds, etc.). This follows the “super-app” model seen in some fintechs (Revolut, N26) and can provide new revenue streams. These are low priority until core payments are solid, but on the strategic roadmap.
5. Ops & Support Improvements:
Admin Tools & Dashboard: Build a web dashboard for operations and support teams . This would allow non-engineers to view statistics, search transactions with more advanced filters (amount ranges, text queries on memos), and export data (e.g. CSV of transactions) for reconciliation. While MVP relies on direct API use, a friendly UI for ops will be important as volume grows. Monitoring & Alerts: Implement internal monitoring dashboards and alerts for abnormal conditions . For example, alert if there’s a spike in failed transactions, or if a service outage occurs. Some of this can be handled by existing APM tools (using the metrics we already collect), but building custom alerts (or integrating with fraud systems) ensures issues are caught quickly. In
---

future, also introduce automated anomaly detection on transaction patterns to catch potential fraud or system errors early. Dual Approval for Sensitive Actions: As noted, introduce a “four-eyes” approval process for high- risk admin operations. In a future version, certain actions (e.g. very large fund/reversal operations) would require a second admin’s approval before execution, to reduce insider risk. MVP relies on audit tracking, but adding this control is on the roadmap given its importance for a mature financial platform. Product Decision Record (Q&A)

Q: How should users specify a transfer recipient – only by account number, or can we use phone numbers/user IDs? A: We decided to support multiple addressing methods in MVP v3.2. Users can send money by providing the recipient’s phone number or internal user_id, not just an account ID . The backend will look up the phone or user ID to find the target account (using the default currency account if multiple). This improvement leverages the expectation in our markets that sending to a phone number is standard, while keeping it all internal (no external directory needed). It greatly improves UX for P2P transfers.

Q: What source of FX rates will the wallet use for currency conversion in MVP? A: For MVP, we chose to use a static, internal fx_rates table in the database as the source of truth . All currency conversion quotes read a pre-configured rate (with a possible markup) from this table. This approach keeps the system simple and deterministic for launch – rates can be updated by ops as needed, but aren’t fetched from an external API in real-time. Each FX Quote record stores the rate (and fee) used at quote time, so the conversion is transparent and locked in for that transaction . In the future, we may integrate live FX feeds or public APIs for dynamic rates, but that adds complexity (scheduling updates, handling API downtime, ensuring regulatory compliance on rate sources) that we deferred for MVP.

Q: How are we handling pagination and search in transaction history for MVP? A: Regular users will page through their own transaction history using cursor-based pagination (fetching, say, 10–20 transactions at a time). For admin/staff use, we provide a basic search API with filters (as described in the Admin API). It returns a bounded set (up to ~100 results) for a given query without server- side pagination, so admins can refine queries if needed. We opted not to build advanced text search or endless pagination in MVP . The rationale is that support queries will be targeted – e.g. looking up a specific transaction or user – and we can keep the implementation simple. If more sophisticated search or bulk export is needed in practice, we will iterate on the admin tools post-MVP.

Q: What is the retention period for idempotency keys, and why? A: We set idempotency record retention to 48 hours in MVP . In practice, this means the system “remembers” a POST request (by its Idempotency-Key) for two days. We picked 48h as it’s a common industry practice – it comfortably covers typical client retry scenarios (including users retrying the next day if they had network issues) . It’s also short enough that we won’t accumulate unbounded data; stale records get purged after 2 days. Some systems use 24h, but we chose a slightly longer window to be safe (accounting for time zone differences or weekend delays). The trade-off is minimal since storing a couple days of idempotency entries isn’t heavy, and it significantly reduces the risk of duplicate processing. We documented this decision and will ensure a cleanup job or TTL mechanism removes expired entries
---

Q: Do our transaction receipts contain all the information users (and ops) need? A: Yes – we made sure to expand the receipt schema for completeness. Each transaction receipt clearly shows which account was debited and which was credited (with user-friendly identifiers like names, phone or email for the other party, when available) . It lists the exact amounts in each currency, any fees applied, and exchange rate info if relevant. We also include a human-readable description or memo (e.g. a note the user entered, or a merchant name for a spend) so the context is clear . Importantly, we hide internal implementation details that aren’t meaningful to users – for example, raw account IDs are not shown (perhaps just the last few digits for reference), focusing instead on labels the user recognizes This decision closed a gap in earlier specs and ensures front-end and back-end are aligned on what’s displayed. It makes the receipt the single source of truth for confirming a transaction, and it’s formatted such that it can be shared or exported (e.g. as a PDF) if needed

Q: Why are we not including KYC or external bank/card linkages in MVP? A: Given the aggressive one-month timeline and the focus on building core wallet functionality first, we deliberately kept KYC onboarding and external payment rails out of scope for MVP . These features (connecting to bank cards, allowing cash-out to banks, verifying customer identities) involve significant additional complexity – both technically and in compliance. Instead, for MVP we constrained usage to internal transfers among already-onboarded Uzum users and simulated cash-in/out via admin tools. This allowed us to deliver a usable product faster. Before we enable real external money movement, we will integrate Uzum Bank’s KYC processes and likely partner with card networks or payment systems, which requires more development and regulatory approvals. Those are high-priority next steps after MVP, but including them from the start would have jeopardized the MVP timeline.

Q: Will admin operations have a 4-eyes (dual approval) control in MVP? A: Not in MVP. At launch, any single authorized admin can perform a sensitive operation (like funding, withdrawing, or reversing funds) directly, and it will execute immediately. We chose to rely on strict audit logging (recording who did what) as the control for MVP . The reasoning is that implementing a maker- checker approval workflow would add a lot of complexity – in UI design, state management, and policy – and given the low volume and trusted team at startup, the audit trail was deemed sufficient for now. In the future, as transaction volume and the ops team grow, we plan to introduce dual-approval for certain high- risk actions (for example, very large transfers or reversals) . This is already noted on our roadmap. It’s a classic trade-off: speed of development vs. strict controls, and for MVP we optimized for speed while planning to tighten controls soon after. Persistence Strategy The wallet service uses a single relational database for persistence. PostgreSQL is required for production deployments due to its robustness in handling transactional updates and ensuring data integrity During development and automated testing, a lightweight SQLite database may be used for convenience, but with the understanding that we will migrate to PostgreSQL in staging/production. All schema changes are managed via migrations to keep dev and prod in sync. Regular backups and point-in-time recovery (PITR) will be configured on the PostgreSQL instance to protect against data loss. Given the financial nature of the data, the database is treated as the source of truth – no ephemeral in-memory balances are kept outside of it, simplifying consistency. We also designed the schema for future scalability (partitioning or sharding can be introduced later if volumes grow, but MVP will run on a
---

single primary database node). Long-term, data retention policies (e.g. archiving old audit logs) will be implemented as needed to comply with regulations and manage storage. Database Schema (MVP Core) The core PostgreSQL schema for MVP includes the following tables (with key fields): tenants – (Future use) Tenant organizations; MVP is single-tenant so this has one entry. Fields: tenant_id (PK), name, status. users – Wallet users. Fields: user_id (PK), tenant_id, phone/email, status (ACTIVE/BLOCKED), etc. accounts – Currency accounts tied to users. Fields: account_id (PK), user_id (owner), currency, status (ACTIVE/CLOSED), created_at . (Each user has one account per currency in MVP.) journal_tx – Ledger transactions (journal headers). Fields: tx_id (PK), type (TRANSFER, FX, admin_op, etc.), status, reference_ids (like correlation_id), timestamps. journal_entry – Ledger entries (double-entry lines). Fields: entry_id (PK), tx_id (FK to journal_tx), account_id (FK to accounts), amount, side (debit/credit), and an entry_type or description fx_quote – FX conversion quotes. Fields: quote_id (PK), from_account_id, to_account_id, amount_from, rate, fee, expires_at, status fx_rates – Exchange rate reference data (static). Fields: currency_pair, rate, markup, last_updated (MVP uses this for all FX conversions.) idempotency_record – Idempotency key log. Fields: key (PK or unique), actor_id, endpoint, request_hash, response_code, response_body, created_at, expires_at (TTL) audit_event – Security audit log. Fields: event_id (PK), actor_id, role, action_type, target_ids (e.g. account_id or tx_id involved), timestamp, metadata (IP, device) (Other support tables like an outbox or reference data tables may exist, but the above are the primary ones for MVP.) Mobile App UX (MVP) The Uzum Wallet will surface in the Uzum mobile app via new screens built to the Uzum UI design kit. The key user flows and interfaces include: Home Dashboard: Shows the user’s wallet balances for each currency (e.g. UZS and USD) with quick action buttons . Users can switch currency views and initiate actions like “Transfer” or “Convert” from this screen. Notifications or status banners (e.g. maintenance alerts) may also appear here. Transfer Money: A flow to send money to another user. The user selects the source account (if multiple currencies) and specifies a recipient. In MVP this can be done by choosing from contacts or entering a phone number/user ID (which the app will resolve to an internal account) or by directly inputting an account ID . The UI will show the recipient’s name if known. The user enters an amount and an optional note. A review screen then confirms details (recipient, amount, any fee = 0 for internal) before submission. After a successful transfer, a confirmation with the receipt is shown, and the user can share or repeat the transfer Currency Conversion: A two-step FX flow. The user selects “Convert Currency”, chooses the source and target currency (only UZS⇄USD in MVP), and enters an amount to convert. The app calls POST /v1/fx/quote and then displays a quote screen showing the exchange rate, fee (if any),
---

and resulting amount . The user can confirm to execute, upon which the final result (updated balances and a receipt showing the conversion details) is displayed Transaction History: Users can view a list of past transactions (for each account). This is accessible via a “History” tab. Transactions are listed with basic info (date, amount, type, counterparty or description) and can be tapped to view the full receipt . The history list uses lazy-loading (cursor pagination) to fetch more records as the user scrolls. Filters (by type, date) are not in MVP but planned later. Receipts & Details: For any transaction (transfer, conversion, etc.), the user can view a detailed receipt. This shows all relevant info: status, timestamp, from/to accounts (with names/contacts), amounts, fees, exchange rate if applicable, and a unique transaction ID. The UI provides an option to export or share this receipt (e.g. generate a PDF or image) for the user’s records Admin Interface (MVP): There is no dedicated GUI for admins in MVP; admins use internal tools or direct API calls (e.g. via Swagger or scripts) to perform admin operations. Admin-focused UI will be developed post-MVP as noted. Design & QA: The mobile UI follows Uzum’s design system for consistency. Standard components (buttons, inputs, dialogs) are used. Biometric authentication or OTP confirmations are handled by the existing app’s security (wallet actions assume the user is already logged in to the app). The wallet features have been tested end-to-end in the app to ensure a smooth user experience (e.g. proper loading states, error messages on failures like insufficient funds, etc.). and the v3.2 upgrade document , ensuring that v3.2 enhancements (recipient resolution, static FX rates, idempotency TTL, receipt details, Rust implementation, extended roadmap) were integrated into the comprehensive specification. The comparative analysis of leading wallets informed the post-MVP roadmap. This v3.2 blueprint is a single source of truth for the product and engineering teams moving forward, superseding all prior versions. uzum_wallet_mvp_blueprint_v3_1_landscape.pdf file://file_00000000ad6071f49d1303084be7d5fa Uzum Wallet MVP Blueprint (v3.2 Upgrade) – Contract-First Monolith (Uzbekistan).pdf file://file_00000000b0dc71f485b5d74f79762115 Comparative Analysis of Leading Wallet Apps and Uzum Wallet Backlog.pdf file://file_00000000b59471f4bc330aa907762545

## Input C
_Note: Compiled from earlier notes (legacy)._


Contract-First Monolith (Uzbekistan) Executive Summary Uzum Wallet MVP v3.2 builds upon the v3.1 foundation by clarifying under-specified elements and incorporating key product decisions to ensure a production-grade implementation. The core scope remains a multi-currency wallet ledger with internal P2P transfers, FX conversion (quote & execute), admin ledger ops, transaction history, and sandbox spend simulation. No changes to the monolithic, contract-first architecture or MVP boundaries – external payment rails, KYC, merchant integrations, and credit products remain out-of-scope for MVP . The v3.2 update focuses on removing ambiguity in APIs and data model (e.g. how recipients are addressed, how FX rates are sourced, idempotency retention), and adds a “Product Decision Record” section capturing Q&A on these design choices. We also expand the post- MVP feature roadmap based on a comparative analysis of leading wallets, highlighting future enhancements (QR payments, loyalty, installments, etc.) to guide strategic planning. These refinements aim to make the MVP spec implementation-clear and aligned with fintech-grade standards, without altering the core MVP architecture or delivery timeline. All financial invariants (atomic double-entry postings, immutable journal, strict idempotency) and production requirements (PostgreSQL, audit logging, basic fraud controls) from v3.1 are upheld in v3.2. Changelog (v3.1 -> v3.2) P2P Recipient Addressing: Extended internal transfer capability to allow specifying the recipient by phone_number or user_id (in addition to account_id ). The service will resolve such identifiers to the target account_id at transaction time, using the user directory. Note: to prevent deadlocks, the transfer logic will always lock accounts in a consistent order (e.g. by user_id). Admin Search API Filters: Defined the admin search endpoint ( GET /v1/admin/search ) to accept minimal query filters – user_id, account_id, tx_id, currency, time_range – returning up to 100 matching transactions or accounts. Clarified that pagination can be handled on the frontend (no server-side cursor for MVP). FX Rate Source: Locked FX conversion in MVP to use a static fx_rates table in the database as the source of currency exchange rates. The POST /v1/fx/quote operation will read from this table (with pre-configured rates/markup) . Future integrations with public FX APIs or dynamic feeds are noted as post-MVP enhancements, but for MVP all quotes use the internal table. Each FX quote record captures the rate (and fee) at quote time as an immutable snapshot for transparency. Idempotency Key Retention: Established a standard 48-hour TTL for stored idempotency keys. Idempotency records (used to prevent double-processing of POST requests) will be persisted for 48 hours, after which they may be purged/expired. This ensures clients can safely retry for a reasonable window without persistent storage growth.
---

Transaction Receipt Schema: Enhanced the transaction receipt model to include a complete breakdown of the transaction details. Receipts (returned by GET /v1/transactions/{tx_id} ) will list all involved accounts with amounts and debit/credit direction, the transaction type and timestamp, and any FX rate and fee applied (for conversion transactions). An optional memo or merchant_label is included for user-friendly context (e.g. user-entered note or merchant name on a spend). The API will exclude internal-only fields, showing only user-meaningful data (e.g. no raw internal IDs besides the transaction ID reference). Product Decision Record (PDR): Added a Q&A style section summarizing key product decisions resolved in v3.2 (e.g. choice of 48h idempotency retention, using a static FX rate table, handling of history pagination) along with brief explanations. Extended Post-MVP Roadmap: Appended a feature backlog derived from comparative market analysis, grouping and prioritizing future enhancements in categories (Payments/Rails, Compliance, UX, Wallet Capabilities, Ops & Support). High-priority items include phone-number P2P and QR payments, loyalty rewards integration, installment (BNPL) features, joint/family accounts, virtual/physical cards, merchant/SME tools, and a potential insurance/investments marketplace, among others. These are not in MVP scope but inform the next phases. Product Scope & Personas End-User Persona: A consumer within the Uzum ecosystem who needs to hold money in local and foreign currency balances, send and receive money easily with other users, and perform transparent currency exchange – all with immediate, immutable receipts for each transaction. In MVP, users can transfer funds internally to others by selecting a contact (now by phone number or user ID as well) or entering account details, and can convert currency within their wallet . The experience emphasizes speed (instant transfers), no fees for internal P2P, and clarity of each transaction (balance updates and receipts). Admin/Operations Persona: Back-office staff or support agents who manage the ledger system. They can create or close user accounts, credit or debit user balances (mint or withdraw in ledger terms), and perform compensating reversals for errors. They also have search tools to lookup transactions or accounts by various identifiers for customer support and reconciliation. MVP assumes a single-admin authority model (no multi-approval) with full audit logging of all admin actions . Admins use the search API to filter transactions (by user, account, etc.) when investigating issues or fulfilling support requests. (MVP Scope Summary: The MVP provides wallet accounts per currency, an internal double-entry ledger for all transfers, and basic wallet operations: P2P transfer, FX conversion, transaction history with receipts, admin ledger controls (open/close accounts, mint, burn, reverse), and a sandbox “spend” simulation. It does not include external top-ups or withdrawals (no card/bank linkage), merchant payments, or KYC onboarding flows . These are deferred to future versions, as is any credit/loan functionality.) Domain Model & Data Invariants Core Entities: The v3.2 data model retains all v3.1 entities with one addition for FX rates: User – End-user record (with unique user_id ), including personal identity info (phone, email, etc.), status flags, and linkage to one or more wallet accounts . Each user belongs to a tenant (Uzum ecosystem tenant, single-tenant for now) and can have one default account per currency.
---

Account – Ledger account representing a wallet balance in a specific currency. Key fields: account_id , user_id (owner), currency , status , created_at . Each user/currency pair has one account (MVP supports UZS and USD accounts per user). Accounts can be ACTIVE or CLOSED (closed accounts are suspended from use, but never physically deleted). JournalTx – A financial transaction header (journal entry) with tx_id (unique), type (e.g. P2P transfer, FX, admin adjustment), status, timestamp, and correlation fields. Every money movement in the system is a JournalTx, which links to one or more JournalEntry lines. Once posted, JournalTx records are immutable (no updates/deletes) as part of the audit trail invariant. JournalEntry – The double-entry line items associated with a JournalTx. Each entry records an account_id , amount, and side (debit or credit). For each JournalTx, total debits equal total credits in each currency, ensuring ledger balance . JournalEntries also carry a reference to the tx_id and perhaps a short description (e.g. “P2P transfer from X to Y”). FxQuote – An FX conversion quote record. Created when a user requests a currency conversion quote. Fields: quote_id , from_account_id , to_account_id , amount_from , rate , fee , expires_at , status . The quote holds a snapshot of the exchange rate (and any fee or markup) offered to the user, and an expiration timestamp (e.g. 60 seconds from quote creation) after which the quote can no longer be executed. In v3.2, quotes use the static FX rate table as source; the rate and fee stored in FxQuote represent the exact terms given to the user at that time. (No separate version ID for the rate is needed, since the quote records the actual rate used.) FxRate – (New in v3.2) A static reference table of currency exchange rates. This table holds the authoritative rates used by the wallet for FX quotes in MVP. Fields might include currency pair (e.g. UZS/USD ), current rate, an optional markup or fee percentage, and a last_updated timestamp. The MVP will preload or maintain this table with the applicable market rates (e.g. derived from Central Bank or internal treasury) and possibly a small markup. All FX quote requests will read from this table to determine the rate/fee offered . (In future, this could be replaced or updated via external API feeds – see Product Decision Record and Roadmap for details.) IdempotencyRecord – Record of a past POST request identified by an Idempotency-Key. Contains the key value, the requesting actor_id (user or admin), the endpoint and a hash of the request payload, plus the resultant status code and response body, and a timestamp . In v3.2, an additional field/attribute tracks the expiration of this record (TTL). The system retains idempotency records for 48 hours from creation, after which they may be purged. This prevents indefinite growth of the table while covering typical client retry windows. AuditEvent – Append-only log of significant actions and system events for security and compliance. Each event has an event_id , actor info (who performed the action and their role/permissions), action type (e.g. ADMIN_FUND, USER_TRANSFER), target identifiers (which user/account/tx were affected), a timestamp, and context like IP or device ID . All admin operations and all financial transactions trigger audit events. These are retained long-term (at least 2–5 years as per compliance needs). Data Invariants:
-  Ledger Immutability: Once a JournalTx and its JournalEntries are marked  POSTED/COMPLETED, they cannot be altered or deleted . Corrections are done via new transactions (e.g. a reversal) rather than editing history. This guarantees an immutable ledger for audit and reconciliation.
- Double-Entry Balance: For every JournalTx, the sum of debit entries equals sum of credit entries in each currency involved. This ensures no money is created or lost in transit. The system enforces this with a DB check constraint or in-code validation.
-  Consistent Account States: Account statuses and balances follow defined rules. For MVP,  blocked balance is always zero (holds are not used in MVP) , so an account’s available balance equals its total
---

ledger balance. If in future “holds” are introduced (for card authorizations, etc.), the schema will adjust, but MVP assumes no pending holds. Closing an account (status = CLOSED) prevents new debits/credits but does not remove the record or its balance history.
- Tenant and Group Structure: MVP is single-tenant (all users under the same umbrella). The schema has a placeholder for tenant_id on user/accounts for future multi-tenant support. Similarly, while MVP is single-owner per account, the schema reserves a wallet_group_id field to support shared/joint wallets in the future (e.g. family accounts). No group functionality is active in MVP beyond this placeholder.
Balance Model & Transaction Lifecycle Each account has three key balance metrics: available, blocked, and ledger. In MVP, blocked = 0 for all accounts (no hold functionality), so effectively available = ledger balance at all times . All transactions affect balances atomically via the journal posting: On a P2P transfer, the sender’s account ledger (and available) balance decreases by the transfer amount, and the recipient’s account increases by the same amount. These debits and credits post in one transaction so the ledger remains consistent. On an FX conversion, two accounts of the same user (e.g. UZS and USD) are involved: the source currency account is debited, and the target currency account is credited with the converted amount (after applying rate and fee). Any FX fee is taken as a separate JournalEntry (crediting a fee revenue account). The net debits (source + fee) equal the credit in target currency after conversion (adjusted for rate) Admin mint (fund) adds balance to a user’s account by debiting a special SYSTEM_MINT account and crediting the user (this represents creating money in the system, backed by an external funding outside MVP scope). Admin withdraw (burn) does the opposite: debits the user’s account and credits a SYSTEM_SINK or treasury account (effectively removing money from circulation in the wallet) . These mimic deposit/withdrawal without external rail integration. Reversal is a special admin operation to correct mistakes: it creates a new JournalTx that effectively negates a target transaction’s effects. For example, if tx_id XYZ credited 100 UZS to A and debited 100 UZS from B, a reversal would debit A and credit B for 100 UZS, with a reference link to XYZ. Reversals ensure even erroneous transactions remain in history (they are not deleted, just offset) Spend Simulation is a test transaction where a user “spends” to a dummy merchant account (SYSTEM_SPEND). It debits the user’s account and credits the SYSTEM_SPEND account, optionally attaching a merchant_label (e.g. “Simulated spend at Uzum Market”) for UI display. This is purely for QA/testing and user demonstration; it has no external effect but appears in history like a normal debit All these workflows follow the ledger invariants above and produce a transaction receipt accessible to the user. The receipt includes the final balances (post-transaction) in each affected account, confirming the updated available amounts. It also captures the essential details of the transaction (e.g. “You sent 50,000 UZS to +99890... (John) on Jan 10, 2026”, including the tx_id for reference).
---

API Specification & Contracts (v1) The MVP API is versioned under /v1 and uses a RESTful contract-first design. All client-facing endpoints and request/response schemas are predefined (OpenAPI) and kept in sync with implementation. Authentication is via JWT (Uzum SSO tokens), with role-based authorization ( USER vs ADMIN ) enforced on protected endpoints . Idempotency-Key header is required on every POST that modifies financial state (transfers, FX execution, admin fund/withdraw, etc.) . Standard HTTP error codes and a stable JSON error envelope are used (e.g. 400 for validation errors, 409 for idempotency conflicts, 401/403 for auth issues, etc.) Admin API Endpoints: (require ADMIN role)
- POST /v1/admin/accounts  – Create a new account for a user in a given currency. Allows ops to set up additional currency accounts for a user. Idempotent: Recommended (client may retry on failure).
- POST /v1/admin/accounts/{id}/close  – Close (suspend) an account. Marks an account as closed (no new transactions allowed). No hard deletes. Idempotent: Yes (re-closing an already closed account is a no-op).
- POST /v1/admin/fund  – Mint funds into a user’s account. The request specifies target account_id , amount, and reference. Internally posts a JournalTx debiting SYSTEM_MINT and crediting the user.
Idempotent: Yes (required, to avoid duplicate mints).
- POST /v1/admin/withdraw  – Burn funds (withdraw) from a user’s account. Debits the user and credits SYSTEM_SINK. Essentially the inverse of fund. Idempotent: Yes (required).
-  POST /v1/admin/reverse  –  Reverse a transaction. Takes a  tx_id  of an existing JournalTx and creates a compensating transaction (credits and debits swapped) to negate it. Used for remediation of mistakes. Idempotent: Yes (required, ensure a given tx_id is reversed only once).
-  GET /v1/admin/search  –  Search ledger records by filters. Allows admins to query transactions or accounts based on user_id , account_id , tx_id , currency , and/or a time range. Returns up to 100 results matching the criteria (e.g. transactions in date range, or all accounts for a user). The response is not paginated on the server; if more results are needed, the admin can refine filters or time window (the assumption is administrative queries are targeted and infrequent). This endpoint helps support staff quickly locate a specific transaction or set of entries by key identifiers. (No idempotency needed for GET.) User Wallet API Endpoints:
- GET /v1/profile  – User Profile & Defaults. Returns user’s basic profile and default account IDs (e.g.
their primary UZS account and USD account) for convenience.
- GET /v1/accounts/{id}  – Account Details. Returns metadata of a specific account (currency, status, created date, etc.), verifying ownership.
- GET /v1/accounts/{id}/balance  – Account Balance. Returns the current balance of the account, broken down into available and blocked amounts (blocked will be 0 in MVP). Also may include the account’s transaction count or last transaction timestamp for reference.
-  GET  /v1/accounts/{id}/transactions?cursor={cursor}&limit={N}  –  Transaction  History.
Retrieves a paginated list of recent transactions for the given account. Supports cursor-based pagination (cursor points to a tx_id or timestamp; limit up to 50 or 100). Ordered by newest first (descending by date/tx_id). This allows users to scroll through their own history. (For MVP, basic filtering by type or date is not provided on this endpoint – only chronological pagination.)
-  GET  /v1/transactions/{tx_id}  –  Transaction  Receipt. Fetches  the  full  details  of  a  specific transaction (if the user has access to it). The receipt includes: the transaction’s status (e.g. COMPLETED),
---

type, timestamp, and a list of the entries (each with account info, amount, and direction debit/credit relative to that account) . It also includes human-readable annotations: for internal transfers, it will show the other party’s name/identifier; for FX, the exchange rate and fee applied are included ; for simulated spends, the merchant label is shown . A short optional memo field is returned if the user attached a note or if the transaction has a relevant label (e.g. “Uzum Market purchase”). The receipt is the source of truth for transaction confirmation and is shareable (in-app users can generate a PDF or image of it) . (No idempotency for GET.)
- POST /v1/transfers  – Peer-to-Peer Transfer. Moves funds from one user’s account to another user’s account. Request includes source account_id (debitor), a way to specify the recipient, and an amount .
In v3.2, the recipient can be identified in three ways: by specifying their account_id directly, by their user_id , or by their phone number. If user_id or phone is provided, the backend will resolve it to the target account_id (choosing the appropriate default account, e.g. in the same currency, or failing if none). This resolution occurs within the transaction processing – after resolution, the service locks both the source and target accounts (using a consistent order by user_id to avoid deadlock) and then validates balances. If the source account has sufficient available funds and both accounts are ACTIVE, a JournalTx is posted with two JournalEntries: one debiting the source and one crediting the destination. On success, a 201 response is returned with the new transaction’s receipt. Idempotent: Yes – duplicate transfers with same Idempotency- Key will return the same result or a conflict if payload differs.
-  POST  /v1/spend/simulate  –  Simulate  Spend. Deducts  an  amount  from  a  user’s  account  as  if spending at a merchant, crediting the SYSTEM_SPEND account. Request includes account_id , amount , and a merchant_label or description. Used for testing or demonstration (e.g. simulating a purchase).
The system processes it similar to a transfer (debit user, credit system account) and records the label in the transaction metadata. Idempotent: Yes – to prevent double charges in simulation.
- POST /v1/fx/quote  – Create FX Quote. Initiates a two-step currency conversion. The user provides a source account_id , target account_id (must belong to the same user and be a different currency), and an amount to convert from the source. The system looks up the applicable exchange rate from the fx_rates table for the currency pair and calculates the converted amount, applying any fee or markup. It creates an FxQuote record with a unique quote_id , embedding the rate, fee, source/target accounts, and an expiration time (e.g. 60 seconds from now) . The response returns the quote details: the offered rate, fee, the amount that would be credited to the target account, and the expires_at timestamp. The quote remains pending until executed or until it expires. Idempotent: Yes (clients can retry quote creation safely, though a new quote will be generated each time; a repeat with the same key could return the same quote or an equivalent one).
- POST /v1/fx/execute  – Execute FX Conversion. Commits a previously obtained FX quote. The request includes the quote_id . The server will retrieve the FxQuote, validate that it is still active and not expired, then perform the currency conversion: a JournalTx is posted that debits the source account (for the original quote amount + fee) and credits the target account (for the quoted converted amount), and credits the fee to the system fee account (if a fee was configured) . If the quote has expired or already been used, the execution is rejected with an error. On success, the response is a transaction receipt of the completed conversion. Idempotent: Yes (only one execution will succeed for a given quote; duplicate calls will return the same result or an error if already executed).
Note: All POST endpoints return standard JSON results on success (typically including the new resource ID or transaction details). Error handling follows a unified model – a JSON with an "error" object containing a machine-readable code and a human-readable message . For example, if a transfer is attempted with insufficient funds, the API returns a 422 or 409 error with code INSUFFICIENT_FUNDS
---

Idempotency conflict (same key used with different request data) returns HTTP 409 with an IDEMPOTENCY_CONFLICT error code Idempotency & Concurrency Controls Idempotency Design: Every client-initiated write operation (money movement) must supply an Idempotency-Key (a UUID) in the request header . The server stores a record of this key and the request details on first processing. If the exact same key is seen again: If the subsequent request is identical (same endpoint and payload hash), the server short-circuits and returns the originally stored response (with an Idempotency-Replayed header) . This allows safe client retries of, say, a transfer, without duplicating the transaction. If the same key is reused with a different payload, the server responds with HTTP 409 Conflict ( IDEMPOTENCY_CONFLICT error) and does not process the new request . This indicates a logical error on client side (key collision). Idempotency records are persisted in the idempotency table with a time-to-live of 48 hours. After 48h, the system may discard the record, meaning the same key could be reused (though clients are expected to use unique keys for new requests). The 48h retention strikes a balance between allowing cross-day retries and preventing unbounded growth of the idempotency log. Concurrency and Locking: The wallet operates under serializable transaction isolation or explicit row locking to prevent race conditions (e.g., two transfers hitting the same account concurrently) . For operations that involve multiple accounts (like a transfer between two users), the implementation must lock the rows (accounts) in a consistent order to avoid deadlocks. For example, always lock in ascending order by user_id (or account_id) when transferring between two accounts. This deterministic locking order ensures no cyclical waits occur . If one transaction is already processing involving Account A then B, another concurrent transaction involving B then A will wait for locks in the same sequence rather than deadlock. We use SELECT ... FOR UPDATE on account balance rows during posting to serialize balance updates. Additionally, to maintain consistency:
- No double-spend: A transfer will check the latest available balance after acquiring the lock; if insufficient, it aborts with INSUFFICIENT_FUNDS (no partial posting).
- Retry logic: In the rare case of a serialization failure (e.g., two FX executes trying to consume the same quote, or other write skew), the operation will fail and can be retried by client (with same idempotency key, yielding safe replay). Such scenarios should be minimal, but the system will log and expose metrics like idempotency conflicts or serialization retries for monitoring Security & Compliance Considerations Authentication & Authorization: As noted, all API calls require a valid JWT. User tokens grant access only to their own data (e.g. cannot fetch another user’s account), enforced by checking the user_id in token vs resource. Admin actions require an admin JWT with elevated scope; all such actions are additionally logged in the AuditEvent table with who performed them . Session management (login, logout) relies on Uzum’s existing SSO/identity service. MVP defers advanced device binding or 2FA enforcement, though basic device info is captured in audit logs.
---

Data Protection: Communication is over HTTPS (TLS). Sensitive data at rest (if any, e.g. user personal info or secrets) is encrypted at the database/storage level (leveraging PostgreSQL TDE or cloud encryption, with keys in a secure KMS). The amount of PII in the wallet system is minimal (mostly phone/email, since detailed KYC info is not stored in MVP). Still, we ensure compliance with Uzbekistan’s personal data laws – e.g., storing data in approved regions and protecting user consent metadata Fraud & Abuse Controls: MVP introduces basic rate limiting: for critical endpoints like transfers, FX, login – to prevent brute-force or abuse. For instance, a user cannot initiate more than X transfers per minute, and an IP address cannot hit the API excessively . These limits mitigate spam or DDoS vectors. There is no automated AML rule engine in MVP, but the audit logs and admin search provide manual oversight capability. Compliance staff can review transaction histories for suspicious patterns. The design keeps hooks for integrating an AML monitoring tool post-MVP (for example, flagging large transfers or rapid in- out movements once KYC is in place). Regulatory Scope: As MVP does not involve external money movement, regulatory exposure is limited. The wallet operates under Uzum’s licensed banking entity by assumption , and all value remains internal. Nevertheless, we log all transactions for potential future regulatory reporting. If/when external payments (e.g. cards, bank connections) and KYC are introduced, full compliance with payment services regulations and identity verification laws will be required. We have reserved data fields and extension points for these (e.g., the ability to store KYC status per user, to enforce limits based on verification tier, etc., in later versions). Audit & Logging: Every important action generates an AuditEvent as described. These logs include sufficient detail to trace who did what and when (critical for forensic analysis). For example, if an admin performs a fund or reverse, the event will capture the admin’s user_id, the target account or tx_id, and a correlation_id linking it to the API request ID for cross-reference . Audit events are stored indefinitely for now (with at least 5-year retention recommended) . The system should also maintain regular database backups and point-in-time recovery (PITR) capability to prevent data loss. Product Decision Record (PDR – v3.2 Q&A) This section records key product questions that were raised in the design of the wallet and the decisions made in v3.2 to address them, along with brief reasoning:

Q: How should users specify a transfer recipient – only by account number, or can we use phone numbers/user IDs for convenience? A: We will support multiple addressing methods in MVP v3.2. Users can send money by providing the recipient’s phone number or internal user_id, not just an account ID. The system will look up the appropriate account behind the scenes (e.g. the recipient’s active account in the same currency). This decision improves UX (it leverages the fact that phone-number-based transfers are a fundamental expectation in our markets ) while keeping it internal – no external phone directory needed beyond our user database. It also sets the stage for later integration with contacts or national payment systems. The implementation will carefully handle resolution and locking (always resolving first, and locking accounts in consistent order by user_id to avoid deadlocks).

Q: What source of FX rates will the wallet use for currency conversion in MVP? A: For MVP, we chose to use a static, configurable FX rate table stored in our database . This
---

means all conversion quotes come from a controlled set of rates (which can be updated by ops as needed, e.g., daily or when significant). The rationale is to minimize external dependencies and unpredictability during MVP; using a fixed table with maybe a small markup ensures we have full control and auditability of the rates given to users. The FxQuote mechanism captures the rate used at the time of quote, so even if rates change later, each quote/transaction is self-contained. In the future, we plan to integrate with public FX APIs or a live feed for real-time rates, but that introduces complexity (refresh intervals, error handling, compliance with central bank rates) that is deferred. MVP’s static table approach is simple, deterministic, and sufficient for initial internal usage. (Note: transparency is maintained by showing the rate and fee on the conversion receipt.)

Q: How are we handling pagination and search in transaction history for MVP? A: Regular users will fetch their transaction history via cursor-based pagination (page through transactions 10–100 at a time). For admin/staff searching, we provide basic filter-based querying ( / v1/admin/search ) that returns a bounded result set (max ~100 records) for a given query. We decided not to implement advanced server-side pagination or full-text search in MVP. Instead, admins can narrow results by specifying a user or time range, etc., such that the results are manageable. The reasoning is that building a sophisticated pagination or search index was not critical for the initial release – operationally, support queries are expected to be targeted (specific user or transaction investigations). This keeps the implementation simple. If in practice more is needed, we will add cursor-based pagination to admin search in a future iteration. For now, the front-end can fetch and then allow the admin to refine or load more if needed.

Q: What is the retention period for idempotency keys and why? A: We set the idempotency record retention to 48 hours for MVP. This means the system will remember a request (by Idempotency-Key) for two days. We chose 48h as a standard in industry – it comfortably covers typical client retry scenarios (including overnight or poor network conditions where a user might retry the next day), but it’s short enough to not worry about unbounded data growth. Some systems use 24h; we opted for 48h just to be safe with time zone differences and potential weekend gaps. There’s minimal downside since the storage overhead for 2 days of keys is manageable, and stale records will be purged. This decision was made to balance reliability and cleanup – we documented it to ensure the ops team sets up a job or uses a TTL column to remove expired idempotency entries.

Q: Do our transaction receipts contain all the information users (and ops) need? A: Yes – we reviewed and expanded the receipt schema to ensure completeness. Each receipt clearly shows which account was debited and which was credited (with identifiers like phone/email or names when available for counterparties), the exact amount in each currency, and any fees or exchange rates applied . We include a human-readable description (memo or merchant name) if provided, so the user knows the context. We deliberately hide internal fields like account_ids that are not meaningful to the user (except perhaps the last few digits for reference), focusing on useful info such as “From: Your UZS Wallet, To: Alice (+99890... phone), Amount: 50,000 UZS”. This decision came from identifying a gap – earlier specs didn’t spell out receipt content, which could lead to inconsistent implementation. Now, by defining it, we ensure the front-end and back-end agree on what’s shown, and internal reconciliation is still possible (since ops can cross-reference tx_id).

Q: Why are we not including KYC or external bank/card linkages in MVP? A: Given the aggressive one-month MVP timeline, we kept KYC and external payment rails out of
---

scope . This was a conscious decision: implementing eKYC or integrating with card networks/ banks would introduce significant regulatory and technical overhead (identity verification flows, compliance checks, external API integrations, etc.) which are not feasible in the MVP timeframe. Instead, MVP operates in a closed environment (Uzum ecosystem users only, money stays inside). We assume users are either pre-verified by Uzum or this is a pilot with limited users. We will address KYC in the post-MVP phase – likely leveraging Uzum Bank’s existing processes – before any broader rollout or any ability to cash in/out. External top-ups/withdrawals (cards, bank account transfers) likewise require partnerships and testing (e.g. integration with Uzcard/Humo or Visa). These are high priority on the roadmap, but for MVP the decision was to simulate them via admin operations (mint/ burn) and focus on perfecting the internal ledger and UX first.

Q: Will admin operations have a 4-eyes (dual approval) control in MVP? A: Not in MVP – admin actions (like fund, withdraw, reverse) will execute immediately with a single admin’s authorization, but everything is logged with who did it. We chose to delay multi-approval workflows until post-MVP . The reasoning: implementing an approval queue or maker-checker system adds complexity in UI, state management, and policy – and for an MVP with likely low volume and trusted ops users, the audit trail was deemed sufficient control. In the future (when transaction volume and risk grow), we plan to introduce a dual-approval for certain sensitive actions (e.g. large fund/reversal operations) as a defense against mistakes or internal fraud. This is noted in the backlog. (These Q&A items serve to document why certain features are designed the way they are in MVP v3.2, providing context for future readers or decision-makers.) Post-MVP Feature Roadmap Beyond the MVP, we have identified a set of features to expand the Uzum Wallet’s capabilities, drawn from a comparative analysis of leading wallet apps in relevant markets . These are grouped into categories and prioritized (High = near-term must-haves, Medium = strategic differentiators, Low = long-term nice-to- have). They were chosen based on competitor offerings and Uzum’s ecosystem strategy, and will guide the next phases of development:
1. Payments & Rails Enhancements:
- Phone Number & Card Number Transfers: Enable seamless P2P payments to users outside the immediate wallet system via phone number or even card number links. This includes integration with local instant payment networks (e.g. Uzcard/Humo in Uzbekistan) to allow sending money to any card or phone-linked account, not just within Uzum . This would extend the current internal-only transfers to a more open network, greatly increasing utility and network effects.
-  QR Code Merchant Payments: Introduce the ability to pay merchants via QR codes. Users can scan a merchant’s QR from the app to transfer payment. This requires building a merchant-present QR scheme or supporting a national standard QR if one exists . High priority, as QR payments are popular in CIS super- apps and would allow Uzum Wallet to be used for in-store purchases, tying into Uzum Bank’s acquiring business (Uzum Pay).
- Bills, Utilities & Mobile Top-ups: Integrate common bill payment services (utility bills, phone airtime top-ups, taxes, etc.) directly into the wallet . Most regional wallets (Kaspi, Qiwi, etc.) offer extensive bill pay options. Uzum Bank already has some of these on web; bringing them into the wallet app would drive frequent engagement. This likely involves connecting to biller aggregators or APIs for each service (post-
---

MVP due to integration work).
- Virtual and Physical Cards Integration: Expand wallet by offering virtual debit cards instantly, and optional physical cards linked to the wallet balance . Uzum has started issuing Visa cards; in-wallet card management (view card details, freeze/unfreeze card, set PIN) would be a key feature. This effectively turns the wallet into a full payment account usable at any POS or online (via NFC payment or card number). It’s high priority to reach parity with competitors like Revolut, which gained traction through easy card issuance . Physical card support also ties in with Uzum’s omnichannel strategy (cards distributed at pickup points, etc.).
2. Compliance & Security:
- KYC and Identity Verification: Implement a tiered KYC process for wallet users (especially if external money flows are enabled). This could involve integrating with digital ID verification services, scanning passports, etc., to comply with regulations before lifting certain limits. MVP deferred KYC , but it will become mandatory when expanding beyond closed-loop usage. Along with KYC, enforce any necessary user risk scoring and sanction screening in partnership with compliance.
-  AML  Transaction  Monitoring: As  volume  grows,  introduce  automated  monitoring  for  suspicious transactions. E.g., rules to flag rapid in/out transfers, large amounts, or blacklisted recipients. This may tie into an AML software or custom rule engine. Alerts from this system would feed to compliance officers for review. This is crucial once external payments and higher volumes come into play (medium to high priority post-MVP).
-  Advanced Login Security: Add features like  biometric authentication (fingerprint/FaceID login support) and robust device binding. While MVP uses basic JWT auth, adding biometrics improves security and user convenience. Also consider adaptive authentication (prompt for re-auth for high-risk actions) and session management (remote logout, device list) as seen in top fintech apps
- Fraud Prevention Tools: Beyond rate-limits, implement features such as transaction OTP confirmations for large payments, AI-based fraud detection (monitoring user behavior), and possibly integration with national anti-fraud systems. Competitors often highlight their real-time fraud alerts ; Uzum should plan similar capabilities to build user trust as the platform scales.
3. User Experience (UX) Enhancements:
- Saved Recipients & Contact Integration: Allow users to save frequent recipients or import contacts, so that sending money is as easy as selecting a name from an address book. MVP requires manual entry (or phone lookup within our system), but future versions will let users mark certain recipients as favorites with nicknames, and possibly sync contacts to find other Uzum Wallet users. This speeds up P2P transfers and leverages social connections.
- Enhanced Transaction History & Insights: Provide richer filtering and search in the user’s transaction list (by date range, by type – e.g. show only FX or only incoming transfers). Also, personal finance features like spending categorization, monthly summaries, or budgeting tools could be added to increase engagement.
While MVP keeps history basic, these UX improvements can differentiate the app in later phases.
- Multi-language Support & Localization: As Uzum grows, ensure the app and receipts can be displayed in multiple languages (Uzbek, Russian, English, etc.). This may involve i18n of currency formats and possibly supporting different currency symbols in UI beyond UZS and USD.
- UI Personalization & Other UX: Features like dark mode, customizable app themes, or quick action shortcuts for common tasks can be considered. These are lower priority but contribute to a polished user experience expected from modern apps.
---

4. Wallet Capabilities Expansion:
- Unified Loyalty & Rewards: Integrate loyalty points or cashback rewards into the wallet . Since Uzum has e-commerce and delivery arms, a unified loyalty program where wallet usage (or marketplace purchases via wallet) yields points or cashback will incentivize usage. For example, users might get a percentage back into their wallet for each Uzum Market purchase paid with the wallet. This requires tracking rewards and possibly a points currency, but it’s a high-impact feature to increase stickiness.
- Installments & Pay-Later (BNPL): Embed Uzum’s Nasiya installment plan product into the wallet flows This means enabling at-checkout financing: e.g., when a user is about to pay (especially for larger amounts, or on marketplace purchases), offer “Pay in 4 installments” or micro-loan options. Many competitors (Kaspi, Wildberries) have had huge success integrating such BNPL offerings directly . This would require real- time credit decisioning and an update to the ledger model (since an installment purchase might create a schedule of future payments). It’s a major feature bridging wallet and lending product, targeted soon after MVP (high priority, leveraging Uzum Bank’s existing consumer loan capabilities).
- Shared/Family Wallets: Introduce joint accounts or family wallet features . This could allow two or more users to share an account or have linked accounts (e.g., a parent-child arrangement with controlled access).
While uncommon in CIS wallets currently, it’s a differentiator and addresses use cases like couples managing shared finances or parents giving pocket money on a kid’s card . Implementing this will require a notion of wallet groups and permissions (the groundwork of which we’ve reserved in the schema ). Medium priority – not immediate, but potentially Q2/Q3 roadmap if aiming to stand out regionally.
- Expanded Multi-Currency & Forex Tools: MVP supports UZS and USD. Going forward, we might add more currencies (EUR, RUB, etc.) if market demand exists . Additionally, features like holding multi-currency balances with real-time FX updates, or setting target conversion rates (like a FX order/alert) could attract users who have forex needs. This positions the wallet closer to a Revolut-like offering for travelers or savers.
-  Merchant & SME Wallet Features: Expand the platform to  business users. For example, provide an SME version of the wallet for small merchants or marketplace sellers . Features might include quick business account onboarding, the ability to receive payments (possibly via the QR payments above), pay suppliers or employees, and view sales analytics. Integrating with Uzum Market for instant payouts to sellers’ wallets is one idea. This essentially merges wallet with basic business banking tools, capturing a new user segment and increasing transaction volume in the ecosystem . This is a natural extension once consumer wallet features are solid, though it introduces regulatory considerations (business KYC) and complexity (perhaps medium to long-term priority).
- Third-Party Integrations (Insurance, Investments): In the longer term, the wallet can evolve into a financial super-app hub. This could include an insurance marketplace (letting users purchase insurance products, e.g. travel or device insurance, through the app) and simple investment products (such as buying gold, stocks, or bonds in fractional amounts) . Global fintechs like N26 and Revolut have taken this route to increase engagement and revenue. For Uzum, this would likely come after core payments and credit features are in place (low priority for now). It would leverage partnerships or existing Uzum Bank offerings to integrate seamlessly.
5. Ops & Support Improvements:
- Admin Tools & Dashboards: Develop a richer admin interface for operations. This includes web dashboards to view aggregate stats, advanced search with filters beyond the basic API (e.g. by amount range or text search in memos), and the ability to export data (e.g. CSV of transactions for reconciliation). While MVP provides minimal API support, a friendly UI for ops can greatly improve efficiency in handling customer issues.
-  Transaction Monitoring & Alerts: Implement internal tooling for ops to set alerts on abnormal system conditions (e.g., a sudden spike in reversals or a service outage impacting ledger). Some of this overlaps
---

with observability, but specifically having alerts and an incident management playbook will be important as the system scales.
- Four-Eyes Approval Workflow: As noted, introduce a maker-checker system for admin operations post-MVP . This means certain actions (especially fund/withdraw above a threshold, or irreversible actions) require a second admin to approve before execution. The groundwork involves creating a pending state and an approval UI. This will reduce the risk of mistakes or insider fraud as operations grow.
- Customer Support Integration: Build features to support customer service workflows, such as the ability for support agents to impersonate or view a user’s wallet (in read-only mode) to help troubleshoot, or to resend receipts to users, etc. Additionally, a mechanism to handle disputes or chargeback-equivalents (when external rails introduced) will be needed – likely as tools for ops to mark transactions as contested, etc. Planning for these in the design will ensure the system can evolve to handle real-world issues gracefully.
Each of these roadmap items will be scoped and designed in detail in future blueprint versions. For now, they serve as a guiding backlog. High-priority features (like external P2P by phone, QR pay, cards, loyalty, installments) will likely be tackled in the next 1-2 phases as they bring Uzum Wallet to competitive parity and leverage Uzum’s unique ecosystem strengths . Medium priority items (shared accounts, SME tools, advanced security) will follow as we solidify the core. Lower priority or longer-term plays (investments, insurance) remain on the horizon to eventually transform Uzum Wallet into a comprehensive financial platform. End of Blueprint v3.2 – This document is prepared in landscape PDF format for internal discussion. It refines the MVP specification for immediate development and provides a clear trajectory for subsequent iterations, ensuring that the team can implement v3.2 with minimal ambiguity and plan future features in alignment with business goals. The focus remains on delivering a secure, robust wallet MVP quickly, while laying the groundwork for a scalable fintech product. uzum_wallet_mvp_blueprint_v3_1_landscape.pdf file://file_00000000391871f4b28fa493d1bb4879 Comparative Analysis of Leading Wallet Apps and Uzum Wallet Backlog.pdf file://file_00000000b59471f4bc330aa907762545 postgresql - Avoiding deadlocks when locking multiple rows without using NOWAIT - Database Administrators Stack Exchange https://dba.stackexchange.com/questions/323040/avoiding-deadlocks-when-locking-multiple-rows-without-using-nowait

## Input D
_Note: Compiled from backlog/analysis notes (legacy)._


Comparative Analysis of Leading Wallet Apps and Uzum Wallet Backlog Overview To inform the Uzum Wallet product strategy, we researched leading digital wallet applications in Russian- speaking and Eurasian (CIS) markets, along with notable European fintech “challenger” apps. Both B2C (consumer) and B2B (business) use cases were considered. Below we compare key features – including multi-currency support, peer-to-peer payments, ecosystem integrations (marketplaces, lending, insurance, loyalty programs), joint/family wallets, business accounts, cards, KYC, and fraud prevention – across major wallets. We then propose a feature comparison table, a prioritized feature backlog for Uzum Wallet (based on competitor overlap, feasibility, and differentiation), and a list of recommended competitor apps/APKs for deeper analysis. Feature Comparison of Leading Wallet Apps The table below summarizes the feature sets of selected wallet applications: Revolut, Monese, N26 (European challengers), and regional players Yandex Pay, Kaspi.kz, Ozon (Ozon Bank), and Wildberries (WB Bank). (Other notable CIS wallets like Qiwi and YooMoney are discussed afterward.)
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward Revolut (EU) Yes: Hold & exchange 25+ currencies, spend in 150+ countries Interbank FX rates (fee- free within limits). Yes: Instant free P2P to other Revolut users; 25+ currencies transfers (External transfers via SWIFT/SEPA with some fees) Yes: Free physical debit Visa/ Mastercard; multiple virtual cards (incl. single- use for security) ; Apple/ Google Pay support. Partial: No traditional joint account, but Revolut ＜18 sub- accounts for kids with their own card (managed by parent) . Family sharing of app subscription for multiple cards. Yes: Separate Revolut Business app/accounts for freelancers and companies (multi- currency accounts, team cards, merchant tools) Moderate: Revolut is a “financial super-app” with various services inside (travel booking, e- commerce cashback deals, etc.), but not tied to an external retail ecosystem Yes: Cashba 0.1–1% premium plans, referral bonuse periodic mercha offers Monese (EU) Limited: Multi- currency support for GBP, EUR (and RON) accounts; international transfers in 15+ currencies . No live FX trading, but can hold balances in multiple currencies. Yes: Free instant transfers between Monese users; external transfers via SEPA/SWIFT. (No built-in social feed, uses standard banking rails.) Yes: Debit Mastercard (contactless); virtual card via Apple Pay/Google Pay. (Instant digital card upon signup). Yes: Offers Joint Accounts (two users with separate cards sharing one account) . No dedicated child account feature. Yes: Monese has Business accounts for entrepreneurs (manage personal and business in one app) Moderate: Integrations with third- parties – e.g. PayPal link (view PayPal balance & transactions in Monese app) Otherwise focused on banking, not a broader e- commerce ecosystem. Limited Lacks a proprie points program Offers b budgeti “pots” (s goals) a occasio partner discoun
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward N26 (EU) Partial: Primarily EUR accounts (for Eurozone). No multi- currency balances (except separate USD accounts for US users). However, fee-free FX spending worldwide up to certain limits for premium users (Mastercard rate) Yes: MoneyBeam instant transfers to other N26 users; standard SEPA transfers to others. (Also supports phone/e- mail transfers to non-users with promised transfer). Yes: Mastercard debit (contactless); up to 2 virtual cards for premium tiers. Supports Apple/ Google Pay. Card controls in- app (freeze, limits) Yes: Launched Joint Accounts (2 people share an account with full legal ownership) in 2024 . Also Shared Spaces sub- accounts that can be co- managed with other users (premium feature) . No child account product. Yes: N26 Business accounts for self-employed (similar features plus 0.1% cashback) Not for registered companies beyond sole proprietors. Moderate: Not tied to an external retail ecosystem; N26 is a pure bank app. It integrates digital services like travel booking (via partner) and an insurance hub in-app, but no marketplace of its own. Limited Premium tiers inc perks (e WeWor credits historic or partn discoun and trav insuran No poin system; Busines gives sm cashbac
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward Yandex Pay (RU) No: Ruble- centric accounts (no multi- currency holding). Focus is domestic payments. (Currency exchange not a feature.) Yes: Free P2P transfers to users and any bank cards (via Russia’s Fast Payment System SBP) – send by phone number easily. Yes: “Pay” Card available virtually for NFC/QR payments ; option to order physical Yandex Pay card (Mastercard or MIR). Supports phone and smartwatch payments (Wear OS) No: No joint or family accounts announced. Individual accounts only (one user per wallet). No: Yandex Pay is consumer- focused. (Yandex’s merchant payment service is YooMoney/ Yandex Checkout, separate from the Pay app.) Yes: Deep integration with Yandex ecosystem – cashback earned as Yandex.Plus loyalty points usable across Yandex services (Taxi, Market, Music, etc.) . Also pay in Yandex services directly. Yes: Up 50% cashbac Yandex. points o selected categor . Ya Plus subscrip boosts rewards benefits across services
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward Kaspi.kz (KZ) Limited: Primarily KZT accounts. Currency exchange available but Kaspi Super-App is domestically focused. (Users can have USD/ EUR deposits via Kaspi Bank, but wallet spend is in KZT.) Yes: Instant free transfers between Kaspi users (phone number or card). Ubiquitous use of QR payments in stores via Kaspi app . (Also supports Kazakhstan’s national instant transfer system.) Yes: Kaspi issues Kaspi Gold debit cards and Kaspi Red (installment credit card). Cards linked to the app; also supports digital QR payments without card. No: No dedicated joint accounts for consumers (each user has their own wallet). Family members can get additional cards on one account, but no multi-user wallet interface. Yes: Kaspi Pay for merchants – the app doubles as a POS for QR payments acceptance . Kaspi also offers SME loans and accounts (Kaspi Business) accessible via the platform for marketplace sellers. Yes: Super- App integration – e-commerce marketplace, groceries, travel booking (Kaspi Travel) , utilities and even government services all built-in . Kaspi seamlessly links payments, fintech, retail and public services in one app. Yes: Kas marketp uses Ka Bonus rewards exclusiv discoun using K paymen method E.g. up t 30% discoun product up to 10 cashbac outside at partn with Ka Card
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward Ozon (RU) No: Ruble accounts (Ozon Bank is Russia- focused). No multi- currency support announced. Yes: Free transfers likely supported (Ozon Bank participates in SBP for instant transfers). Also can send money to any bank card. Yes: Ozon Card (a debit card) – virtual issuance via app, with option for physical card . Offers up to 10% cashback on non- Ozon purchases and big discounts on Ozon marketplace No: No joint/family accounts. Each Ozon Bank account is individual. Yes: Ozon for Sellers – Ozon Bank offers business accounts & loans for marketplace sellers (Launched banking services for entrepreneurs with cashback and discounts tied to marketplace activity.) Yes: Tightly integrated with Ozon e- commerce platform. Users get up to 30% discounts on Ozon products when paying with Ozon Card , and seamless checkout using Ozon account balance or card. Seller hub integration (financing, analytics) via Ozon Bank. Yes: Oz has a po based lo program (“Ozon rewards e.g. 5% in point marketp purchas paid wit Ozon Ca . Plu high int on savin attract deposit
---

Wallet App Multi- Currency & FX Peer-to- Peer Transfers Cards (Physical/ Virtual) Joint/ Family Accounts Business Accounts Ecosystem Integrations Loyalty Reward Wildberries (RU) No: Ruble- based accounts. (Operates in CIS markets with local currency, but no multi- currency wallet functionality for users.) Yes: Free P2P transfers – WB Bank’s “Balance Pay” app allows quick transfers via Russia’s SBP system Also can send to cards. Yes: WB Card – a virtual prepaid bank card launched by Wildberries Bank (with digital wallet) Physical card rollout likely followed. Card is used for Wildberries purchases (max discounts) and outside spending. No: No joint/family accounts. Accounts are individual. Yes: WB Bank for Business – Wildberries Bank was originally set up to handle supplier payments; now offers free business accounts with unlimited transfers Marketplace sellers can use WB Bank for settlements and financing. Yes: Integrated with Wildberries marketplace – paying with the WB wallet yields maximum discounts on Wildberries purchases . Also integrated Wildberries Travel (launched 2023) and other services in-app. Yes: Wildber offers dynami discoun instead points – extra % paying f WB acco Likely h promo cashbac events. may bu perks lik free del for usin card. Other Notable Regional Wallets: Qiwi Wallet (Russia/CIS) and YooMoney (formerly Yandex.Money) are long-standing e-wallets. Both support ruble accounts with easy top-ups and bill payments, P2P transfers, and virtual cards. Qiwi pioneered cash-in kiosks and phone-number accounts with tiered KYC, and it issues Qiwi Visa cards for online spending. YooMoney (now part of Sber) remains a popular online payment method, offering a wallet with a Mastercard, integrations for e-commerce checkout, and installment plan options . However, these older wallets lack the super-app ecosystems of Yandex Pay, Kaspi or Wildberries. For example, YooMoney evolved from a simple electronic wallet into a more bank-like app with transfers, cards, cashback, and even investments (as of 2018) , but Yandex has since shifted focus to the new Yandex Pay app. Qiwi and YooMoney excel in utility payments, mobile top-ups, and person-to- person transfers, which are baseline features now expected in any Uzum Wallet offering. Prioritized Feature Backlog for Uzum Wallet Based on the comparison above and Uzum’s strategic goals, below is a prioritized backlog of features for Uzum Wallet. This prioritization considers which features are must-haves for parity with competitors, which
---

offer high user value, and where Uzum can differentiate by leveraging its ecosystem. Each feature includes a brief rationale: Seamless P2P Transfers (High Priority): Implement instant, fee-free peer-to-peer transfers by phone number or card number. This is a fundamental expectation – e.g. Kaspi and Wildberries offer instant transfers via national systems (SBP in Russia, etc.) . Uzum should leverage local instant payment networks (e.g. Uzcard/Humo rails in Uzbekistan) so users can easily send money to friends and merchants. Rationale: Ubiquitous P2P builds network effects and user engagement (a wallet that “everyone uses to send money”). QR Code Payments for Merchants (High Priority): Enable QR code payment scanning in the wallet for in-store and online purchases. QR payments are popular in CIS super-apps (Kaspi.kz users pay via QR at shops and markets ). Uzum Wallet can integrate Uzbekistan’s QR payment standard or its own system, allowing small businesses to accept Uzum payments (linking to Uzum Bank). Rationale: This feature drives B2B adoption – merchants can use the wallet to accept customer payments, and users get a convenient cashless option. Tying this to Uzum Bank’s merchant acquiring (as Uzum Pay) will expand the ecosystem. Bills, Utilities and Top-Ups (High Priority): Offer comprehensive bill payment (utilities, telecom, taxes) and mobile top-up services in-app. All regional wallets (Qiwi, YooMoney, Kaspi, etc.) heavily feature this. Indeed, Uzum Bank already allows paying for goods, services, and utility bills – the wallet should make these front-and-center. Rationale: Frequent use-cases like paying phone bills or electricity keep users returning to the app regularly, increasing retention. Virtual and Physical Cards (High Priority): Provide an Uzum Wallet virtual card (for online payments and NFC via Google/Apple Pay) and option to order a physical Visa card. Uzum has started issuing virtual cards (700k users) and now physical Visa cards via Uzum Bank Ensuring the wallet app lets users instantly get a card, set PIN, freeze/unfreeze, and use it globally is key. Rationale: Cards extend wallet usage to any POS or website. Competitors like Revolut and Ozon saw success by offering free cards with cashback . Uzum’s physical card integration with Uzum Market pickup points is a smart omnichannel approach that should be expanded nationwide. Unified Loyalty & Cashback Program (High Priority): Integrate Uzum Market loyalty rewards into the wallet. For example, offer cashback or points on purchases across the Uzum ecosystem – marketplace, delivery (Uzum Tezkor), etc., credited to the wallet. Yandex Pay converts spending to Yandex.Plus points , and Ozon/Wildberries give special discounts when paying from their wallets . Rationale: A unified rewards program encourages users to transact within the ecosystem. Uzum can provide bonus discounts on marketplace goods if paid via Uzum Wallet, or accumulate points that can be spent on free deliveries, etc. This differentiates Uzum Wallet by leveraging its multi-vertical platform (e-commerce + fintech). Installments and Micro-Credit (High Priority): Fully integrate Uzum Nasiya (installment plans and microloans) into the wallet experience. Many competitors use built-in credit to boost sales: Yandex Pay’s Split gives users instant installment options , and Kaspi’s BNPL is deeply embedded in checkout . Uzum already offers up to 25 million UZS installment limits via its bank’s Nasiya program . This should be made seamless: e.g. a user shopping on Uzum Market can choose “Pay with Uzum Wallet in 4 installments” at checkout, or a wallet user can get a small cash loan in one tap. 1. 36 2. 26 3. 41 4. 42 43 44 43 5. 24 38 6.
---

Rationale: Integrated lending drives transactions and revenue, and keeps users from seeking third- party credit. Given Uzum’s existing credit arm, this is a strong competitive differentiator in Uzbekistan. Family/Shared Accounts (Medium Priority): Introduce shared wallet features, such as joint accounts for couples or family accounts with sub-cards for children. European fintechs are adding these (Monese joint accounts , N26 Joint Accounts , Revolut <18 for kids ). In CIS, this is not yet common – implementing it could set Uzum apart. For example, two Uzum users could hold a joint balance for household expenses, each with their own card, or parents can create a controlled wallet for a teenager. Rationale: Family features increase user acquisition (bring your partner or child into the ecosystem) and stickiness, appealing to a relatively untapped segment in the region. Multi-Currency Accounts (Medium Priority): Enable multi-currency support in the wallet, e.g. UZS and USD accounts. While local focus is paramount, many Uzbek users hold savings in US dollars or travel abroad. Revolut’s success shows the appeal of holding multiple currencies at good rates Uzum Bank could allow a user to convert and store funds in USD (or EUR/RUB) and spend abroad via the card with low fees. Rationale: This feature would position Uzum Wallet as a pioneer in its market, attracting customers who currently use cash or other banks for FX needs. It leverages Uzbekistan’s liberalizing currency regime and fills a gap left by global fintechs not operating locally. Advanced Security & Fraud Protection (Medium Priority): Match best-in-class security features: biometric login, instant card freeze/unfreeze , dynamic CVV or single-use virtual cards for online payments , spend notifications, and AI-driven fraud monitoring. Revolut’s app, for example, flags high-risk transactions and alerts users in real-time . Rationale: As digital payments grow, users need confidence. Emphasizing Uzum’s secure design (perhaps with Uzbekistan’s nascent cyber standards) will build trust, especially for a new wallet. Many of these features are now standard, so parity is important (Uzum already uses secure digital ID verification for onboarding ). Merchant Tools and SME Integration (Medium Priority): Expand Uzum Wallet for Business – allowing SMEs and marketplace sellers to use the wallet/bank for their finances. Kaspi and Wildberries demonstrated the value of tying merchants into the ecosystem (Kaspi Pay for QR, Wildberries Bank for supplier payments). Uzum can offer an SME version of the wallet app with features like quick onboarding of a merchant account, instant settlement of marketplace sales to the wallet, the ability to pay suppliers or salaries, and easy access to business loans from Uzum Bank. Rationale: This drives B2B adoption and locks in Uzum’s marketplace sellers and even offline businesses into using Uzum’s financial services. It also grows transaction volume via business payments. Given Uzum’s existing focus on individuals and SMEs in its ecosystem , this is a natural extension. Third-Party Integrations (Lower Priority): Consider integrating or offering additional services like insurance and investments through the wallet in the longer term. For example, the wallet could feature an insurance marketplace (selling device insurance or travel insurance to users – similar to what N26 and Revolut do for revenue). Likewise, if regulations allow, a simplified investments offering (such as buying gold, or local bonds) could be a future differentiator once core features are in place. Rationale: These are not immediate must-haves in the Uzbek market now, but planning for them aligns with the super-app trend. They can become new revenue streams and increase user engagement time within the app. 7. 12 45 5 8. 1 9. 46 46 47 48 10. 49 50 11.
---

In summary, must-have core features (P2P, bill pay, cards, loyalty, BNPL) should be delivered first to achieve parity and leverage Uzum’s strengths. Next, differentiators like family accounts, multi-currency, and deepened B2B tools can be introduced to surpass competitors. Throughout, Uzum should capitalize on integration across its ecosystem – something only a few players like Kaspi have done so comprehensively – to offer a one-stop financial and commerce solution. Recommended Competitor Apps for Analysis For further product insight, it is advisable to download and analyze the UX/UI and feature flows of the following competitor applications. This can reveal details on onboarding, navigation, and hidden features (e.g. how KYC is handled, how cards are managed, how loyalty is presented). Below is a list of recommended apps (Android APKs or App Store links) to inspect, with source links: Revolut – All-in-one finance app (global). (Android: Google Play Store, iOS: App Store) Monese – Mobile money account for expats (UK/EU). (Android: Google Play Store, iOS: App Store) N26 – Digital bank app (EU/Germany). (Android: Google Play Store, iOS: App Store) Yandex Pay – Wallet & banking app (Russia). (Android: Google Play Store) Kaspi.kz – Kaspi Super App (Kazakhstan). (Android: Google Play – “Kaspi Книга” app) Ozon Bank – Ozon_card finance app (Russia). (Android: Google Play Store) Wildberries Balance Pay – Wildberries Bank app (Russia/CIS). (Android: Google Play Store) Qiwi Wallet – Digital wallet (Russia/CIS). (Android: Google Play Store – “QIWI Кошелек”) YooMoney – E-wallet formerly Yandex.Money (Russia). (Android: Google Play Store – “ЮMoney”) Each of these apps offers valuable reference: for example, Yandex Pay and Wildberries Balance Pay will show how ecosystem services (loyalty, installments) are integrated into the UI, while Revolut and N26 showcase best practices in clean design and feature discoverability (and how they handle complex features like trading within a simple interface). Kaspi.kz’s app (likely accessible via APK sites if not in Play Store internationally) is a must-see for its super-app navigation (payments, marketplace, government services all- in-one). By tearing down these apps, the Uzum team can gather inspiration on feature implementation and identify gaps to refine the Uzum Wallet product backlog further.
---

Revolut is Revolutionizing Banking with a Digital Twist https://www.linkedin.com/pulse/revolut-revolutionizing-banking-digital-twist-bob-bartleson-ptyye Revolut: Spend, Save, Trade - Apps on Google Play https://play.google.com/store/apps/details?id=com.revolut.revolut&hl=en_US Full Review: Monese (2024) Pros & Cons, Fees, Plan Comparison https://fintechcompass.net/personal-bank-accounts/monese/ Joint account - Monese https://www.monese.com/products/joint-account?lng=en Business Account - Monese https://www.monese.com/business?lng=en N26 Review 2025: A Deep Dive into Its Features, Services, and User Experience https://www.btcc.com/en-US/square/R0thIRANexus/703311 N26 Launches Joint Accounts in 21 New Markets to Enable Customers to Manage Finances as a Couple https://ffnews.com/newsarticle/fintech/n26-launches-joint-accounts-in-21-new-markets-to-enable-customers-to-manage- finances-as-a-couple/ Compare N26 bank accounts and features for freelancers https://n26.com/en-eu/plans-business Yandex Pay: pay as you like - Apps on Google Play https://play.google.com/store/apps/details?id=com.yandex.bank&hl=en_CA How Does Kaspi.kz JSC Company Work? – PortersFiveForce.com https://portersfiveforce.com/blogs/how-it-works/kaspi [PDF] Kaspi.kz launches online travel platform Kaspi Travel https://ir.kaspi.kz/media/Press%20Release_Kaspi%20Travel.pdf Kaspi | Fin-Tech Super-App Business - by James Emanuel https://rockandturner.substack.com/p/kaspi-fin-tech-super-app-business Ozon launches banking service for entrepreneurs https://corp.ozon.com/tpost/zmv8ylh991-ozon-launches-banking-service-for-entrep Ozon Банк: выгодные покупки - Apps on Google Play https://play.google.com/store/apps/details?id=ru.ozon.fintech.finance&hl=en_US Ozon.Card goes digital https://corp.ozon.com/tpost/p1kp86ijz1-ozoncard-goes-digital Ozon Банк – кешбэк до 25% и бесплатное обслуживание для ... https://finance.ozon.ru/ Balance Pay - Apps on Google Play https://play.google.com/store/apps/details?id=ru.wildberries.wbpayclient&hl=en_US Wildberries - Wikipedia https://en.wikipedia.org/wiki/Wildberries
---

WB Банк — Для частных лиц и бизнеса https://wb-bank.ru/ Yandex.Money moves mobile wallet forward with Moven https://www.fintechfutures.com/digital-payments/yandex-money-moves-mobile-wallet-forward-with-moven Uzum Bank and Visa Launch Plastic Debit Cards - News - Uzum https://uzum.com/en/press-center/news-and-press-releases/uzum-bank-and-visa-launch-plastic-debit-cards/ Ozon Банк - App Store https://apps.apple.com/us/app/ozon-%D0%B1%D0%B0%D0%BD%D0%BA/id1629869891 Uzum: Main Page https://uzum.com/ Uzum Secures $70 M Equity Financing Led by Tencent and VR ... https://www.theasianbanker.com/mediafeed-news/details?rkey=20250805AE44452&filter=23792&pd=05%20Aug%202025
