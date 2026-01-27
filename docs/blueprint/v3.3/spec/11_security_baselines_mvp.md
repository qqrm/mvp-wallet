# 11. SECURITY BASELINES (MVP)

- JWT auth required for all endpoints.
- JWT issuer/provider details are out of scope for MVP (no Uzum SSO assumption).
  - Required minimum claim: sub = phone_number (or user_id that equals phone_number in MVP)
  - Admin access is determined by a role/permission claim (implementation-defined in MVP).
- Role-based authorization for admin endpoints.
- Per-user and per-IP throttling for POST money endpoints.
- Secrets MUST be stored in KMS/Vault in prod.
- Env vars allowed only in local dev.
