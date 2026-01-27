# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 07

Idempotency  conflict  (same  key  used  with  different  request  data)  returns  HTTP  409  with  an
IDEMPOTENCY_CONFLICT  error code
.
Idempotency & Concurrency Controls
Idempotency  Design: Every  client-initiated  write  operation  (money  movement)  must  supply  an
Idempotency-Key (a UUID) in the request header
. The server stores a record of this key and the
request details on first processing. If the exact same key is seen again:
If the subsequent request is identical (same endpoint and payload hash), the server short-circuits
and returns the originally stored response (with an Idempotency-Replayed header)
. This
allows safe client retries of, say, a transfer, without duplicating the transaction.
If the same key is reused with a different payload, the server responds with HTTP 409 Conflict
( IDEMPOTENCY_CONFLICT  error) and does not process the new request
. This indicates a logical
error on client side (key collision).
Idempotency records are persisted in the idempotency  table with a time-to-live of 48 hours.
After 48h, the system may discard the record, meaning the same key could be reused (though clients
are expected to use unique keys for new requests). The 48h retention strikes a balance between
allowing cross-day retries and preventing unbounded growth of the idempotency log.
Concurrency and Locking: The wallet operates under  serializable transaction isolation or explicit row
locking to prevent race conditions (e.g., two transfers hitting the same account concurrently)
. For
operations that involve multiple accounts (like a transfer between two users), the implementation must lock
the rows (accounts) in a consistent order to avoid deadlocks. For example, always lock in ascending order
by  user_id  (or account_id) when transferring between two accounts. This deterministic locking order
ensures no cyclical waits occur
. If one transaction is already processing involving Account A then B,
another concurrent transaction involving B then A will wait for locks in the same sequence rather than
deadlock. We use SELECT ... FOR UPDATE  on account balance rows during posting to serialize balance
updates.
Additionally, to maintain consistency:
- No double-spend: A transfer will check the latest available balance after acquiring the lock; if insufficient,
it aborts with INSUFFICIENT_FUNDS  (no partial posting).
- Retry logic: In the rare case of a serialization failure (e.g., two FX executes trying to consume the same
quote, or other write skew), the operation will fail and can be retried by client (with same idempotency key,
yielding safe replay). Such scenarios should be minimal, but the system will log and expose metrics like
idempotency conflicts or serialization retries for monitoring
.
Security & Compliance Considerations
Authentication & Authorization: As noted, all API calls require a valid JWT. User tokens grant access only
to their own data (e.g. cannot fetch another user’s account), enforced by checking the user_id  in token vs
resource. Admin actions require an admin JWT with elevated scope; all such actions are additionally logged
in the AuditEvent table with who performed them
. Session management (login, logout) relies on Uzum’s
existing SSO/identity service. MVP defers advanced device binding or 2FA enforcement, though basic device
info is captured in audit logs.
30
31
32
19
- 33
9
- 30
- 34
35
36
37
38
7
