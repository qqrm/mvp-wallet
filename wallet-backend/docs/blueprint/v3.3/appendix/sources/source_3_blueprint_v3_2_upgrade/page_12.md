# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 12

4. Wallet Capabilities Expansion:
- Unified Loyalty & Rewards: Integrate loyalty points or cashback rewards into the wallet
. Since Uzum
has  e-commerce  and  delivery  arms,  a  unified  loyalty  program  where  wallet  usage  (or  marketplace
purchases via wallet) yields points or cashback will incentivize usage. For example, users might get a
percentage back into their wallet for each Uzum Market purchase paid with the wallet. This requires
tracking rewards and possibly a points currency, but it’s a high-impact feature to increase stickiness.
- Installments & Pay-Later (BNPL): Embed Uzum’s Nasiya installment plan product into the wallet flows
.
This means enabling at-checkout financing: e.g., when a user is about to pay (especially for larger amounts,
or on marketplace purchases), offer “Pay in 4 installments” or micro-loan options. Many competitors (Kaspi,
Wildberries) have had huge success integrating such BNPL offerings directly
. This would require real-
time credit decisioning and an update to the ledger model (since an installment purchase might create a
schedule of future payments). It’s a major feature bridging wallet and lending product, targeted soon after
MVP (high priority, leveraging Uzum Bank’s existing consumer loan capabilities).
- Shared/Family Wallets: Introduce joint accounts or family wallet features
. This could allow two or more
users to share an account or have linked accounts (e.g., a parent-child arrangement with controlled access).
While  uncommon  in  CIS  wallets  currently,  it’s  a  differentiator  and  addresses  use  cases  like  couples
managing shared finances or parents giving pocket money on a kid’s card
. Implementing this will
require a notion of wallet groups and permissions (the groundwork of which we’ve reserved in the schema
). Medium priority – not immediate, but potentially Q2/Q3 roadmap if aiming to stand out regionally.
- Expanded Multi-Currency & Forex Tools: MVP supports UZS and USD. Going forward, we might add more
currencies (EUR, RUB, etc.) if market demand exists
. Additionally, features like holding multi-currency
balances with real-time FX updates, or setting target conversion rates (like a FX order/alert) could attract
users who have forex needs. This positions the wallet closer to a Revolut-like offering for travelers or savers.
-  Merchant & SME Wallet Features: Expand the platform to  business users. For example, provide an SME
version of the wallet for small merchants or marketplace sellers
. Features might include quick business
account onboarding, the ability to receive payments (possibly via the QR payments above), pay suppliers or
employees, and view sales analytics. Integrating with Uzum Market for instant payouts to sellers’ wallets is
one idea. This essentially merges wallet with basic business banking tools, capturing a new user segment
and increasing transaction volume in the ecosystem
. This is a natural extension once consumer
wallet features are solid, though it introduces regulatory considerations (business KYC) and complexity
(perhaps medium to long-term priority).
- Third-Party Integrations (Insurance, Investments): In the longer term, the wallet can evolve into a financial
super-app hub. This could include an insurance marketplace (letting users purchase insurance products,
e.g. travel or device insurance, through the app) and simple investment products (such as buying gold,
stocks, or bonds in fractional amounts)
. Global fintechs like N26 and Revolut have taken this route to
increase engagement and revenue. For Uzum, this would likely come after core payments and credit
features are in place (low priority for now). It would leverage partnerships or existing Uzum Bank offerings
to integrate seamlessly.
5. Ops & Support Improvements:
- Admin Tools & Dashboards: Develop a richer admin interface for operations. This includes web dashboards
to view aggregate stats, advanced search with filters beyond the basic API (e.g. by amount range or text
search in memos), and the ability to export data (e.g. CSV of transactions for reconciliation). While MVP
provides minimal API support, a friendly UI for ops can greatly improve efficiency in handling customer
issues.
-  Transaction Monitoring & Alerts: Implement internal tooling for ops to set alerts on abnormal system
conditions (e.g., a sudden spike in reversals or a service outage impacting ledger). Some of this overlaps
51
52
52
53
53
12
54
55
56
57
58
12
