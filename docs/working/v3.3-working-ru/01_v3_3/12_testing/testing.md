# 12. Тестирование

E2E (critical journeys):
- login → profile → balance
- admin create account → fund
- P2P success / insufficient funds
- spend simulate posts once
- FX quote+execute success / expired quote
- admin withdraw burn to sink
- reverse compensates
- cursor pagination history

Contract tests:
- OpenAPI schema validation
- error code stability
- pagination contract

Property-based:
- double-entry sum zero
- no negative available
- idempotency replay same result
- idempotency conflict no side effects
- reverse is compensating

См. первоисточник: [SOURCE 1 §p017](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p017)–[SOURCE 1 §p018](../../../../blueprint/v3.3/appendix/sources/source_1_blueprint_v3_1.md#p018).
