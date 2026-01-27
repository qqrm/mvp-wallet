# SOURCE_2__Blueprint_v3_2

## Page 06

}
}
All
 error
 codes
 are
 predefined
 (e.g.
 VALIDATION_ERROR ,
 INSUFFICIENT_FUNDS ,
IDEMPOTENCY_CONFLICT , etc.) and the error response format is consistent across endpoints
.
Idempotency & Concurrency
Every money-moving POST request must include a unique Idempotency-Key. The system stores the
outcome of each request by key; a retry with the same key returns the saved result (without re-
executing)
. If a repeat uses the same key but a different payload, the request is rejected with
409 IDEMPOTENCY_CONFLICT
.
Idempotency records are retained for 48 hours to allow safe client retries, after which they expire
and may be purged
. This TTL prevents unbounded growth of the idempotency log while covering
typical retry windows
.
Concurrency: The service ensures only one transaction modifies a given account’s balance at a time.
Each transfer or balance-changing operation acquires a write lock on the affected account rows (e.g.
using SELECT ... FOR UPDATE ) or runs in a serialized transaction
. For operations involving
two accounts (like P2P transfers), accounts are locked in a consistent order (e.g. by sorted user/
account ID) to avoid deadlocks
.
Security Architecture
Authentication & Authorization: All endpoints require a valid JWT issued by Uzum’s central identity
(SSO) system
. User tokens grant access only to their own resources (enforced by checking the
token’s user_id against the requested data)
. Admin endpoints require an admin-scoped token; all
such actions are additionally recorded in audit logs with the acting admin’s identity
.
Access Control: Role-based access is enforced (regular users cannot call admin APIs). Within user-
facing APIs, each request is also authorized against the owning user’s ID to prevent data leaks (e.g.,
a user cannot fetch another user’s account or transactions)
.
Transport Security: All communication is over HTTPS with JWT-based auth as above. The wallet
service is assumed to run in a secure network environment (within Uzum’s cloud/VPC) and behind an
API gateway that handles TLS termination and basic protections.
Rate Limiting: To prevent abuse, the system should enforce basic rate limits on critical endpoints
(e.g. transfers, login) per user and per IP. This is planned as part of the deployment configuration
(e.g. using an API gateway or middleware)
. No fine-grained fraud rules are in MVP beyond these
limits and the AuditEvent logging.
Sensitive Data: The wallet does not store highly sensitive personal data in MVP (no full KYC info),
and payment data stays internal. Standard data protection measures (encryption at rest, GDPR/
Uzbek PDPL compliance) are followed as applicable
. In the future, if external integrations are
added, additional PCI/PII safeguards will be introduced.
Compliance & Regulatory Considerations
Licensing & Operational Entity: For MVP, the wallet operates under Uzum Bank’s existing financial
license as an internal product feature (closed-loop within Uzum ecosystem)
. This avoids needing
40
41
- 30
38
42
- 10
20
- 43
44
- 45
46
47
- 46
-
- 48
- 49
- 50
6
