# SOURCE_1__Blueprint_v3_1

## Page 29

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 29
Cybersecurity baseline
Secure coding + TLS + rate limits; if classified as KII, apply PP-167 controls and assessment readiness. [R6]
AML/CFT readiness
Audit-ready ledger + monitoring hooks; support extraction of transaction reports. [R3]
Minimal compliance MVP checklist
- Scope statement in product/legal docs: MVP is internal wallet ledger; external rails and KYC are deferred.
- Security baseline: RBAC + audit logs for admin endpoints; secrets managed; TLS everywhere.
- Privacy baseline: data inventory + retention policy draft; least-privileged DB roles; access logs.
- Operational baseline: Postgres backups + PITR; restore drill executed; migrations gated in CI.
- Incident readiness: on-call owner; runbook for balance discrepancy investigation using tx_id receipts.
