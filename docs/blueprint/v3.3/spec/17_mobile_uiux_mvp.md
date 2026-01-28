# 17. MOBILE UI/UX (MVP)

Navigation
- Bottom tabs: Home / Transfers / History / Settings (3–4 tabs).

Principles
- Available balance is always consistent with receipts.
- Every action has a review step and produces a receipt containing tx_id.
- Errors: user-friendly message for the UI plus a hidden/stable error.code for support/diagnostics.

Screen list (>= 12)
- Splash / session restore
- Login (phone + OTP)
- Home
- Accounts list (if multi-currency)
- Account details (balance breakdown)
- Transaction list (filters + pagination)
- Transaction details / receipt
- Transfer form
- Transfer review + confirm
- FX convert form (quote)
- FX quote review + execute result
- Spend simulation (sandbox)
- Settings & Security
- Limits (read-only)
