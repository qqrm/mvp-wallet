# SOURCE_2__Blueprint_v3_2

## Page 04

Data Invariants:
Ledger Immutability: Once a JournalTx and its JournalEntries are marked POSTED/COMPLETED,
they cannot be altered or deleted. Corrections are done via new transactions (e.g. a reversal) rather
than editing history. This guarantees an immutable ledger for audit and reconciliation
.
Double-Entry Balance: For every JournalTx, the sum of debit entries equals sum of credit entries in
each currency involved. This ensures no money is created or lost in transit. The system enforces this
with a DB check constraint or in-code validation.
Consistent Account States: Account statuses and balances follow defined rules. For MVP, blocked
balance is always zero (holds are not used in MVP
), so an account’s available balance equals its
total ledger balance. If in future “holds” are introduced (for card authorizations, etc.), the schema will
adjust, but MVP assumes no pending holds. Closing an account (status = CLOSED) prevents new
debits/credits but does not remove the record or its balance history.
Tenant and Group Structure: MVP is single-tenant (all users under the same umbrella). The schema
has a placeholder for tenant_id on user/accounts for future multi-tenant support. Similarly, while
MVP is single-owner per account, the schema reserves a wallet_group_id field to support shared/
joint wallets in the future (e.g. family accounts). No group functionality is active in MVP beyond this
placeholder
.
Balance Model & Transaction Lifecycle
Account Lifecycle: Accounts start as ACTIVE and can be CLOSED by an admin (closed accounts
cannot be used for new transactions, but their history remains)
. MVP does not implement
account freezes or holds beyond this status change.
Internal Transfer (P2P): Validate both source and recipient accounts are ACTIVE and source has
sufficient balance, then create a JournalTx with two JournalEntries (debit sender’s account and credit
receiver’s account)
. The transfer posts atomically; on success, both accounts’ balances update.
Admin Fund (Mint): An admin can credit a user’s balance by “minting” money into the system. The
JournalTx debits a system mint account (SYSTEM_MINT) and credits the user’s account
.
Admin Withdraw (Burn): An admin can deduct a user’s balance by “burning” money out of
circulation. The JournalTx debits the user’s account and credits a system sink account (SYSTEM_SINK)
. (This models cash-out in MVP, since external outflows are not integrated.)
Admin Reverse: To correct mistakes, an admin can reverse a prior transaction by creating a
compensating JournalTx that debits the account that was previously credited and credits the one
that was debited
. Every reversal is a new transaction with its own tx_id, linked to the original
transaction for audit.
Spend Simulation: The user can simulate a spend (for testing) which transfers an amount from the
user’s account to a designated system spend account (SYSTEM_SPEND), optionally attaching a
merchant label or note
. This does not involve external parties but helps test the end-to-end flow
(debit user, credit system account) and receipt generation.
API Specification & Contracts
User API Endpoints ( /v1/* ):
POST /v1/transfers  – Create an internal P2P transfer from one user account to another.
Idempotent: Yes (Idempotency-Key required)
.
- 21
-
- 22
- 23
- 24
- 25
- 26
- 27
- 28
- 29
- 30
4
