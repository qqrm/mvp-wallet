# SOURCE_2__Blueprint_v3_2

## Page 10

UI Personalization: Offer features like Dark Mode, custom themes, and quick action shortcuts for
common tasks
. While lower priority, these polish the user experience and meet modern user
expectations. They can be added incrementally once core features are stable.
4. Wallet Capabilities Expansion:
Unified Loyalty & Rewards: Integrate Uzum ecosystem loyalty programs (e.g. Uzum Market
cashback or points) into the wallet
. For example, users could earn points or cashback for using
Uzum Wallet or shopping in the marketplace, visible in the wallet. This encourages usage across
Uzum’s services. Implementing this may involve a points ledger or tracking rewards as a separate
balance/currency.
Installments & Pay-Later (BNPL): Embed Uzum’s “Nasiya” installment plan or similar buy-now-pay-
later features directly into wallet payments
. At checkout, users could split a purchase into
installments or get a micro-loan in one tap. Many competitors have had success with in-app credit
options (e.g. Kaspi’s BNPL). This would leverage Uzum Bank’s lending capabilities and require linking
transaction flows with credit approvals.
Shared/Family Wallets: Support joint accounts or family wallets where multiple users can share
access to funds
. For example, a couple could have a shared wallet, or a parent could oversee a
teen’s sub-account. This requires adding wallet group management and permission controls, which
have been anticipated by reserving wallet_group_id  in the schema. It’s a medium priority
feature that could differentiate Uzum Wallet in the region.
Expanded Multi-Currency: Beyond UZS and USD, consider adding more currency accounts (EUR,
RUB, etc.) if user demand arises
. Features like real-time FX rate alerts, the ability to hold multiple
currencies and exchange at user-chosen times (like Revolut’s model) could attract users with
international needs. This positions the wallet closer to a multi-currency finance app for travelers or
savers.
Merchant & SME Features: Develop a version of the wallet for small business users. This could
include quick onboarding for merchants, the ability to receive customer payments (potentially via
the QR system above), payout to suppliers or employees, and tools like transaction reports or
analytics
. Integrating with Uzum Market’s seller platform to instantly pay out sales to the wallet
is one idea. This taps into B2B use cases and would likely require additional compliance (business
KYC) and feature complexity (medium-term priority).
Third-Party Integrations: In the longer term, integrate other financial services like insurance and
investments through the wallet interface
. For example, offer an insurance marketplace (users
can buy travel or phone insurance via the app) or simple investment products (gold, bonds, etc.).
This follows the “super-app” model seen in some fintechs (Revolut, N26) and can provide new
revenue streams. These are low priority until core payments are solid, but on the strategic roadmap.
5. Ops & Support Improvements:
Admin Tools & Dashboard: Build a web dashboard for operations and support teams
. This
would allow non-engineers to view statistics, search transactions with more advanced filters (amount
ranges, text queries on memos), and export data (e.g. CSV of transactions) for reconciliation. While
MVP relies on direct API use, a friendly UI for ops will be important as volume grows.
Monitoring & Alerts: Implement internal monitoring dashboards and alerts for abnormal
conditions
. For example, alert if there’s a spike in failed transactions, or if a service outage
occurs. Some of this can be handled by existing APM tools (using the metrics we already collect), but
building custom alerts (or integrating with fraud systems) ensures issues are caught quickly. In
- 73
- 74
- 75
- 76
- 77
- 78
- 79
- 80
- 81
10
