# SOURCE_2__Blueprint_v3_2

## Page 05

POST /v1/fx/quote  – Request a currency exchange quote (two-step FX flow). Returns a quote
record with rate, fee, and an expiration timestamp
. Idempotent: Yes (client can safely retry; each
attempt generates a new quote).
POST /v1/fx/execute  – Execute a previously obtained FX quote, performing the debit and credit
in respective accounts if the quote is still valid
. Idempotent: Yes (only one execution succeeds;
duplicate calls return the same result or an error)
.
POST /v1/spend/simulate  – Simulate a spend (transfer to a system spend account) for testing
or sandbox use
. Idempotent: Yes.
GET /v1/accounts/{id}  – Fetch account details (status, currency, created_at) for a user’s
account.
GET /v1/accounts/{id}/balance  – Fetch the balance breakdown of a user’s account (available
and blocked amounts). Blocked will be 0 in MVP (no holds).
GET /v1/accounts/{id}/transactions?cursor=&limit=  – List transaction history for an
account (paginated by cursor). Returns recent transactions with their receipt details.
GET /v1/transactions/{tx_id}  – Retrieve the full receipt for a specific transaction (if the
requesting user has access to it). Includes details of the transaction, amounts, and updated balances
.
Admin API Endpoints ( /v1/admin/* ):
POST /v1/admin/accounts/{id}/close  – Close (suspend) a user account. Closed accounts
cannot be used for new transactions (no deletions in ledger)
.
POST /v1/admin/fund  – Mint funds into a user’s account (increase balance, credit user, debit
system mint pool)
.
POST /v1/admin/withdraw  – Burn funds from a user’s account (decrease balance, debit user,
credit system sink)
.
POST /v1/admin/reverse  – Reverse a previous transaction by tx_id (create compensating journal
entries to negate a specific transaction)
.
GET /v1/admin/search  – Search for transactions or accounts by filters (e.g. user_id, account_id,
tx_id, currency, time range)
. Returns up to 100 results per query (no server-side pagination;
refine query to narrow results). Used for ops and support lookups.
All POST endpoints that modify financial state require an Idempotency-Key header and implement robust
idempotency (duplicate requests with the same key will not be processed twice)
. GET endpoints are
side-effect-free and do not require idempotency keys. All list endpoints use cursor-based pagination (with
limit  and cursor  parameters) for stable ordering
.
Error Handling: Errors are returned with HTTP appropriate status codes and a JSON body containing an
error  object. For example, a failed transfer due to insufficient funds might return:
{
"error": {
"code": "INSUFFICIENT_FUNDS",
"message": "Not enough available balance",
"details": { "account_id": "acc_sender", "available": 120000, "required":
200000 }
- 31
- 32
33
- 29
-
-
-
- 34
35
- 24
- 26
- 27
- 36
- 37
30
38
39
5
