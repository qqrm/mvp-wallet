# 4. DOMAIN MODEL (MVP)

4.1 User
Fields (minimum):
- phone_number (unique, primary identifier in MVP)
- user_id (optional stable internal identifier; may be equal to phone_number in MVP)
- email (optional)
- full_name (optional)
- status: ACTIVE | BLOCKED
- created_at

Notes:
- External SSO / Uzum Identity integration is out of scope for MVP.

4.2 Account
Each user has 1 account per currency in MVP.
Fields:
- account_id
- phone_number (owner)
- currency (ISO code) — MVP: UZS, USD
- status: ACTIVE | CLOSED
- created_at

Rules:
- Uniqueness: (phone_number, currency) MUST be unique (no duplicate accounts per currency).
- Create semantics (admin): if an account for (phone_number, currency) already exists, create MUST return the existing account (dedupe).
- CLOSED account cannot be used for new postings.

4.3 Transaction (JournalTx)
Fields:
- tx_id
- tx_type: TRANSFER | FX | FUND | WITHDRAW | REVERSE | SPEND_SIMULATION
- status: CREATED | POSTED | REVERSED | FAILED
- created_at, posted_at (optional)

Rules:
- POSTED is the final success state in MVP (do not use COMPLETED).
