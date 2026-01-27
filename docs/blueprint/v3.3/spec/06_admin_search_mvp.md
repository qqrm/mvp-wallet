# 6. ADMIN SEARCH (MVP)

Endpoint: GET /v1/admin/search

Filters (minimal, locked):
- user_id
- account_id
- tx_id
- currency
- time_range (from,to)

Response:
- up to 100 results per request
- server does not implement cursor pagination in MVP
- frontend can handle “load more” by shrinking/shift time_range
