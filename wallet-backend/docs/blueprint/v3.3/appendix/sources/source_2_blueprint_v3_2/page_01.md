# SOURCE_2__Blueprint_v3_2

## Page 01

Uzum Wallet MVP Blueprint (v3.2)
Version 3.2 – Contract-First Monolith Architecture (Uzbekistan)
Executive Summary
MVP boundaries: Multi-currency wallet with user balances and a double-entry ledger, internal P2P
transfers, FX conversion (quote + execute), admin ledger operations (mint, burn, reverse),
transaction history with receipts, and a sandbox “spend” simulation
.
Explicitly NOT in MVP: KYC onboarding, external payment rails (no card or bank top-ups/
withdrawals), merchant payment integration, automated dispute/chargeback handling, or credit/
loan features
. These are deferred beyond MVP.
Core guarantees: Immutable journal (no deletion or alteration of posted transactions) and atomic
multi-ledger postings, with strict idempotency enforcement on all money-moving endpoints
.
Every financial request is processed exactly once, or not at all, even under retries.
Infrastructure: Back-end is a contract-first monolithic service implemented in Rust (replacing the
earlier Go approach) for safety and performance. Data persists in PostgreSQL (required for
production) with strong consistency; SQLite can be used for local dev/testing with migrations to
Postgres
. Full audit logging and basic monitoring are built in.
MVP v3.2 updates: No change to the core scope or architecture from v3.1
. The v3.2 iteration
focuses on removing ambiguity in the specification and tightening definitions (e.g. how recipients
are identified, how FX rates are obtained, idempotency key retention)
. It adds a Product Decision
Record (Q&A) section addressing these design choices, and expands the post-MVP feature roadmap
based on a comparative analysis of leading wallets
.
Changelog (v3.1 → v3.2)
P2P Recipient Addressing: Extended internal transfer capability to allow specifying the recipient by
phone_number  or user_id  (in addition to account_id ). The service will resolve such
identifiers to the target account_id  at transaction time, using the user directory. Note: to prevent
deadlocks, the transfer logic will always lock accounts in a consistent order (e.g. by user_id )
.
Admin Search API Filters: Defined the admin search endpoint ( GET /v1/admin/search  ) to
accept minimal query filters – user_id , account_id , tx_id, currency, time_range – returning up
to 100 matching transactions or accounts. Clarified that pagination can be handled on the frontend
(no server-side cursor for MVP)
.
FX Rate Source: Locked FX conversion in MVP to use a static fx_rates table in the database as the
source of currency exchange rates. The POST /v1/fx/quote  operation will read from this table
(with pre-configured rates/markup)
. Future integrations with public FX APIs or dynamic feeds are
noted as post-MVP enhancements, but for MVP all quotes use the internal table. Each FX quote
record captures the rate (and fee) at quote time as an immutable snapshot for transparency.
Idempotency Key Retention: Established a standard 48-hour TTL for stored idempotency keys.
Idempotency records (used to prevent double-processing of POST requests) will be persisted for 48
- 1
- 2
- 3
- 3
- 4
5
6
- 7
- 8
- 9
- 1
