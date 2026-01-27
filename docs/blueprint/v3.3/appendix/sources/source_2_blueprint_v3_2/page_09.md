# SOURCE_2__Blueprint_v3_2

## Page 09

national standard (or Uzum’s own QR format) and tying into Uzum Bank’s acquiring business (Uzum
Pay). High priority, as QR payments are popular in the region and enable offline acceptance.
Bills & Utilities Integration: Let users pay utility bills, top-up mobile airtime, taxes, and other
common bills directly from the wallet
. This requires integrations with biller aggregators or
relevant APIs. Most competitor wallets offer extensive bill-pay options, driving frequent
engagement. Uzum Bank already supports some bill payments on web; bringing them into the app
would increase usage.
Virtual & Physical Cards: Offer wallet-linked payment cards. Users could get a virtual card instantly
(for online payments or to add to Apple/Google Pay) and optionally order a physical Visa/Mastercard
. Card controls (set PIN, freeze/unfreeze, spending limits) would be provided. This extends wallet
usage to any POS or online merchant and leverages Uzum Bank’s card issuance capabilities (noting
Uzum has begun issuing cards). High priority for parity with modern wallets.
2. Compliance & Security:
KYC & Identity Verification: Implement tiered Know-Your-Customer verification for wallet users
before enabling external money flows or higher limits. This could integrate digital ID checks,
document upload, or bank eKYC services
. MVP defers full KYC, but it will be mandatory later to
comply with regulations when broadening access.
Automated AML Transaction Monitoring: As volume grows, deploy rules or machine learning to
detect suspicious activities
. For example, flag rapid in/out transfers, large transactions, or
known-risk accounts. These could feed alerts to compliance officers. This enhancement becomes
crucial once external transfers are allowed and volumes increase (medium-high priority post-MVP).
Advanced Login Security: Enhance user authentication with features like biometric login
(fingerprint/FaceID) and device management
. This improves security and UX. Also consider
adaptive authentication (step-up verification for high-risk actions) and session management tools
(viewing active sessions, remote logout). These are standard in leading fintech apps and would boost
user trust.
Fraud Prevention Tools: Add safeguards such as transaction OTP confirmations for large payments,
velocity limits, and integration with any national anti-fraud systems
. Longer-term, incorporate AI-
based fraud detection to analyze user behavior and flag anomalies (as some global apps do). These
features are planned as the platform scales to protect users and the business.
3. User Experience (UX) Enhancements:
Saved Recipients & Contacts: Allow users to save frequent recipients and import contacts to find
other Uzum Wallet users easily
. Instead of entering details each time, users can simply select a
saved name. This speeds up P2P transfers and leverages social connections (future versions might
sync phone contacts to suggest friends who have the wallet
).
Enhanced History & Insights: Provide richer transaction history features in the app. For example,
add filters (by date, type, amount) and search in the user’s transaction list
. Additionally, personal
finance insights like categorizing spending, monthly summaries, or budgeting tools can increase
engagement. MVP keeps history basic, but these can differentiate the app later.
Multi-Language Support: Localize the app UI and receipts into multiple languages (Uzbek, Russian,
English, etc.)
. As Uzum’s user base is multilingual, providing content in users’ preferred language
improves accessibility. This involves translating static text and possibly supporting different currency
formats/symbols.
- 63
- 64
- 65
- 66
- 67
- 68
- 69
70
- 71
- 72
9
