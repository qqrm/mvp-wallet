# Wallet Backend — Architecture & Invariants

This document defines the **core architectural rules and invariants**
for the Wallet Backend system.

This is the **source of truth** for correctness.
Violations of these rules are considered **critical bugs**.

---

## 1. Money safety invariants

### 1.1 No implicit user creation
- Any monetary operation must fail with `404 user not found`
  if the user does not exist.
- Only `/v1/admin/...` endpoints are allowed to create users.

### 1.2 Atomic money movements
- A single API call must either:
  - fully apply (ledger tx + entries + projection), or
  - not apply at all.
- All money operations must run inside **one DB transaction**.

### 1.3 Double-entry ledger
- For every `tx_id`:

sum(credits) - sum(debits) == 0

- Each operation must:
- insert exactly one `ledger_transactions` row,
- insert matching `ledger_entries` rows.

### 1.4 Balance projection
- `balance_projection` is a **derived view**:

balance_projection(account, currency)
== sum(ledger_entries for that account & currency)

- Projection updates must be done in the **same transaction**
as ledger inserts.

### 1.5 No negative user balances
- User wallets must never reach:

available_minor < 0

- This must be enforced **atomically in SQL**
(conditional UPDATE), not by pre-checks.

---

## 2. Idempotency model

### 2.1 HTTP idempotency
- All POST endpoints that create ledger operations
require `Idempotency-Key`.
- Same key + same request → same response.
- Same key + different request → `409 conflict`.

### 2.2 Scope
- Idempotency is scoped by:

(actor_scope, idempotency_key)

Example:
- `user:{user_id}:transfer`
- `admin:topup`

### 2.3 Stored responses
- The system must store:
- HTTP status code,
- full JSON response body.
- Replays must return **bit-identical responses**.

---

## 3. Payment lifecycle (FSM)

Payments follow a strict state machine:

authorized → posted → refunded
authorized → canceled


Forbidden transitions:
- refund before capture
- capture after cancel
- double refund
- double capture

State transitions must be enforced:
- at the **database level** (constraints / checks),
- and validated in the service layer.

---

## 4. RBAC & security model

### 4.1 Authentication
Non-localhost requests require:

Authorization: Bearer <token>

Localhost requests (loopback IP or `Host: localhost`) bypass token auth and derive identity from `X-Dev-User` or `?as=` (default `u01`).


### 4.2 Admin
- Token: `ADMIN_TOKEN` (env)
- Permissions:
  - create users
  - open currency accounts
  - topup
  - view any user

### 4.3 User
- Token format:

user:<user_id>:<sig>

- Can only access:

  - /v1/wallet/<same_user_id>/...
  - /v1/transactions/<tx_id>  (only if the tx belongs to the user)


---

## 5. Currency & accounts

### 5.1 Explicit currency accounts
- A user must explicitly open a currency account before use:

POST /v1/admin/users/{user_id}/accounts

- Money operations must fail with:

409 currency account not opened


### 5.2 System accounts
- System cash account may:
- auto-open currencies,
- allow negative balances.

---

## 6. Implementation principles (Rust)

- Core domain logic must be pure and testable.
- Use validated types:

TryFrom<Req> for ReqValidated

- Use newtypes:
- `UserId`
- `Currency`
- `AmountMinor`
- `IdempotencyKey`

No hidden side-effects:
- No automatic creation inside money flows.
- No "ensure" helpers that mutate state implicitly.

---

## 7. Observability guarantees

The system must support:
- full traceability per transaction,
- correlation IDs across:
- API request,
- ledger tx,
- outbox event.

---

## 8. Non-goals

This system does NOT:
- support floating point money,
- modify ledger history,
- auto-correct invalid states,
- allow best-effort money operations.

Money is strictly **append-only, immutable, and auditable**.
