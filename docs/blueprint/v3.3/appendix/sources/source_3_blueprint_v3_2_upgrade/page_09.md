# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 09

means all conversion quotes come from a controlled set of rates (which can be updated by ops as
needed, e.g., daily or when significant). The rationale is to minimize external dependencies and
unpredictability during MVP; using a fixed table with maybe a small markup ensures we have full
control and auditability of the rates given to users. The FxQuote mechanism captures the rate used
at the time of quote, so even if rates change later, each quote/transaction is self-contained. In the
future, we plan to integrate with public FX APIs or a live feed for real-time rates, but that introduces
complexity (refresh intervals, error handling, compliance with central bank rates) that is deferred.
MVP’s static table approach is simple, deterministic, and sufficient for initial internal usage.  (Note:
transparency is maintained by showing the rate and fee on the conversion receipt.)
Q: How are we handling pagination and search in transaction history for MVP?
A: Regular users will fetch their transaction history via cursor-based pagination (page through
transactions 10–100 at a time). For admin/staff searching, we provide basic filter-based querying ( /
v1/admin/search ) that returns a bounded result set (max ~100 records) for a given query. We
decided  not to implement advanced server-side pagination or full-text search in MVP. Instead,
admins can narrow results by specifying a user or time range, etc., such that the results are
manageable. The reasoning is that building a sophisticated pagination or search index was not
critical for the initial release – operationally, support queries are expected to be targeted (specific
user or transaction investigations). This keeps the implementation simple. If in practice more is
needed, we will add cursor-based pagination to admin search in a future iteration. For now, the
front-end can fetch and then allow the admin to refine or load more if needed.
Q: What is the retention period for idempotency keys and why?
A: We set the idempotency record retention to  48 hours for MVP. This means the system will
remember a request (by Idempotency-Key) for two days. We chose 48h as a standard in industry – it
comfortably covers typical client retry scenarios (including overnight or poor network conditions
where a user might retry the next day), but it’s short enough to not worry about unbounded data
growth. Some systems use 24h; we opted for 48h just to be safe with time zone differences and
potential weekend gaps. There’s minimal downside since the storage overhead for 2 days of keys is
manageable, and stale records will be purged. This decision was made to balance reliability and
cleanup – we documented it to ensure the ops team sets up a job or uses a TTL column to remove
expired idempotency entries.
Q: Do our transaction receipts contain all the information users (and ops) need?
A: Yes – we reviewed and expanded the receipt schema to ensure completeness. Each receipt clearly
shows which account was debited and which was credited (with identifiers like phone/email or
names when available for counterparties), the exact amount in each currency, and any fees or
exchange  rates  applied
.  We  include  a  human-readable  description  (memo  or  merchant
name) if provided, so the user knows the context. We deliberately hide internal fields like account_ids
that are not meaningful to the user (except perhaps the last few digits for reference), focusing on
useful info such as “From: Your UZS Wallet, To: Alice (+99890... phone), Amount: 50,000 UZS”. This
decision came from identifying a gap – earlier specs didn’t spell out receipt content, which could lead
to inconsistent implementation. Now, by defining it, we ensure the front-end and back-end agree on
what’s shown, and internal reconciliation is still possible (since ops can cross-reference tx_id).
Q: Why are we not including KYC or external bank/card linkages in MVP?
A: Given the aggressive one-month MVP timeline, we kept KYC and external payment rails out of
-
-
- 43
26
- 9
