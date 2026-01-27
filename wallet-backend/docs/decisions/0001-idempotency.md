# HTTP Idempotency MVP (no response cache)

## Context
We need HTTP idempotency for money-moving endpoints. The MVP must guarantee "same Idempotency-Key → same effect" without caching response JSON.

## Decision
* Use the `http_idempotency` table as a lightweight coordination record.
* Store only the request hash, a status (`reserved`/`completed`/`failed`), lease timestamps, and `result_tx_id`.
* Build replay responses by reloading data from the ledger by `result_tx_id` (and the original tx id for refunds).
* Reserve + finalize the idempotency record in the **same SQL transaction** as the monetary operation.
* Use a lease TTL on `reserved` so crashed in-flight requests can be re-acquired.
* Reject key reuse with a different payload with `409 Conflict`.

## Consequences
* Repeat calls with the same key return the same `tx_id` without double-applying money.
* If the same key is reused with a different payload, we return `409 Conflict`.
* We do not store response JSON in the MVP, which keeps the table small and avoids stale response cache.
* For scale, we can later introduce response caching keyed by `result_tx_id` if needed.
