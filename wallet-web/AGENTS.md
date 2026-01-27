# AGENTS.md — Wallet Web

Scope: `wallet-web/`.

## Operating rules
- Keep changes minimal and reviewable.
- Do not add TODOs or commented-out code.

## Mandatory quality gates (run from repo root)
- `npm --prefix wallet-web ci`
- `npm --prefix wallet-web run typecheck`
- `npm --prefix wallet-web run build`
