# SOURCE_2__Blueprint_v3_2

## Page 03

Domain Model & Data Invariants
Core Entities:
User – End-user record (with unique user_id), including personal identity info (phone, email, etc.),
status flags, and linkage to one or more wallet accounts. Each user belongs to a tenant (Uzum
ecosystem tenant, single-tenant for now) and can have one default account per currency.
Account – Ledger account representing a wallet balance in a specific currency. Key fields: account_id,
user_id (owner), currency, status, created_at. Each user/currency pair has one account (MVP supports
UZS and USD accounts per user). Accounts can be ACTIVE or CLOSED (closed accounts are
suspended from use, but never physically deleted).
JournalTx – A financial transaction header (journal entry) with tx_id (unique), type (e.g. P2P transfer,
FX, admin adjustment), status, timestamp, and correlation fields. Every money movement in the
system is a JournalTx, which links to one or more JournalEntry lines. Once posted, JournalTx records
are immutable (no updates/deletes) as part of the audit trail invariant.
JournalEntry – The double-entry line items associated with a JournalTx. Each entry records an
account_id, amount, and side (debit or credit). For each JournalTx, total debits equal total credits in
each currency, ensuring ledger balance. JournalEntries also carry a reference to the tx_id and
perhaps a short description (e.g. “P2P transfer from X to Y”).
FxQuote – An FX conversion quote record. Created when a user requests a currency conversion
quote. Fields: quote_id, from_account_id, to_account_id, amount_from, rate, fee, expires_at, status.
The quote holds a snapshot of the exchange rate (and any fee or markup) offered to the user, and an
expiration timestamp (e.g. 60 seconds from quote creation) after which the quote can no longer be
executed. In v3.2, quotes use the static FX rate table as source; the rate and fee stored in FxQuote
represent the exact terms given to the user at that time. (No separate version ID for the rate is
needed, since the quote records the actual rate used.)
FxRate – (New in v3.2) A static reference table of currency exchange rates. This table holds the
authoritative rates used by the wallet for FX quotes in MVP. Fields might include currency pair (e.g.
UZS/USD), current rate, an optional markup or fee percentage, and a last_updated timestamp. The
MVP will preload or maintain this table with the applicable market rates (e.g. derived from Central
Bank or internal treasury) and possibly a small markup. All FX quote requests will read from this
table to determine the rate/fee offered. (In future, this could be replaced or updated via external API
feeds – see Product Decision Record and Roadmap for details.)
IdempotencyRecord – Record of a past POST request identified by an Idempotency-Key. Contains
the key value, the requesting actor_id (user or admin), the endpoint and a hash of the request
payload, plus the resultant status code and response body, and a timestamp. In v3.2, an additional
field/attribute tracks the expiration of this record (TTL). The system retains idempotency records for
48 hours from creation, after which they may be purged
. This prevents indefinite growth of the
table while covering typical client retry windows.
AuditEvent – Append-only log of significant actions and system events for security and compliance.
Each event has an event_id, actor info (who performed the action and their role/permissions), action
type (e.g. ADMIN_FUND, USER_TRANSFER), target identifiers (which user/account/tx were affected), a
timestamp, and context like IP or device ID. All admin operations and all financial transactions
trigger audit events. These are retained long-term (at least 2–5 years as per compliance needs).
-
-
-
-
-
-
- 20
- 3
