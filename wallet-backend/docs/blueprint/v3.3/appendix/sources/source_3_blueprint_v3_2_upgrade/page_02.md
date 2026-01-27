# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 02

Transaction Receipt Schema: Enhanced the transaction receipt model to include a complete
breakdown of the transaction details. Receipts (returned by GET /v1/transactions/{tx_id} )
will list all involved accounts with amounts and debit/credit direction, the transaction type and
timestamp, and any FX rate and fee applied (for conversion transactions). An optional memo or
merchant_label is included for user-friendly context (e.g. user-entered note or merchant name on a
spend). The API will exclude internal-only fields, showing only user-meaningful data (e.g. no raw
internal IDs besides the transaction ID reference).
Product Decision Record (PDR): Added a Q&A style section summarizing key product decisions
resolved in v3.2 (e.g. choice of 48h idempotency retention, using a static FX rate table, handling of
history pagination) along with brief explanations.
Extended Post-MVP Roadmap: Appended a feature backlog derived from comparative market
analysis, grouping and prioritizing future enhancements in categories (Payments/Rails, Compliance,
UX, Wallet Capabilities, Ops & Support). High-priority items include phone-number P2P and QR
payments, loyalty rewards integration, installment (BNPL) features, joint/family accounts,
virtual/physical cards, merchant/SME tools, and a potential insurance/investments
marketplace, among others. These are not in MVP scope but inform the next phases.
Product Scope & Personas
End-User Persona: A consumer within the Uzum ecosystem who needs to hold money in local and foreign
currency balances, send and receive money easily with other users, and perform transparent currency
exchange – all with immediate, immutable receipts for each transaction. In MVP, users can transfer funds
internally to others by selecting a contact (now by phone number or user ID as well) or entering account
details,  and  can  convert  currency  within  their  wallet
.  The  experience  emphasizes  speed  (instant
transfers), no fees for internal P2P, and clarity of each transaction (balance updates and receipts).
Admin/Operations Persona: Back-office staff or support agents who manage the ledger system. They can
create or close user accounts, credit or debit user balances (mint or withdraw in ledger terms), and perform
compensating reversals for errors. They also have search tools to lookup transactions or accounts by
various identifiers for customer support and reconciliation. MVP assumes a single-admin authority model
(no multi-approval) with full audit logging of all admin actions
. Admins use the search API to filter
transactions (by user, account, etc.) when investigating issues or fulfilling support requests.
(MVP Scope Summary: The MVP provides wallet accounts per currency, an internal double-entry ledger
for all transfers, and basic wallet operations: P2P transfer, FX conversion, transaction history with receipts,
admin ledger controls (open/close accounts, mint, burn, reverse), and a sandbox “spend” simulation. It does
not include external top-ups or withdrawals (no card/bank linkage), merchant payments, or KYC onboarding
flows
. These are deferred to future versions, as is any credit/loan functionality.)
Domain Model & Data Invariants
Core Entities: The v3.2 data model retains all v3.1 entities with one addition for FX rates:
User – End-user record (with unique user_id ), including personal identity info (phone, email, etc.),
status flags, and linkage to one or more wallet accounts
. Each user belongs to a tenant (Uzum
ecosystem tenant, single-tenant for now) and can have one default account per currency.
-
-
- 3
4
5
- 6
2
