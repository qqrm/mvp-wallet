# 11. SECURITY BASELINES (MVP)

- JWT auth required for all endpoints.
- Role-based authorization for admin endpoints.
- Per-user and per-IP throttling for POST money endpoints.
- Secrets MUST be stored in KMS/Vault in prod.
- Env vars allowed only in local dev.
