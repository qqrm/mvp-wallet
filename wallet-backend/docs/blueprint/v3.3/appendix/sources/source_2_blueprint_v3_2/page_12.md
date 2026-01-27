# SOURCE_2__Blueprint_v3_2

## Page 12

Q: Do our transaction receipts contain all the information users (and ops) need?
A: Yes – we made sure to expand the receipt schema for completeness. Each transaction receipt clearly
shows which account was debited and which was credited (with user-friendly identifiers like names, phone
or email for the other party, when available)
. It lists the exact amounts in each currency, any fees
applied, and exchange rate info if relevant. We also include a human-readable description or memo (e.g. a
note the user entered, or a merchant name for a spend) so the context is clear
. Importantly, we hide
internal implementation details that aren’t meaningful to users – for example, raw account IDs are not
shown (perhaps just the last few digits for reference), focusing instead on labels the user recognizes
.
This decision closed a gap in earlier specs and ensures front-end and back-end are aligned on what’s
displayed. It makes the receipt the single source of truth for confirming a transaction, and it’s formatted
such that it can be shared or exported (e.g. as a PDF) if needed
.
Q: Why are we not including KYC or external bank/card linkages in MVP?
A: Given the aggressive one-month timeline and the focus on building core wallet functionality first, we
deliberately  kept  KYC  onboarding and  external  payment  rails out  of  scope  for  MVP
.  These
features  (connecting  to  bank  cards,  allowing  cash-out  to  banks,  verifying  customer  identities)  involve
significant additional complexity – both technically and in compliance. Instead, for MVP we constrained
usage to internal transfers among already-onboarded Uzum users and simulated cash-in/out via admin
tools. This allowed us to deliver a usable product faster. Before we enable real external money movement,
we will integrate Uzum Bank’s KYC processes and likely partner with card networks or payment systems,
which requires more development and regulatory approvals. Those are high-priority next steps after MVP,
but including them from the start would have jeopardized the MVP timeline.
Q: Will admin operations have a 4-eyes (dual approval) control in MVP?
A: Not in MVP. At launch, any single authorized admin can perform a sensitive operation (like funding,
withdrawing, or reversing funds) directly, and it will execute immediately. We chose to rely on strict audit
logging (recording who did what) as the control for MVP
. The reasoning is that implementing a maker-
checker approval workflow would add a lot of complexity – in UI design, state management, and policy –
and given the low volume and trusted team at startup, the audit trail was deemed sufficient for now. In the
future, as transaction volume and the ops team grow, we plan to introduce dual-approval for certain high-
risk actions (for example, very large transfers or reversals)
. This is already noted on our roadmap. It’s a
classic trade-off: speed of development vs. strict controls, and for MVP we optimized for speed while
planning to tighten controls soon after.
Persistence Strategy
The wallet service uses a single relational database for persistence. PostgreSQL is required for production
deployments due to its robustness in handling transactional updates and ensuring data integrity
.
During development and automated testing, a lightweight SQLite database may be used for convenience,
but with the understanding that we will migrate to PostgreSQL in staging/production. All schema changes
are managed via migrations to keep dev and prod in sync.
Regular backups and point-in-time recovery (PITR) will be configured on the PostgreSQL instance to protect
against data loss. Given the financial nature of the data, the database is treated as the source of truth – no
ephemeral in-memory balances are kept outside of it, simplifying consistency. We also designed the schema
for future scalability (partitioning or sharding can be introduced later if volumes grow, but MVP will run on a
89
90
91
92
35
4
93
28
94
3
12
