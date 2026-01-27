# Uzum Wallet (MVP) — monorepo

Repo layout:
- `wallet-backend/` — Rust backend (Cargo workspace)
- `wallet-web/` — Vue 3 frontend (Vite)

## Quick start (dev)

### With Docker Compose (API + Web)
1) Copy env example:
   - `cp .env.example .env` (or create `.env` manually)
2) Run:
   - `docker compose up --build`

Services:
- API: http://localhost:3000
- Web: http://localhost:5173

Auth (MVP):
- Admin: `Authorization: Bearer <ADMIN_TOKEN>` (default: `admin-dev-token`)
- User: `Authorization: Bearer user:<user_id>:<sig>` (see backend docs, or paste a token in the UI)

## Notes
- Dev DB is SQLite by default, stored in the `api-data` docker volume.
- For browser-to-API calls, `wallet-web` uses Vite proxy for `/v1/*` so CORS is not required in dev.
