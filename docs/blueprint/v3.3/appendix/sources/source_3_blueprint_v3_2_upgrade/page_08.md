# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 08

Data Protection: Communication is over HTTPS (TLS). Sensitive data at rest (if any, e.g. user personal info
or secrets) is encrypted at the database/storage level (leveraging PostgreSQL TDE or cloud encryption, with
keys in a secure KMS). The amount of PII in the wallet system is minimal (mostly phone/email, since detailed
KYC info is not stored in MVP). Still, we ensure compliance with Uzbekistan’s personal data laws – e.g.,
storing data in approved regions and protecting user consent metadata
.
Fraud & Abuse Controls: MVP introduces basic rate limiting: for critical endpoints like transfers, FX, login –
to prevent brute-force or abuse. For instance, a user cannot initiate more than X transfers per minute, and
an IP address cannot hit the API excessively
. These limits mitigate spam or DDoS vectors. There is no
automated AML rule engine in MVP, but the  audit logs and  admin search provide manual oversight
capability. Compliance staff can review transaction histories for suspicious patterns. The design keeps
hooks for integrating an AML monitoring tool post-MVP (for example, flagging large transfers or rapid in-
out movements once KYC is in place).
Regulatory Scope: As MVP does not involve external money movement, regulatory exposure is limited. The
wallet operates under Uzum’s licensed banking entity by assumption
, and all value remains internal.
Nevertheless, we log all transactions for potential future regulatory reporting. If/when external payments
(e.g. cards, bank connections) and KYC are introduced, full compliance with payment services regulations
and identity verification laws will be required. We have reserved data fields and extension points for these
(e.g., the ability to store KYC status per user, to enforce limits based on verification tier, etc., in later
versions).
Audit  &  Logging: Every  important  action  generates  an  AuditEvent  as  described.  These  logs  include
sufficient detail to trace who did what and when (critical for forensic analysis). For example, if an admin
performs a fund or reverse, the event will capture the admin’s user_id, the target account or tx_id, and a
correlation_id linking it to the API request ID for cross-reference
. Audit events are stored indefinitely for
now (with at least 5-year retention recommended)
. The system should also maintain regular database
backups and point-in-time recovery (PITR) capability to prevent data loss.
Product Decision Record (PDR – v3.2 Q&A)
This section records key product questions that were raised in the design of the wallet and the decisions
made in v3.2 to address them, along with brief reasoning:
Q: How should users specify a transfer recipient – only by account number, or can we use
phone numbers/user IDs for convenience?
A: We will support multiple addressing methods in MVP v3.2. Users can send money by providing the
recipient’s phone number or internal user_id, not just an account ID. The system will look up the
appropriate account behind the scenes (e.g. the recipient’s active account in the same currency). This
decision improves UX (it leverages the fact that phone-number-based transfers are a fundamental
expectation in our markets
) while keeping it internal – no external phone directory needed
beyond our user database. It also sets the stage for later integration with contacts or national
payment systems. The implementation will carefully handle resolution and locking (always resolving
first, and locking accounts in consistent order by user_id to avoid deadlocks).
Q: What source of FX rates will the wallet use for currency conversion in MVP?
A: For MVP, we chose to use a static, configurable FX rate table stored in our database
. This
39
40
41
10
42
- 3
- 2
8
