# SOURCE_2__Blueprint_v3_2

## Page 07

a separate e-money license at launch. All stored value remains within the regulated bank entity, and
external cash-out is simulated via admin operations.
Personal Data Law: User data handling aligns with Uzbekistan’s Law on Personal Data Protection.
All personal data is stored in approved jurisdictions (onshore as required) and protected per
regulation (consent records, etc.)
. Although MVP collects minimal PII (only phone/email), we
ensure proper data security and will register with authorities if needed.
AML/CFT: Even though full KYC is deferred, the system maintains auditability of all transactions to
support anti-money-laundering oversight
. Unusual activity can be detected via the audit logs.
Prior to enabling external transfers, Uzum will integrate formal KYC verification and AML transaction
monitoring rules (e.g., sanction list screening, suspicious pattern flags) to meet regulatory
requirements.
Consumer Protection: All transactions produce immutable receipts and audit trails to resolve
disputes. There are no automated dispute reversal workflows in MVP (issues are handled manually
by admins), but logs ensure accountability. Limits on transfer amounts and velocity can be imposed
by configuration if required by regulation.
Observability & Monitoring
Audit Logging: Every critical action is recorded in an audit_event  log (see AuditEvent entity) with
timestamp, actor, action type, and context
. This applies to all admin operations and financial
transactions. Audit logs are immutable and retained long-term (at least 2–5 years) for compliance
and forensic purposes.
Metrics & Monitoring: The system exposes basic operational metrics: e.g. transaction throughput,
latency percentiles (p95), error rates (insufficient_funds errors, idempotency conflicts, DB
serialization retries)
. These metrics can be collected and alerted on via the monitoring
infrastructure. Unusual events (like repeated failed transfers or rapid balance changes) can be
flagged for review (though MVP does not include an automated alerting module beyond what the
ops team sets up).
Tracing & Correlation: Each request carries a correlation_id and is traceable end-to-end. JournalTx
records and audit events include this correlation_id (and the initiating user/admin) to tie together log
entries for a single operation, simplifying troubleshooting across distributed components.
Reliability & Performance Patterns
Atomicity: All ledger updates (creating a JournalTx and its JournalEntry lines, updating balances)
occur within a single database transaction that either fully succeeds or rolls back
. This
guarantees that partial updates cannot occur (no stuck half-posted transfers).
Outbox for Integrations: The design includes a placeholder for an outbox table to record events
for external integrations (e.g. sending notifications or integrating with external payment networks).
MVP does not require external messaging, so the outbox is not actively used, but the schema is
prepared for future use (ensuring eventual consistency if integrating with other services)
.
Isolation Level: To maintain consistency under concurrent load, the database transaction isolation
level can be set to SERIALIZABLE or we use explicit row locks on all affected accounts during
updates
. This prevents race conditions like lost updates to balances. In testing, we simulate high
concurrency (e.g. 50 transfers/sec for 10 min) to ensure no invariant violations
.
Scalability: The MVP monolith is expected to handle initial load within a single service instance and
database. We identified no immediate performance bottlenecks: critical paths (transfers, FX) were
- 49
- 49
-
- 51
- 52
53
-
- 52
- 54
- 43
55
- 7
