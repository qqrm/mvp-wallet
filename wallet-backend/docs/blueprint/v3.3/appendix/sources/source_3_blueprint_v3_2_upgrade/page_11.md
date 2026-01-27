# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 11

MVP due to integration work).
- Virtual and Physical Cards Integration: Expand wallet by offering virtual debit cards instantly, and optional
physical  cards  linked  to  the  wallet  balance
.  Uzum  has  started  issuing  Visa  cards;  in-wallet  card
management (view card details, freeze/unfreeze card, set PIN) would be a key feature. This effectively turns
the wallet into a full payment account usable at any POS or online (via NFC payment or card number). It’s
high priority to reach parity with competitors like Revolut, which gained traction through easy card issuance
. Physical card support also ties in with Uzum’s omnichannel strategy (cards distributed at pickup points,
etc.).
2. Compliance & Security:
- KYC and Identity Verification: Implement a tiered KYC process for wallet users (especially if external money
flows are enabled). This could involve integrating with digital ID verification services, scanning passports,
etc., to comply with regulations before lifting certain limits. MVP deferred KYC
, but it will become
mandatory when expanding beyond closed-loop usage. Along with KYC, enforce any necessary user risk
scoring and sanction screening in partnership with compliance.
-  AML  Transaction  Monitoring: As  volume  grows,  introduce  automated  monitoring  for  suspicious
transactions. E.g., rules to flag rapid in/out transfers, large amounts, or blacklisted recipients. This may tie
into an AML software or custom rule engine. Alerts from this system would feed to compliance officers for
review. This is crucial once external payments and higher volumes come into play (medium to high priority
post-MVP).
-  Advanced Login Security: Add features like  biometric authentication (fingerprint/FaceID login support)
and robust device binding. While MVP uses basic JWT auth, adding biometrics improves security and user
convenience. Also consider adaptive authentication (prompt for re-auth for high-risk actions) and session
management (remote logout, device list) as seen in top fintech apps
.
- Fraud Prevention Tools: Beyond rate-limits, implement features such as transaction OTP confirmations for
large payments, AI-based fraud detection (monitoring user behavior), and possibly integration with national
anti-fraud systems. Competitors often highlight their real-time fraud alerts
; Uzum should plan similar
capabilities to build user trust as the platform scales.
3. User Experience (UX) Enhancements:
- Saved Recipients & Contact Integration: Allow users to save frequent recipients or import contacts, so that
sending money is as easy as selecting a name from an address book. MVP requires manual entry (or phone
lookup within our system), but future versions will let users mark certain recipients as favorites with
nicknames, and possibly sync contacts to find other Uzum Wallet users. This speeds up P2P transfers and
leverages social connections.
- Enhanced Transaction History & Insights: Provide richer filtering and search in the user’s transaction list (by
date range, by type – e.g. show only FX or only incoming transfers). Also, personal finance features like
spending categorization, monthly summaries, or budgeting tools could be added to increase engagement.
While MVP keeps history basic, these UX improvements can differentiate the app in later phases.
- Multi-language Support & Localization: As Uzum grows, ensure the app and receipts can be displayed in
multiple languages (Uzbek, Russian, English, etc.). This may involve i18n of currency formats and possibly
supporting different currency symbols in UI beyond UZS and USD.
- UI Personalization & Other UX: Features like dark mode, customizable app themes, or quick action shortcuts
for common tasks can be considered. These are lower priority but contribute to a polished user experience
expected from modern apps.
48
49
5
50
50
11
