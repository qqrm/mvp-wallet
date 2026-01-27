# 11. SECURITY BASELINES (MVP)

- JWT auth required for all endpoints.
- Identity provider (e.g., Uzum SSO) is OUT OF SCOPE for MVP.
- MVP assumes `sub` claim identifies the user as `phone_number` (string).
- Role-based authorization for admin endpoints.
- Per-user and per-IP throttling for POST money endpoints.
- Secrets MUST be stored in KMS/Vault in prod.
- Env vars allowed only in local dev.
