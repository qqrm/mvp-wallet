ALTER TABLE http_idempotency ADD COLUMN status TEXT NOT NULL DEFAULT 'reserved';
ALTER TABLE http_idempotency ADD COLUMN result_tx_id TEXT;
ALTER TABLE http_idempotency ADD COLUMN reserved_at TEXT;
ALTER TABLE http_idempotency ADD COLUMN completed_at TEXT;
ALTER TABLE http_idempotency ADD COLUMN error_code INTEGER;
ALTER TABLE http_idempotency ADD COLUMN error_message TEXT;

UPDATE http_idempotency
SET reserved_at = created_at
WHERE reserved_at IS NULL;
