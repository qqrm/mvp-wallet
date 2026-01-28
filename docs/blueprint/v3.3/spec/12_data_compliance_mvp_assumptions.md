# 12. DATA & COMPLIANCE (MVP ASSUMPTIONS)

MVP does not include KYC or external payment rails. Despite that, the MVP must be bank-grade in:
- ledger correctness
- data privacy
- auditability

Assumptions and constraints:
- Data locality: Uzbekistan requirements assumed; deployment must support on-shore hosting.
- Cross-border processing must be treated as a legal decision; keep the architecture portable.
- Long-term audit trail retention is required.
- AML/CFT context: even without KYC, provide auditability and monitoring hooks for future controls.
- Cybersecurity: treat the system as potentially subject to critical-infrastructure requirements.

Engineering controls required in MVP:
- RBAC with least privilege and access logging.
- TLS everywhere and secure secret management.
- PII minimization and explicit retention policies for PII and logs.
- PostgreSQL for staging/prod; backups + PITR; restore drill before launch; migrations gated in CI.
- A runbook for investigating balance discrepancies using tx_id receipts and audit events.
