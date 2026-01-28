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

- Current behavior: `wallet.db` persists across restarts.
  - To reset manually: stop the service and delete `wallet-backend/wallet.db` (also `wallet.db-wal` and `wallet.db-shm` if present).
- Target behavior (decision): on every start in demo mode the backend replaces the current DB with a template snapshot.
  - See `wallet-backend/docs/decisions/0003-demo-db-reset-on-startup.md`.

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

In dev-no-auth mode (`WALLET_DEV_NO_AUTH=1`) these endpoints are available without a token. Outside dev mode they require admin auth.

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
