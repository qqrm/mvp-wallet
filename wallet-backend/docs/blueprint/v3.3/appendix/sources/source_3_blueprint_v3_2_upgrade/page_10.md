# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 10

scope
. This was a conscious decision: implementing eKYC or integrating with card networks/
banks would introduce significant regulatory and technical overhead (identity verification flows,
compliance checks, external API integrations, etc.) which are not feasible in the MVP timeframe.
Instead, MVP operates in a closed environment (Uzum ecosystem users only, money stays inside).
We assume users are either pre-verified by Uzum or this is a pilot with limited users. We will address
KYC in the post-MVP phase – likely leveraging Uzum Bank’s existing processes – before any broader
rollout or any ability to cash in/out. External top-ups/withdrawals (cards, bank account transfers)
likewise require partnerships and testing (e.g. integration with Uzcard/Humo or Visa). These are high
priority on the roadmap, but for MVP the decision was to simulate them via admin operations (mint/
burn) and focus on perfecting the internal ledger and UX first.
Q: Will admin operations have a 4-eyes (dual approval) control in MVP?
A: Not in MVP – admin actions (like fund, withdraw, reverse) will execute immediately with a single
admin’s authorization, but everything is logged with who did it. We chose to delay multi-approval
workflows until post-MVP
. The reasoning: implementing an approval queue or maker-checker
system adds complexity in UI, state management, and policy – and for an MVP with likely low volume
and trusted ops users, the audit trail was deemed sufficient control. In the future (when transaction
volume and risk grow), we plan to introduce a dual-approval for certain sensitive actions (e.g. large
fund/reversal operations) as a defense against mistakes or internal fraud. This is noted in the
backlog.
(These Q&A items serve to document why certain features are designed the way they are in MVP v3.2, providing
context for future readers or decision-makers.)
Post-MVP Feature Roadmap
Beyond the MVP, we have identified a set of features to expand the Uzum Wallet’s capabilities, drawn from a
comparative analysis of leading wallet apps in relevant markets
. These are grouped into categories
and prioritized (High = near-term must-haves, Medium = strategic differentiators, Low = long-term nice-to-
have). They were chosen based on competitor offerings and Uzum’s ecosystem strategy, and will guide the
next phases of development:
1. Payments & Rails Enhancements:
- Phone Number & Card Number Transfers: Enable seamless P2P payments to users outside the immediate
wallet system via phone number or even card number links. This includes integration with local instant
payment networks (e.g. Uzcard/Humo in Uzbekistan) to allow sending money to any card or phone-linked
account, not just within Uzum
. This would extend the current internal-only transfers to a more open
network, greatly increasing utility and network effects.
-  QR Code Merchant Payments: Introduce the ability to pay merchants via QR codes. Users can scan a
merchant’s QR from the app to transfer payment. This requires building a merchant-present QR scheme or
supporting a national standard QR if one exists
. High priority, as QR payments are popular in CIS super-
apps and would allow Uzum Wallet to be used for in-store purchases, tying into Uzum Bank’s acquiring
business (Uzum Pay).
- Bills, Utilities & Mobile Top-ups: Integrate common bill payment services (utility bills, phone airtime top-ups,
taxes, etc.) directly into the wallet
. Most regional wallets (Kaspi, Qiwi, etc.) offer extensive bill pay
options. Uzum Bank already has some of these on web; bringing them into the wallet app would drive
frequent engagement. This likely involves connecting to biller aggregators or APIs for each service (post-
5
- 4
44
45
3
46
47
10
