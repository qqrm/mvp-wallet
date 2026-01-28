-- Ensure core currency registry entries are present (idempotent).

INSERT OR IGNORE INTO currencies(code, minor_units, created_at)
VALUES
  ('RUB', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ('UZS', 0, strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ('USD', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ('EUR', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now'));
