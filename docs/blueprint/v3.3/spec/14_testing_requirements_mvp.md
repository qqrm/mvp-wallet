# 14. TESTING REQUIREMENTS (MVP)

14.1 Property-based (invariants)
- Double-entry sum is zero per transaction/currency.
- No negative available balance.
- No money created/destroyed unexpectedly.
- Idempotency replay returns the exact same result.
- Idempotency conflict (payload mismatch) has no side effects.
- Reverse operations are compensating and preserve invariants.

14.2 Contract tests
- OpenAPI schema validation in CI (schema == runtime).
- Error code stability.
- Pagination contract stability.

14.3 Integration / E2E critical journeys
- login → profile → balance
- admin create account → fund
- P2P success and insufficient funds
- spend simulate posts once
- FX quote + execute success and expired quote
- admin withdraw (burn to sink)
- reverse compensates
- cursor pagination for history
- receipt retrieval by tx_id for each flow

14.4 Load / concurrency (minimum)
- Demonstrate no deadlocks under concurrent transfers.
- Transfers steady load (example target: 50 rps for 10 minutes; p95 < 200ms) — tune per environment.
- FX burst scenario.
- Lock contention test.
