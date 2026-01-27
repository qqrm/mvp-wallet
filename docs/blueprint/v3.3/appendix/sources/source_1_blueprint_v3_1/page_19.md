# SOURCE_1__Blueprint_v3_1

## Page 19

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 19
(6) MOBILE UI/UX MVP SPEC (UZUM UI KIT-ALIGNED)
Information architecture / navigation
- Bottom navigation (3-4 tabs): Home, Transfers, History, Settings.
- Home is the default: balances + quick actions.
- All money actions are confirmable with a review screen + biometric/OTP (optional).
- Transaction receipt is shareable (PDF/image) and contains fx rate/fee when applicable.
MVP screen list (>= 12)
- S1 Splash / Session restore
- S2 Login (phone + OTP) / Sign-in
- S3 Home (wallet overview)
- S4 Accounts list (if multi-currency)
- S5 Account details (balance breakdown)
- S6 Transaction list (filters + pagination)
- S7 Transaction details / receipt
- S8 Transfer form (P2P)
- S9 Transfer review + confirm
- S10 FX convert form (quote)
- S11 FX quote review + execute result
- S12 Spend simulation (sandbox/developer mode)
- S13 Settings & Security
- S14 Limits (read-only in MVP)
Screen specifications (implementation-ready)
S3 Home (Wallet overview)
Purpose: Show total and per-currency balances; provide quick actions.
