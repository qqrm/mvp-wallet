# SOURCE_2__Blueprint_v3_2

## Page 11

future, also introduce automated anomaly detection on transaction patterns to catch potential fraud
or system errors early.
Dual Approval for Sensitive Actions: As noted, introduce a “four-eyes” approval process for high-
risk admin operations. In a future version, certain actions (e.g. very large fund/reversal operations)
would require a second admin’s approval before execution, to reduce insider risk. MVP relies on
audit tracking, but adding this control is on the roadmap given its importance for a mature financial
platform.
Product Decision Record (Q&A)
Q: How should users specify a transfer recipient – only by account number, or can we use phone
numbers/user IDs?
A: We decided to support multiple addressing methods in MVP v3.2. Users can send money by providing the
recipient’s phone number or internal user_id, not just an account ID
. The backend will look up the
phone  or  user  ID  to  find  the  target  account  (using  the  default  currency  account  if  multiple).  This
improvement leverages the expectation in our markets that sending to a phone number is standard, while
keeping it all internal (no external directory needed). It greatly improves UX for P2P transfers.
Q: What source of FX rates will the wallet use for currency conversion in MVP?
A: For MVP, we chose to use a static, internal fx_rates table in the database as the source of truth
. All
currency conversion quotes read a pre-configured rate (with a possible markup) from this table. This
approach keeps the system simple and deterministic for launch – rates can be updated by ops as needed,
but aren’t fetched from an external API in real-time. Each FX Quote record stores the rate (and fee) used at
quote time, so the conversion is transparent and locked in for that transaction
. In the future, we may
integrate live FX feeds or public APIs for dynamic rates, but that adds complexity (scheduling updates,
handling API downtime, ensuring regulatory compliance on rate sources) that we deferred for MVP.
Q: How are we handling pagination and search in transaction history for MVP?
A: Regular users will page through their own transaction history using cursor-based pagination (fetching,
say, 10–20 transactions at a time). For admin/staff use, we provide a basic search API with filters
 (as
described in the Admin API). It returns a bounded set (up to ~100 results) for a given query without server-
side pagination, so admins can refine queries if needed. We opted not to build advanced text search or
endless pagination in MVP
. The rationale is that support queries will be targeted – e.g. looking up a
specific transaction or user – and we can keep the implementation simple. If more sophisticated search or
bulk export is needed in practice, we will iterate on the admin tools post-MVP.
Q: What is the retention period for idempotency keys, and why?
A: We set idempotency record retention to  48 hours in MVP
. In practice, this means the system
“remembers” a POST request (by its Idempotency-Key) for two days. We picked 48h as it’s a common
industry practice – it comfortably covers typical client retry scenarios (including users retrying the next day if
they had network issues)
. It’s also short enough that we won’t accumulate unbounded data; stale
records get purged after 2 days. Some systems use 24h, but we chose a slightly longer window to be safe
(accounting for time zone differences or weekend delays). The trade-off is minimal since storing a couple
days of idempotency entries isn’t heavy, and it significantly reduces the risk of duplicate processing. We
documented this decision and will ensure a cleanup job or TTL mechanism removes expired entries
.
- 82
9
83
84
85
86
87
88
11
