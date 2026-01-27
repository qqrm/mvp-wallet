# SOURCE_1__Blueprint_v3_1

## Page 31

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 31
What changes depending on answer: Affects admin workflow and audit controls.
Default assumption: Default: 1-person admin in MVP + full audit log; 4-eyes in post-MVP.
Q: User authentication method: Uzum SSO vs standalone OTP?
Why it matters: Impacts onboarding speed and security boundary.
What changes depending on answer: Affects token issuance, session revocation, device binding.
Default assumption: Default: integrate with Uzum identity JWT via gateway.
Q: Do we need 'blocked/holds' in MVP?
Why it matters: Hold/capture introduces complexity but required for real merchant flows.
What changes depending on answer: Affects balance model and APIs.
Default assumption: Default: blocked=0; holds deferred.
Q: Is 'shared/family wallet' on near-term roadmap (Q2) or later?
Why it matters: Impacts data model (wallet groups) and permissions design.
What changes depending on answer: Affects whether to invest in tenant/wallet-group now.
Default assumption: Default: roadmap item only; schema reserves wallet_group_id.
