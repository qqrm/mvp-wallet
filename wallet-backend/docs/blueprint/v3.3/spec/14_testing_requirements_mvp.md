# 14. TESTING REQUIREMENTS (MVP)

14.1 Invariant tests (property-based)
- No money created/destroyed unexpectedly.
- Double-entry sum zero per tx/currency.
- Idempotency replay returns exact same result.

14.2 Integration tests
- fund -> transfer -> balance + receipt
- fx quote -> execute -> receipt
- reverse -> balances corrected

14.3 Load test (minimum)
- Demonstrate no deadlocks under concurrent transfers.
