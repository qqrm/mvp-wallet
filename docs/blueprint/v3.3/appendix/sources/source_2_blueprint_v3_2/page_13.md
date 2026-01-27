# SOURCE_2__Blueprint_v3_2

## Page 13

single primary database node). Long-term, data retention policies (e.g. archiving old audit logs) will be
implemented as needed to comply with regulations and manage storage.
Database Schema (MVP Core)
The core PostgreSQL schema for MVP includes the following tables (with key fields):
tenants – (Future use) Tenant organizations; MVP is single-tenant so this has one entry. Fields:
tenant_id (PK), name, status.
users – Wallet users. Fields: user_id (PK), tenant_id, phone/email, status (ACTIVE/BLOCKED), etc.
.
accounts – Currency accounts tied to users. Fields: account_id (PK), user_id (owner), currency, status
(ACTIVE/CLOSED), created_at
. (Each user has one account per currency in MVP.)
journal_tx – Ledger transactions (journal headers). Fields: tx_id (PK), type (TRANSFER, FX, admin_op,
etc.), status, reference_ids (like correlation_id), timestamps.
journal_entry – Ledger entries (double-entry lines). Fields: entry_id (PK), tx_id (FK to journal_tx),
account_id (FK to accounts), amount, side (debit/credit), and an entry_type or description
.
fx_quote – FX conversion quotes. Fields: quote_id (PK), from_account_id, to_account_id,
amount_from, rate, fee, expires_at, status
.
fx_rates – Exchange rate reference data (static). Fields: currency_pair, rate, markup, last_updated
.
(MVP uses this for all FX conversions.)
idempotency_record – Idempotency key log. Fields: key (PK or unique), actor_id, endpoint,
request_hash, response_code, response_body, created_at, expires_at (TTL)
.
audit_event – Security audit log. Fields: event_id (PK), actor_id, role, action_type, target_ids (e.g.
account_id or tx_id involved), timestamp, metadata (IP, device)
.
(Other support tables like an outbox or reference data tables may exist, but the above are the primary ones for
MVP.)
Mobile App UX (MVP)
The Uzum Wallet will surface in the Uzum mobile app via new screens built to the Uzum UI design kit. The
key user flows and interfaces include:
Home Dashboard: Shows the user’s wallet balances for each currency (e.g. UZS and USD) with quick
action buttons
. Users can switch currency views and initiate actions like “Transfer” or “Convert”
from this screen. Notifications or status banners (e.g. maintenance alerts) may also appear here.
Transfer Money: A flow to send money to another user. The user selects the source account (if
multiple currencies) and specifies a recipient. In MVP this can be done by choosing from contacts or
entering a phone number/user ID (which the app will resolve to an internal account) or by directly
inputting an account ID
. The UI will show the recipient’s name if known. The user enters an
amount and an optional note. A review screen then confirms details (recipient, amount, any fee = 0
for internal) before submission. After a successful transfer, a confirmation with the receipt is shown,
and the user can share or repeat the transfer
.
Currency Conversion: A two-step FX flow. The user selects “Convert Currency”, chooses the source
and target currency (only UZS⇄USD in MVP), and enters an amount to convert. The app calls
POST /v1/fx/quote  and then displays a quote screen showing the exchange rate, fee (if any),
-
- 95
- 95
-
- 96
- 96
- 77
- 97
- 51
- 98
- 99
82
100
- 13
