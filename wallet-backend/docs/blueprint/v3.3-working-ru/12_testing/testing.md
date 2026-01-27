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

См. первоисточник: `00_source_pdf_text/page_017.md`–`page_018.md`.
