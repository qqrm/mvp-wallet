-- Payments / holds and an MVP outbox.
--
-- We store the payment state machine in `ledger_transactions.state` as TEXT.
-- Existing rows use `posted`.
-- New states:
--   - authorized
--   - posted
--   - reversed
--   - refunded
--
-- Tx types are also TEXT:
--   - topup
--   - transfer
--   - payment
--   - refund

CREATE TABLE IF NOT EXISTS outbox_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    published_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_outbox_events_created_at
    ON outbox_events(created_at);

-- For a basic publisher worker: select where published_at IS NULL.
CREATE INDEX IF NOT EXISTS idx_outbox_events_unpublished
    ON outbox_events(published_at);
