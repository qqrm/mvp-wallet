# SOURCE_1__Blueprint_v3_1

## Page 18

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 18
- prop_idempotency_conflict: same key + different payload returns 409 and no side effects.
- prop_reverse_is_compensating: reversing a tx yields net zero delta across affected accounts.
Integration vs unit tests
- Target ratio: ~60% integration (DB + HTTP) / 40% unit (pure business rules).
- Unit tests: rounding, fee calc, quote expiry logic, validation, state machine transitions.
- Integration tests: DB constraints, locking behavior, idempotency storage, full posting atomicity.
Load/performance tests (minimal MVP plan)
- load_post_transfers_steady: 50 rps transfers for 10 min; p95 < 200ms; no invariant violations.
- load_fx_quote_execute_burst: 20 rps quote + execute; ensure no duplicate postings.
- db_lock_contention_test: concurrent debits on same account -> exactly one succeeds; others fail deterministically.
Security tests (practical checklist)
- auth_required_all_endpoints: 401 without token; 403 for admin endpoints without ADMIN role.
- rate_limit_transfer_abuse: repeated calls hit 429 with stable error code.
- input_validation_fuzz: malformed JSON, negative amounts, overflow values -> 400 VALIDATION_ERROR.
- idempotency_key_enforcement: missing key on financial POST -> 400 VALIDATION_ERROR.
- audit_log_written: every admin action produces AuditEvent with correlation_id.
Monitoring/alerting (metrics + SLO hints)
- SLI: posting_success_rate, transfer_p95_latency, fx_execute_p95_latency.
- Errors: insufficient_funds_rate, idempotency_conflict_rate, db_serialization_fail_rate.
- Audit integrity: audit_event_write_failures must be 0 (P0 incident if non-zero).
- SLO hint: 99.9% of postings succeed without manual intervention; p95 < 300ms in-region.
