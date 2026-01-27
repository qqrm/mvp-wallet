-- Currency registry: supported currency codes + minor units.
-- This makes currency validation DB-driven and deterministic.

CREATE TABLE IF NOT EXISTS currencies (
  code        TEXT PRIMARY KEY,   -- canonical uppercase currency code (e.g. UZS)
  minor_units INTEGER NOT NULL,   -- 0..=6
  created_at  TEXT NOT NULL,
  CHECK (minor_units >= 0 AND minor_units <= 6),
  CHECK (code = upper(code))
);

INSERT OR IGNORE INTO currencies(code, minor_units, created_at)
VALUES
  ('RUB', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ('USD', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ('UZS', 0, strftime('%Y-%m-%dT%H:%M:%fZ','now'));
