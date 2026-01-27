# SOURCE_1__Blueprint_v3_1

## Page 28

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 28
(8) UZBEKISTAN COMPLIANCE RESEARCH (2025–2026)
This section lists the most relevant Uzbekistan regulations for a wallet/ledger product and translates them into concrete engineering controls. MVP scope has no
KYC and no external payment rails, but correctness, privacy, and auditability are mandatory.
Relevant regulators / frameworks
- Central Bank of the Republic of Uzbekistan (CBU): payments ecosystem oversight and regulatory framework references. [R4]
- LexUZ national legislation portal: authoritative legal texts for payments, personal data, AML/CFT, cybersecurity. [R1][R2][R3][R6]
- Uzbekistan personal data regime (operator duties, security and confidentiality obligations). [R2]
- Cybersecurity framework for Critical Information Infrastructure (KII) may apply depending on classification. [R6]
KYC/AML requirements baseline (MVP interpretation)
- AML/CFT law defines a compliance environment for financial transactions and related obligations for regulated institutions. [R3]
- MVP engineering requirement: audit-ready ledger (immutable journal, traceability, admin accountability) so KYC/AML can be added post-MVP without redesign.
- Add basic monitoring hooks: velocity limits, repeated failed transfers, suspicious admin actions; store flags in audit_event.
Data retention / privacy expectations (engineering view)
- Personal data must be processed lawfully with confidentiality and protection controls. [R2]
- Maintain a data inventory and minimize sensitive fields; do not store unnecessary documents/PII in MVP.
- Data localization rules and cross-border processing are evolving; keep an in-country deploy option and make cross-border a legal decision. [R2][R8]
Reporting / audit requirements (high-level)
- Immutable financial history + receipts support internal finance reconciliation and potential regulator audits.
- Admin actions must be fully attributable (who/when/what/why).
- Retain audit logs with defined retention window (suggestion: >= 5 years for financial journal; policy decision).
Requirement -> Engineering control
| Requirement | Engineering control (MVP) |
| --- | --- |
| Payments legal basis awareness | Keep MVP boundary as internal ledger feature; external rails only via licensed entity integration. [R1][R4] |
| Personal data protection | RBAC, access logs, encryption in transit, minimal PII, retention policy. [R2] |
