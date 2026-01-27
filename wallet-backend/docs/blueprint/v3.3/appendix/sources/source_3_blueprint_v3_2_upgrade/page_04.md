# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 04

ledger balance. If in future “holds” are introduced (for card authorizations, etc.), the schema will adjust, but
MVP assumes no pending holds. Closing an account (status = CLOSED) prevents new debits/credits but does
not remove the record or its balance history.
- Tenant and Group Structure: MVP is single-tenant (all users under the same umbrella). The schema has a
placeholder for  tenant_id  on user/accounts for future multi-tenant support. Similarly, while MVP is
single-owner  per  account,  the  schema  reserves  a  wallet_group_id  field  to  support  shared/joint
wallets in  the  future
 (e.g.  family  accounts).  No  group  functionality  is  active  in  MVP  beyond  this
placeholder.
Balance Model & Transaction Lifecycle
Each account has three key balance metrics: available, blocked, and ledger. In MVP, blocked = 0  for all
accounts  (no  hold  functionality),  so  effectively  available  =  ledger  balance at  all  times
.  All
transactions affect balances atomically via the journal posting:
On a P2P transfer, the sender’s account ledger (and available) balance decreases by the transfer
amount, and the recipient’s account increases by the same amount. These debits and credits post in
one transaction so the ledger remains consistent.
On an FX conversion, two accounts of the same user (e.g. UZS and USD) are involved: the source
currency account is debited, and the target currency account is credited with the converted amount
(after applying rate and fee). Any FX fee is taken as a separate JournalEntry (crediting a fee revenue
account). The net debits (source + fee) equal the credit in target currency after conversion (adjusted
for rate)
.
Admin mint (fund) adds balance to a user’s account by debiting a special SYSTEM_MINT account
and crediting the user (this represents creating money in the system, backed by an external funding
outside MVP scope). Admin withdraw (burn) does the opposite: debits the user’s account and
credits a SYSTEM_SINK or treasury account (effectively removing money from circulation in the
wallet)
. These mimic deposit/withdrawal without external rail integration.
Reversal is a special admin operation to correct mistakes: it creates a new JournalTx that effectively
negates a target transaction’s effects. For example, if tx_id XYZ credited 100 UZS to A and debited
100 UZS from B, a reversal would debit A and credit B for 100 UZS, with a reference link to XYZ.
Reversals ensure even erroneous transactions remain in history (they are not deleted, just offset)
.
Spend Simulation is a test transaction where a user “spends” to a dummy merchant account
(SYSTEM_SPEND). It debits the user’s account and credits the SYSTEM_SPEND account, optionally
attaching a merchant_label (e.g. “Simulated spend at Uzum Market”) for UI display. This is purely for
QA/testing and user demonstration; it has no external effect but appears in history like a normal
debit
.
All these workflows follow the ledger invariants above and produce a transaction receipt accessible to the
user. The receipt includes the final balances (post-transaction) in each affected account, confirming the
updated available amounts. It also captures the essential details of the transaction (e.g. “You sent 50,000
UZS to +99890... (John) on Jan 10, 2026”, including the tx_id for reference).
12
13
11
-
- 14
- 15
- 16
- 17
4
