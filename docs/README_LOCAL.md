# Local development (MVP)

This doc is about fast local development/testing (without waiting for GitHub Actions).

## Backend (Rust)

Run from repo root.

Linux/macOS:

- Dev mode (no auth + seed demo data):
  - `WALLET_DEV_NO_AUTH=1 WALLET_DEV_SEED=1 cargo run --manifest-path wallet-backend/Cargo.toml -p wallet-api`

Windows (PowerShell):

```powershell
$env:WALLET_DEV_NO_AUTH = "1"
$env:WALLET_DEV_SEED = "1"

cargo run --manifest-path wallet-backend/Cargo.toml -p wallet-api
```

Health check:
- `http://127.0.0.1:3000/health`

Swagger UI:
- `http://127.0.0.1:3000/swagger-ui`

Demo state reset:

- `WALLET_DEV_SEED=1` switches the DB file to `wallet-dev.db`, deletes it on each startup, runs migrations, and seeds deterministic demo data.
- Without `WALLET_DEV_SEED`, the backend uses `wallet.db` and preserves state across restarts.

Dev-no-auth behavior (`WALLET_DEV_NO_AUTH=1`):

- `/v1/wallet/{user_id}/*` endpoints use the `{user_id}` path segment as the authenticated user.
  - If `X-Dev-User` (or `?as=`) is provided and does not match the path user, the request returns `400` with a clear mismatch error.
- User-scoped endpoints without a `user_id` path (e.g. `/v1/profile`, `/v1/accounts`) accept `X-Dev-User` or `?as=`. If absent, they default to `u01`.
- Account-scoped endpoints (e.g. `/v1/accounts/{account_id}`) derive the user from the account owner. If `X-Dev-User` is provided and mismatches the owner, the request returns `400`.
- `/v1/admin/*` and `/v1/dev/*` are treated as Admin automatically.

## Frontend (Vue)

Run from repo root.

- Install deps (lockfile):
  - `npm --prefix wallet-web ci`
- Dev server:
  - `npm --prefix wallet-web run dev`

The web app expects the backend at `http://127.0.0.1:3000`.

Dev endpoints for user/account discovery (for UI combobox/switcher):

- `GET /v1/dev/users`
- `GET /v1/dev/users/{user_id}/accounts`

In dev-no-auth mode (`WALLET_DEV_NO_AUTH=1`) these endpoints are available without a token and the handlers are active. Outside dev mode they return `404` even for admin users.

## Local checks (no `just`)

Rust (run from repo root):

- `cargo fmt --manifest-path wallet-backend/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path wallet-backend/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo test --manifest-path wallet-backend/Cargo.toml`
- `cargo build --manifest-path wallet-backend/Cargo.toml -p wallet-api`

Vue (run from repo root):

- `npm --prefix wallet-web ci`
- `npm --prefix wallet-web run typecheck`
- `npm --prefix wallet-web run build`
