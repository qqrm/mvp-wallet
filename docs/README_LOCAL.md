# Local development (MVP)

This doc is about fast local development/testing (without waiting for GitHub Actions).

## Backend (Rust)

Run from repo root.

Linux/macOS:

- Dev mode (seed demo data):
  - `WALLET_DEV_SEED=1 cargo run --manifest-path wallet-backend/Cargo.toml -p wallet-api`

Windows (PowerShell):

```powershell
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

Localhost auth behavior:

- Authentication is disabled on localhost (loopback IP or `Host: localhost`).
- Identity comes only from `X-Dev-User` or `?as=`; if absent it defaults to `u01`.
- `/v1/wallet/{user_id}/*` must match the selected user; mismatches return `400`.
- `/v1/admin/*` and `/v1/dev/*` are treated as Admin automatically.

## Frontend (Vue)

Run from repo root.

- Install deps (lockfile):
  - `npm --prefix wallet-web ci`
- Dev server:
  - `npm --prefix wallet-web run dev`

The web app expects the backend at `http://127.0.0.1:3000`.

User switching:

- `/wallet` defaults to `u01`.
- `/wallet?as=u02` loads `u02` and sends `X-Dev-User: u02` on all requests.

Dev endpoints for user/account discovery (for UI combobox/switcher):

- `GET /v1/dev/users`
- `GET /v1/dev/users/{user_id}/accounts`

These endpoints are available only on localhost. Outside localhost they return `404` even for admin users.

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
