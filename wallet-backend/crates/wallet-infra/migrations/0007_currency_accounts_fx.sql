-- v3.3: currency_accounts (API-level account_id per currency) + FX tables (schema only)
PRAGMA foreign_keys = ON;

-- Currency-scoped accounts: maps (root_account_id, currency) -> currency_account_id.
-- NOTE: We keep existing ledger schema where account_id == root_account_id and currency is a column.
-- currency_accounts is an API-facing mapping layer, not a replacement for `accounts`.
CREATE TABLE IF NOT EXISTS currency_accounts (
  id              INTEGER PRIMARY KEY AUTOINCREMENT,
  root_account_id INTEGER NOT NULL,
  currency        TEXT NOT NULL,
  status          TEXT NOT NULL DEFAULT 'active', -- 'active' | 'closed'
  created_at      TEXT NOT NULL,
  closed_at       TEXT,
  label           TEXT NOT NULL DEFAULT '',
  UNIQUE(root_account_id, currency),
  FOREIGN KEY(root_account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_currency_accounts_root
ON currency_accounts(root_account_id, currency);

-- Backfill from legacy table: one row per (account_id, currency).
INSERT OR IGNORE INTO currency_accounts(root_account_id, currency, status, created_at, label)
SELECT account_id, currency, 'active', opened_at, ''
FROM account_currency;

-- Generic key/value settings.
CREATE TABLE IF NOT EXISTS settings (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

INSERT OR IGNORE INTO settings(key, value, updated_at)
VALUES ('fx_fee_bps', '100', strftime('%Y-%m-%dT%H:%M:%fZ','now'));

-- Admin-managed FX rates table (static source for MVP).
-- rate is stored as fixed-point: rate_scaled / rate_scale.
CREATE TABLE IF NOT EXISTS fx_rates (
  base_currency  TEXT NOT NULL,
  quote_currency TEXT NOT NULL,
  rate_scaled    INTEGER NOT NULL,
  rate_scale     INTEGER NOT NULL,
  updated_at     TEXT NOT NULL,
  PRIMARY KEY (base_currency, quote_currency)
);

-- Stored firm quotes (snapshot) with TTL; can be referenced in receipts/audit.
CREATE TABLE IF NOT EXISTS fx_quotes (
  quote_id           TEXT PRIMARY KEY,
  user_id            TEXT NOT NULL,
  from_currency      TEXT NOT NULL,
  to_currency        TEXT NOT NULL,
  amount_from_minor  INTEGER NOT NULL,
  amount_to_minor    INTEGER NOT NULL,
  rate_scaled        INTEGER NOT NULL,
  rate_scale         INTEGER NOT NULL,
  fee_bps            INTEGER NOT NULL,
  fee_minor          INTEGER NOT NULL,
  created_at         TEXT NOT NULL,
  expires_at         TEXT NOT NULL,
  executed_tx_id     TEXT,
  executed_at        TEXT
);

CREATE INDEX IF NOT EXISTS idx_fx_quotes_user_created
ON fx_quotes(user_id, created_at DESC);
