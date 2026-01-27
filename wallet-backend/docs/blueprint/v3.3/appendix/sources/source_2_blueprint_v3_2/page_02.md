# SOURCE_2__Blueprint_v3_2

## Page 02

hours, after which they may be purged/expired. This ensures clients can safely retry for a reasonable
window without persistent storage growth
.
Transaction Receipt Schema: Enhanced the transaction receipt model to include a complete
breakdown of the transaction details. Receipts (returned by GET /v1/transactions/{tx_id} )
will list all involved accounts with amounts and debit/credit direction, the transaction type and
timestamp, and any FX rate and fee applied (for conversion transactions). An optional memo or
merchant_label is included for user-friendly context (e.g. user-entered note or merchant name on a
spend). The API will exclude internal-only fields, showing only user-meaningful data (e.g. no raw
internal IDs besides the transaction ID reference)
.
Product Decision Record (PDR): Added a Q&A style section summarizing key product decisions
resolved in v3.2 (e.g. choice of 48h idempotency retention, using a static FX rate table, handling of
history pagination) along with brief explanations
.
Extended Post-MVP Roadmap: Appended a feature backlog derived from comparative market
analysis, grouping and prioritizing future enhancements in categories (Payments/Rails, Compliance,
UX, Wallet Capabilities, Ops & Support). High-priority items include phone-number P2P and QR
payments, loyalty rewards integration, installment (BNPL) features, joint/family accounts, virtual/
physical cards, merchant/SME tools, and a potential insurance/investments marketplace, among
others
. These are not in MVP scope but inform the next phases.
Product Scope & Personas
End-User Persona: A consumer in the Uzum ecosystem who holds money in local (UZS) and foreign (USD)
currency balances. They need to send and receive money easily with other users and perform transparent
currency exchange, with an immediate immutable receipt for each transaction. In MVP, users can transfer
funds internally to other users by selecting a contact (now by phone number or user ID) or entering an
account number
. They can also convert money between currencies within their wallet. The experience
emphasizes speed (instant transfers), no fees for internal P2P, and clear confirmation of each transaction
(balance updates and detailed receipts).
Admin/Operations Persona: Back-office staff or support agents who manage the ledger system. They can
create or close user accounts, credit or debit user balances (mint or withdraw), and perform compensating
reversals to correct errors
. They have basic search tools to look up transactions or accounts by various
identifiers for customer support and reconciliation. MVP assumes a single-admin authority model (no dual
approval) but with full audit logging of all admin actions. Admins use the search API to filter transactions (by
user, account, etc.) when investigating issues or assisting users
.
MVP Scope Summary: The MVP covers multi-currency wallet accounts per user (UZS and USD), an internal
double-entry ledger for all transfers, and basic wallet operations: internal P2P transfers, FX conversion
(quote & execute), viewing transaction history with receipts, admin ledger controls (open/close accounts,
mint,  burn,  reverse),  and  a  sandbox  spend  simulation
.  It  does  not include  external  top-ups  or
withdrawals (no card/bank linkages), merchant payments, or KYC onboarding flows – those are deferred to
post-MVP phases
.
10
- 11
12
- 13
- 14
15
16
17
18
19
2
