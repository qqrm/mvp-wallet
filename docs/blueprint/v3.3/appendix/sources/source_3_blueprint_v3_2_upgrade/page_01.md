# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 01

Uzum Wallet MVP Blueprint (v3.2 Upgrade) –
Contract-First Monolith (Uzbekistan)
Version: 3.2 | Date: 2026-??-?? | Audience: CEO, Product, Engineering, Security, Compliance
Executive Summary
Uzum  Wallet  MVP  v3.2  builds  upon  the  v3.1  foundation  by  clarifying  under-specified  elements  and
incorporating  key  product  decisions  to  ensure  a  production-grade implementation.  The  core  scope
remains a multi-currency wallet ledger with internal P2P transfers, FX conversion (quote & execute),
admin ledger ops, transaction history, and sandbox spend simulation. No changes to the monolithic,
contract-first architecture or MVP boundaries – external payment rails, KYC, merchant integrations, and
credit products remain out-of-scope for MVP
. The v3.2 update focuses on removing ambiguity in APIs
and data model (e.g. how recipients are addressed, how FX rates are sourced, idempotency retention), and
adds a “Product Decision Record” section capturing Q&A on these design choices. We also expand the post-
MVP  feature  roadmap based  on  a  comparative  analysis  of  leading  wallets,  highlighting  future
enhancements (QR payments, loyalty, installments, etc.) to guide strategic planning. These refinements aim
to make the MVP spec implementation-clear and aligned with fintech-grade standards, without altering the
core MVP architecture or delivery timeline. All financial invariants (atomic double-entry postings, immutable
journal, strict idempotency) and production requirements (PostgreSQL, audit logging, basic fraud controls)
from v3.1 are upheld in v3.2.
Changelog (v3.1 -> v3.2)
P2P Recipient Addressing: Extended internal transfer capability to allow specifying the recipient by
phone_number  or user_id  (in addition to account_id ). The service will resolve such
identifiers to the target account_id  at transaction time, using the user directory. Note: to prevent
deadlocks, the transfer logic will always lock accounts in a consistent order (e.g. by user_id).
Admin Search API Filters: Defined the admin search endpoint ( GET /v1/admin/search ) to
accept minimal query filters – user_id, account_id, tx_id, currency, time_range – returning up to
100 matching transactions or accounts. Clarified that pagination can be handled on the frontend (no
server-side cursor for MVP).
FX Rate Source: Locked FX conversion in MVP to use a static fx_rates table in the database as the
source of currency exchange rates. The POST /v1/fx/quote  operation will read from this table
(with pre-configured rates/markup)
. Future integrations with public FX APIs or dynamic feeds are
noted as post-MVP enhancements, but for MVP all quotes use the internal table. Each FX quote
record captures the rate (and fee) at quote time as an immutable snapshot for transparency.
Idempotency Key Retention: Established a standard 48-hour TTL for stored idempotency keys.
Idempotency records (used to prevent double-processing of POST requests) will be persisted for 48
hours, after which they may be purged/expired. This ensures clients can safely retry for a reasonable
window without persistent storage growth.
1
-
-
- 2
- 1
