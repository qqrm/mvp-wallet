# Justfile (root)

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
# Frontend helpers
# --------------------

web-install:
    @pwsh -NoLogo -NoProfile -Command ^
        "$ProgressPreference='SilentlyContinue'; " ^
        "Set-StrictMode -Version Latest; " ^
        "Push-Location 'wallet-web'; " ^
        "npm ci --no-audit --no-fund; " ^
        "if ($LASTEXITCODE -ne 0) { " ^
        "  Write-Host 'npm ci failed (lock mismatch). Running npm install to resync lockfile...' -ForegroundColor Yellow; " ^
        "  npm install --no-audit --no-fund; " ^
        "  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE } " ^
        "} " ^
        "Pop-Location"


web-build:
  npm --prefix {{WEB_DIR}} run build

# In this repo there are no "test"/"lint" scripts for the web yet.
# Minimal "test/lint" = typecheck+build (vue-tsc + vite build).
web-test:
  npm --prefix {{WEB_DIR}} run typecheck
  just web-build

web-lint:
  npm --prefix {{WEB_DIR}} run typecheck

web-dev:
  npm --prefix {{WEB_DIR}} run dev

clean-web:
  {{ if os() == "windows" { "just clean-web-win" } else { "just clean-web-unix" } }}

clean-web-win:
  Remove-Item -Recurse -Force -ErrorAction SilentlyContinue wallet-web/node_modules, wallet-web/dist

clean-web-unix:
  rm -rf wallet-web/node_modules wallet-web/dist

# --------------------
# Backend helpers
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
# Combined (requested MVP commands)
# --------------------

build:
  just web-install
  just backend-build
  just web-build

test:
  just web-install
  just backend-test
  just web-test

lint:
  just web-install
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
# Dev environment (two windows)
# --------------------

dev:
  just web-install
  {{ if os() == "windows" { "just dev-win" } else { "just dev-unix" } }}

dev-win:
  Start-Process -WorkingDirectory 'wallet-backend' -FilePath 'powershell.exe' -ArgumentList @('-NoLogo','-NoProfile','-Command','cargo run -p wallet-api')
  Start-Process -WorkingDirectory 'wallet-web' -FilePath 'powershell.exe' -ArgumentList @('-NoLogo','-NoProfile','-Command','npm run dev')
  Write-Host 'API: http://127.0.0.1:3000'
  Write-Host 'WEB: http://127.0.0.1:5173'

dev-unix:
  (cd wallet-backend && cargo run -p wallet-api) & (cd wallet-web && npm run dev) ; wait

reload:
  just drop-db
  just dev
