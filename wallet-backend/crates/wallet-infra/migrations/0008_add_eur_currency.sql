-- Add EUR to currency registry (deterministic, idempotent).

INSERT OR IGNORE INTO currencies(code, minor_units, created_at)
VALUES ('EUR', 2, strftime('%Y-%m-%dT%H:%M:%fZ','now'));
