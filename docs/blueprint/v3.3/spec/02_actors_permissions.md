# 2. ACTORS & PERMISSIONS

2.1 User (role=USER)
- Can read own accounts/balances/history/receipts.
- Can perform P2P transfers (internal only).
- Can request FX quotes and execute FX conversions.
- Can run spend simulation.

2.2 Admin (role=ADMIN)
- Can create user accounts, close user accounts.
- Can fund (mint) and withdraw (burn) balances.
- Can reverse posted transactions.
- Can search ledger/transactions for support.

2.3 System (actor=SYSTEM)
- Performs internal postings (fees, system accounts).
- Emits system audit events.
