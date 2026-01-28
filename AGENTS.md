# Agent instructions (repo-wide)

This file defines how an agent must work in this repository.

## Operating rules
- Keep PRs small and reviewable. No architecture rewrites unless explicitly asked.
- Do not add TODOs or commented-out code.
- If any required gate fails, fix it and re-run the failed gate(s) until green.
- Prefer deterministic, reproducible commands. Do not rely on local IDE state.
- Do not weaken auth/security checks unless explicitly requested. For local/demo convenience, prefer explicit DEV-only paths guarded by localhost checks.

## Local quality gates (run locally before opening PR)
Run commands from the repo root. Use direct tool commands (no `just`) unless the task explicitly asks for `just`.

### Rust (workspace: `wallet-backend/`)
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

Optional (faster feedback before the full suite):
- `cargo check --manifest-path wallet-backend/Cargo.toml --all-targets --all-features`

### Vue (project: `wallet-web/`)
These gates must catch build-time failures (e.g. Vite/esbuild transform errors, duplicate exports, etc.).

1) Install (must use lockfile):
- `npm --prefix wallet-web ci`

If it fails with “package.json and package-lock.json are not in sync”:
- run `npm --prefix wallet-web install`
- commit the updated `wallet-web/package-lock.json`
- then re-run `npm --prefix wallet-web ci`

2) Typecheck:
- `npm --prefix wallet-web run typecheck`

3) Build (must succeed; do not ignore Vite overlay errors):
- `npm --prefix wallet-web run build`

4) Optional if scripts exist (only run if present in `wallet-web/package.json`):
- `npm --prefix wallet-web run lint`
- `npm --prefix wallet-web run test`

## CI requirements
CI must run the same gates as above (at least: Vue `ci + typecheck + build`, Rust `fmt + clippy + test + build`).
Do not rely on developers running checks locally; CI is the source of truth.

## Required PR description format
Include:
- Exact commands run (copy/paste) + result summary.
- Files changed (high level).
- Any follow-ups / known limitations.
