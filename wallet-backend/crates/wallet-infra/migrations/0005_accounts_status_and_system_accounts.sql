-- Account lifecycle: closed_at + label.
-- NOTE: SQLite supports ADD COLUMN but not dropping columns/indexes.

ALTER TABLE accounts ADD COLUMN closed_at TEXT;
ALTER TABLE accounts ADD COLUMN label TEXT NOT NULL DEFAULT '';

-- Seed system accounts for default currency (UZS).
-- Currency-scoped owner_id keeps the current schema simple.

INSERT OR IGNORE INTO accounts(owner_type, owner_id, status, created_at, label)
VALUES
  ('system', 'SYSTEM_SINK:UZS', 'active', strftime('%Y-%m-%dT%H:%M:%fZ','now'), 'SYSTEM_SINK'),
  ('system', 'SYSTEM_SPEND:UZS', 'active', strftime('%Y-%m-%dT%H:%M:%fZ','now'), 'SYSTEM_SPEND');

-- Open currency + projection for the seeded system accounts.
INSERT OR IGNORE INTO account_currency(account_id, currency, allow_negative, allow_hold, opened_at)
SELECT id, 'UZS', 1, 0, strftime('%Y-%m-%dT%H:%M:%fZ','now')
FROM accounts
WHERE owner_type = 'system' AND owner_id IN ('SYSTEM_SINK:UZS', 'SYSTEM_SPEND:UZS');

INSERT OR IGNORE INTO balance_projection(account_id, currency, available_minor, hold_minor, updated_at)
SELECT id, 'UZS', 0, 0, strftime('%Y-%m-%dT%H:%M:%fZ','now')
FROM accounts
WHERE owner_type = 'system' AND owner_id IN ('SYSTEM_SINK:UZS', 'SYSTEM_SPEND:UZS');
