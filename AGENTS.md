# AGENTS.md — Repo Root

This file defines how the agent must operate for the entire repo.
It is not a product spec.

## Operating rules
- Keep PRs small and incremental; no architecture rewrites unless explicitly asked.
- Do not add TODOs or commented-out code.
- If any required gate fails, fix it before returning.

## Mandatory quality gates (must run locally and in CI)

### Rust (workspace in `wallet-backend/`, run from repo root)
1. Format check:
   - `cargo fmt --manifest-path wallet-backend/Cargo.toml --all -- --check`
2. Lints:
   - `cargo clippy --manifest-path wallet-backend/Cargo.toml --all-targets --all-features -- -D warnings`
3. Tests:
   - `cargo test --manifest-path wallet-backend/Cargo.toml`
4. Build:
   - `cargo build --manifest-path wallet-backend/Cargo.toml -p wallet-api`

### Vue (run from repo root)
1. Install (lockfile required):
   - `npm --prefix wallet-web ci`
2. Typecheck:
   - `npm --prefix wallet-web run typecheck`
3. Build:
   - `npm --prefix wallet-web run build`

## One-command gates (repo-level Justfile)
Use the root `Justfile` commands:
- `just lint`
- `just test`
- `just build`
- `just ci`

Notes:
- Web lint = typecheck only (`npm --prefix wallet-web run typecheck`).
- Web test = typecheck + build (`npm --prefix wallet-web run typecheck` and `npm --prefix wallet-web run build`).

## CI enforcement
CI must run the same gates as above. If CI is configured, it must run `just ci`
(or the explicit underlying commands) on both Windows and Ubuntu via a matrix.

## Required PR description format
Every PR description must include:
- Commands run + results (exact commands).
- Files changed.
- Any follow-ups / known limitations.
