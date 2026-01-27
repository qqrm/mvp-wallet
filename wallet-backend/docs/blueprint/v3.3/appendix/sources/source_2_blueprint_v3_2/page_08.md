# SOURCE_2__Blueprint_v3_2

## Page 08

profiled to meet sub-200ms response times at moderate throughput
. As usage grows, vertical
scaling of the DB and read replicas for history queries can be employed, and the service can scale
horizontally behind a load balancer if needed (stateless API nodes relying on the shared DB).
Testing Strategy
The implementation will include comprehensive automated tests:
Unit Tests & Property Tests: Core business logic (ledger postings, balance updates, idempotency
handling) is covered by unit tests. Property-based tests validate key invariants (e.g. no net money
creation, idempotent replays yield same result)
.
Integration Tests: The full stack is tested with in-memory or test DB to ensure end-to-end flows
work (e.g. funding -> transfer -> balances update -> receipt correctness)
. Each API endpoint has
contract tests to verify request/response schemas match the OpenAPI specification.
Scenario E2E Tests: Critical user journeys are exercised in sequence. For example: admin funds an
account, user transfers funds to another user, user executes an FX conversion, etc., checking final
balances and receipts at each step
. Error scenarios (insufficient funds, invalid inputs, duplicate
requests) are also tested to ensure proper error codes and no side effects.
Performance Testing: Basic load tests simulate concurrent usage (e.g. sustained transfers per
second, bursts of FX quotes) to verify the system maintains acceptable latency and does not violate
consistency. The goal is p95 latency under 200ms for typical operations under expected MVP load
. Any deadlock or serialization retry issues observed in testing will be tuned (e.g. adjusting
indexing or lock granularity).
MVP Delivery Plan
Week 1: Finalize the API contracts (OpenAPI spec) and database schema. Implement the core ledger
posting engine (journal entries, account balance updates) and the idempotency middleware
.
Week 2: Implement all admin-facing APIs (account create/close, fund, withdraw, reverse) and user-
facing P2P transfer endpoints. Build the transaction history endpoint and basic transaction receipt
model
.
Week 3: Implement the FX conversion flow (quote and execute) and the sandbox spend simulation.
Add observability hooks (logging, metrics) and ensure audit logging is capturing all events.
Week 4: Integrate the backend with the mobile app UI (using Uzum UI components). Conduct end-
to-end testing of all user flows and perform bug fixes and refinements. Prepare deployment and
security review.
Post-MVP Feature Roadmap
1. Payments & Rails Enhancements:
External P2P by Phone/Card: Expand peer-to-peer transfers beyond the internal wallet. Allow
sending money to any user via phone number or even directly to a card number, integrating with
local instant payment networks (e.g. Uzcard/Humo)
. This would let users send money to people
not yet on Uzum (or to any card), greatly increasing reach and utility.
QR Code Merchant Payments: Introduce the ability to pay merchants via QR codes. Users can scan
a merchant’s QR in-store to transfer payment from their wallet
. This likely involves supporting a
55
- 56
- 57
- 58
- 55
- 59
- 60
-
-
- 61
- 62
8
