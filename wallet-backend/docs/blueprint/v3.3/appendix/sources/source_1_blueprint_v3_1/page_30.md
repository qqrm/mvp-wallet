# SOURCE_1__Blueprint_v3_1

## Page 30

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 30
(9) OPEN QUESTIONS (DECISION LIST)
Q: What legal entity operates the wallet ledger (bank vs payment org vs internal feature)?
Why it matters: Determines licensing/compliance scope and whether external rails can be added.
What changes depending on answer: Affects auth model, regulator reporting, and future integrations.
Default assumption: Default: treat as internal feature under Uzum ecosystem licensed entity; external rails out of MVP.
Q: Which currencies are in MVP (UZS only vs UZS+USD vs more)?
Why it matters: FX complexity and rounding/fee logic depends on supported pairs.
What changes depending on answer: Affects UI (account list) and reconciliation scripts.
Default assumption: Default: UZS + USD only.
Q: FX rate source: Central Bank reference, partner provider, or fixed config?
Why it matters: Incorrect rate source causes revenue leakage or user disputes.
What changes depending on answer: Affects quote validity, refresh frequency, and transparency rules.
Default assumption: Default: fixed configurable rate table + markup for MVP; replace later.
Q: Is FX fee a flat fee, percentage, or markup-in-rate?
Why it matters: Changes receipts, product pricing, and ledger postings.
What changes depending on answer: Affects rounding and user-visible breakdown.
Default assumption: Default: small % fee posted to SYSTEM_FX_FEE.
Q: Retention policy for journal + audit logs (months/years)?
Why it matters: Impacts storage costs and compliance posture.
What changes depending on answer: Affects DB partitioning and backup strategy.
Default assumption: Default: financial journal >= 5 years; audit_event >= 2 years (policy confirmation needed).
Q: Admin operations governance: 1-person authority vs 4-eyes approval?
Why it matters: Insider risk and operational safety in fintech.
