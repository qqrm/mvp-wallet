# Justfile (root) — fixed: dev/web-dev always self-initialize web deps

set dotenv-load := true
set shell := ["bash", "-lc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

BACKEND_DIR := "wallet-backend"
WEB_DIR := "wallet-web"
API_PKG := "wallet-api"

default: help

help:
  @just --list

# --------------------
# Frontend
# --------------------

# Installs web deps if missing (or if WALLET_WEB_INSTALL=1).
# Strategy:
# - if package-lock.json exists -> try npm ci, fallback to npm install
# - else -> npm install
web-install:
  powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File scripts/web-install.ps1

web-build: web-install
  npm --prefix {{WEB_DIR}} run build

web-test: web-install
  npm --prefix {{WEB_DIR}} run typecheck
  npm --prefix {{WEB_DIR}} run build

web-lint: web-install
  npm --prefix {{WEB_DIR}} run typecheck

web-dev: web-install
  npm --prefix {{WEB_DIR}} run dev

clean-web:
  {{ if os() == "windows" { "just clean-web-win" } else { "just clean-web-unix" } }}

clean-web-win:
  Remove-Item -Recurse -Force -ErrorAction SilentlyContinue wallet-web/node_modules, wallet-web/dist

clean-web-unix:
  rm -rf wallet-web/node_modules wallet-web/dist

# --------------------
# Backend
# --------------------

backend-build:
  cargo build --manifest-path {{BACKEND_DIR}}/Cargo.toml -p {{API_PKG}}

backend-test:
  cargo test --manifest-path {{BACKEND_DIR}}/Cargo.toml

backend-fmt-check:
  cargo fmt --manifest-path {{BACKEND_DIR}}/Cargo.toml --all -- --check

backend-clippy:
  cargo clippy --manifest-path {{BACKEND_DIR}}/Cargo.toml --all-targets --all-features -- -D warnings

backend-run:
  cargo run --manifest-path {{BACKEND_DIR}}/Cargo.toml -p {{API_PKG}}

# Runs the component-level throughput/latency perf harness (no HTTP/JSON).
backend-perf *args:
  cargo run --manifest-path {{BACKEND_DIR}}/Cargo.toml -p wallet-perf -- {{args}}

clean-backend:
  cargo clean --manifest-path {{BACKEND_DIR}}/Cargo.toml

# --------------------
# DB
# --------------------

drop-db:
  {{ if os() == "windows" { "just drop-db-win" } else { "just drop-db-unix" } }}

drop-db-win:
  $p = if ($env:WALLET_DB_PATH) { $env:WALLET_DB_PATH } else { 'wallet-backend/wallet.db' }
  Remove-Item -Force -ErrorAction SilentlyContinue $p, "$p-wal", "$p-shm"

drop-db-unix:
  DB_PATH="${WALLET_DB_PATH:-wallet-backend/wallet.db}"
  rm -f "$DB_PATH" "$DB_PATH-wal" "$DB_PATH-shm"

reset-db: drop-db

# --------------------
# Combined
# --------------------

build:
  just backend-build
  just web-build

test:
  just backend-test
  just web-test

lint:
  just backend-fmt-check
  just backend-clippy
  just web-lint

ci:
  just lint
  just test
  just build

clean:
  just clean-backend
  just clean-web

# --------------------
# Dev (one command to run everything)
# --------------------

dev:
  {{ if os() == "windows" { "just dev-win" } else { "just dev-unix" } }}

dev-win:
  just web-install
  Start-Process -WorkingDirectory 'wallet-backend' -FilePath 'powershell.exe' -ArgumentList @('-NoLogo','-NoProfile','-Command','cargo run -p wallet-api')
  Start-Process -WorkingDirectory 'wallet-web' -FilePath 'powershell.exe' -ArgumentList @('-NoLogo','-NoProfile','-Command','npm run dev')
  Write-Host 'API: http://127.0.0.1:3000'
  Write-Host 'WEB: http://127.0.0.1:5173'

dev-unix:
  just web-install
  (cd wallet-backend && cargo run -p wallet-api) & (cd wallet-web && npm run dev) ; wait

reload:
  just drop-db
  just dev
