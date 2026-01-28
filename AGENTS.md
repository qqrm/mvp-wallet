This file defines how an agent must work in this repository.

## Operating rules
- Keep PRs small and reviewable. No architecture rewrites unless explicitly asked.
- Do not add TODOs or commented-out code.
- If any required gate fails, fix it and re-run the failed gate(s) until green.
- Prefer deterministic, reproducible commands. Do not rely on local IDE state.

## Local quality gates (run locally before opening PR)
Run commands from the repo root.

### Rust (workspace: `wallet-backend/`)
Use direct `cargo` commands (no `just`).

1) Format check (must be clean):
- `cargo fmt --manifest-path wallet-backend/Cargo.toml --all -- --check`

If it fails, apply formatting and re-check:
- `cargo fmt --manifest-path wallet-backend/Cargo.toml --all`
- then re-run the `--check` command above.

2) Lints (no warnings):
- `cargo clippy --manifest-path wallet-backend/Cargo.toml --all-targets --all-features -- -D warnings`

3) Tests:
- `cargo test --manifest-path wallet-backend/Cargo.toml`

4) Build (at minimum the API crate):
- `cargo build --manifest-path wallet-backend/Cargo.toml -p wallet-api`

Optional (if you want faster feedback before the full suite):
- `cargo check --manifest-path wallet-backend/Cargo.toml --all-targets --all-features`

### Vue (project: `wallet-web/`)
Use direct `npm` commands (no `just`).

1) Install (must use lockfile):
- `npm --prefix wallet-web ci`

If it fails with “package.json and package-lock.json are not in sync”:
- run `npm --prefix wallet-web install`
- commit the updated `wallet-web/package-lock.json`
- then re-run `npm --prefix wallet-web ci`

2) Typecheck:
- `npm --prefix wallet-web run typecheck`

3) Build:
- `npm --prefix wallet-web run build`

These frontend gates catch Vite/esbuild transform errors (e.g., duplicate exports in `wallet-web/src/shared/api/endpoints.ts`).

## Required PR description format
Include:
- Exact commands run (copy/paste) + result summary.
- Files changed (high level).
- Any follow-ups / known limitations.
