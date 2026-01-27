# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 05

API Specification & Contracts (v1)
The MVP API is versioned under /v1  and uses a RESTful contract-first design. All client-facing endpoints
and  request/response  schemas  are  predefined  (OpenAPI)  and  kept  in  sync  with  implementation.
Authentication is via JWT (Uzum SSO tokens), with role-based authorization ( USER  vs ADMIN ) enforced on
protected endpoints
. Idempotency-Key header is required on every POST that modifies financial state
(transfers, FX execution, admin fund/withdraw, etc.)
. Standard HTTP error codes and a stable JSON
error envelope are used (e.g. 400 for validation errors, 409 for idempotency conflicts, 401/403 for auth
issues, etc.)
.
Admin API Endpoints: (require ADMIN role)
- POST /v1/admin/accounts  – Create a new account for a user in a given currency. Allows ops to set
up additional currency accounts for a user. Idempotent: Recommended (client may retry on failure).
- POST /v1/admin/accounts/{id}/close  – Close (suspend) an account. Marks an account as closed
(no new transactions allowed). No hard deletes. Idempotent: Yes (re-closing an already closed account is a
no-op).
- POST /v1/admin/fund  – Mint funds into a user’s account. The request specifies target account_id ,
amount,  and  reference.  Internally  posts  a  JournalTx  debiting  SYSTEM_MINT  and  crediting  the  user.
Idempotent: Yes (required, to avoid duplicate mints).
- POST /v1/admin/withdraw  – Burn funds (withdraw) from a user’s account. Debits the user and credits
SYSTEM_SINK. Essentially the inverse of fund. Idempotent: Yes (required).
-  POST /v1/admin/reverse  –  Reverse a transaction. Takes a  tx_id  of an existing JournalTx and
creates a compensating transaction (credits and debits swapped) to negate it. Used for remediation of
mistakes. Idempotent: Yes (required, ensure a given tx_id is reversed only once).
-  GET /v1/admin/search  –  Search ledger records by filters. Allows admins to query transactions or
accounts based on user_id , account_id , tx_id , currency , and/or a time range. Returns up to
100 results matching the criteria (e.g. transactions in date range, or all accounts for a user). The response is
not paginated on the server; if more results are needed, the admin can refine filters or time window (the
assumption is administrative queries are targeted and infrequent). This endpoint helps support staff quickly
locate a specific transaction or set of entries by key identifiers. (No idempotency needed for GET.)
User Wallet API Endpoints:
- GET /v1/profile  – User Profile & Defaults. Returns user’s basic profile and default account IDs (e.g.
their primary UZS account and USD account) for convenience.
- GET /v1/accounts/{id}  – Account Details. Returns metadata of a specific account (currency, status,
created date, etc.), verifying ownership.
- GET /v1/accounts/{id}/balance  – Account Balance. Returns the current balance of the account,
broken down into available and blocked amounts (blocked will be 0 in MVP). Also may include the account’s
transaction count or last transaction timestamp for reference.
-  GET  /v1/accounts/{id}/transactions?cursor={cursor}&limit={N}  –  Transaction  History.
Retrieves a paginated list of recent transactions for the given account. Supports cursor-based pagination
(cursor points to a tx_id or timestamp; limit  up to 50 or 100). Ordered by newest first (descending by
date/tx_id). This allows users to scroll through their own history. (For MVP, basic filtering by type or date is not
provided on this endpoint – only chronological pagination.)
-  GET  /v1/transactions/{tx_id}  –  Transaction  Receipt. Fetches  the  full  details  of  a  specific
transaction (if the user has access to it). The receipt includes: the transaction’s status (e.g. COMPLETED),
18
19
20
21
22
5
