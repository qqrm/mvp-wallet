PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS accounts (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  owner_type   TEXT NOT NULL,   -- 'user' | 'system'
  owner_id     TEXT NOT NULL,   -- user_id as string, or system key
  status       TEXT NOT NULL DEFAULT 'active',
  created_at   TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_owner
ON accounts(owner_type, owner_id);

CREATE TABLE IF NOT EXISTS account_currency (
  account_id     INTEGER NOT NULL,
  currency       TEXT NOT NULL,
  allow_negative INTEGER NOT NULL DEFAULT 0,
  allow_hold     INTEGER NOT NULL DEFAULT 0,
  opened_at      TEXT NOT NULL,
  PRIMARY KEY(account_id, currency),
  FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ledger_transactions (
  id              TEXT PRIMARY KEY,      -- uuid
  idempotency_key TEXT NOT NULL,
  tx_type         TEXT NOT NULL,         -- 'topup' | 'transfer'
  state           TEXT NOT NULL,         -- 'posted'
  user_account_id INTEGER NOT NULL,      -- "main actor" (payer / topup receiver)
  currency        TEXT NOT NULL,
  amount_minor    INTEGER NOT NULL,
  created_at      TEXT NOT NULL,
  posted_at       TEXT NOT NULL,
  metadata_json   TEXT NULL,
  FOREIGN KEY(user_account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ledger_transactions_idem_user
ON ledger_transactions(user_account_id, idempotency_key);

CREATE INDEX IF NOT EXISTS idx_ledger_transactions_user
ON ledger_transactions(user_account_id, created_at DESC);

CREATE TABLE IF NOT EXISTS ledger_entries (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  tx_id       TEXT NOT NULL,
  account_id  INTEGER NOT NULL,
  currency    TEXT NOT NULL,
  direction   TEXT NOT NULL,     -- 'debit' | 'credit'
  amount_minor INTEGER NOT NULL,
  created_at  TEXT NOT NULL,
  FOREIGN KEY(tx_id) REFERENCES ledger_transactions(id) ON DELETE CASCADE,
  FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_tx
ON ledger_entries(tx_id);

CREATE TABLE IF NOT EXISTS balance_projection (
  account_id    INTEGER NOT NULL,
  currency      TEXT NOT NULL,
  available_minor INTEGER NOT NULL DEFAULT 0,
  hold_minor      INTEGER NOT NULL DEFAULT 0,
  updated_at    TEXT NOT NULL,
  PRIMARY KEY(account_id, currency),
  FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_balance_projection_account
ON balance_projection(account_id);
