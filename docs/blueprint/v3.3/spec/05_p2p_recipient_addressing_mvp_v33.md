# 5. P2P RECIPIENT ADDRESSING (MVP v3.3)

MVP MUST support recipient addressing by:
- to_account_id (direct)
- to_user_id (resolve)
- to_phone_number (resolve)

Resolution rule:
- if recipient provided via phone/user_id:
  - resolve recipient user
  - select recipient ACTIVE account in SAME currency as source transfer
  - if no matching account: return RECIPIENT_ACCOUNT_NOT_FOUND

Deadlock avoidance:
- Lock accounts in deterministic order (e.g., by (user_id, account_id) ascending)
  before balance checks and posting.
