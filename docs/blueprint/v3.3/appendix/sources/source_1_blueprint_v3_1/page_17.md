# SOURCE_1__Blueprint_v3_1

## Page 17

Uzum Wallet MVP Blueprint v3.1 (MVP, monolith, contract-first)
2026-01-26
Confidential draft - for CEO/implementation discussion
Page 17
(5) RELIABILITY & TEST PLAN (ENGINEERING TECHNIQUES)
E2E tests (critical journeys)
- e2e_login_and_get_wallet_overview: login -> fetch /profile -> get balance.
- e2e_admin_create_account_then_fund: create account -> fund -> balance increased.
- e2e_p2p_transfer_success: sender funded -> transfer -> sender decreases, receiver increases; tx receipt correct.
- e2e_p2p_insufficient_funds: transfer fails with INSUFFICIENT_FUNDS and no ledger mutation.
- e2e_spend_simulate_posts_once: simulate spend -> exactly one tx posted; balance decreases once.
- e2e_fx_quote_execute_success: quote -> execute -> multi-currency balances updated and fee posted.
- e2e_fx_execute_expired_quote: quote expires -> execute fails with INVALID_STATE.
- e2e_admin_withdraw_burn_to_sink: withdraw -> SYSTEM_SINK credited, user debited.
- e2e_admin_reverse_compensates: post tx -> reverse -> net effect zero; original remains immutable.
- e2e_cursor_pagination_history: create N tx -> paginate -> no gaps/duplicates, stable order.
Contract tests
- openapi_schema_validation_all_endpoints: responses validate against OpenAPI for success + error cases.
- consumer_error_code_stability: each failure mode returns correct error.code and HTTP status.
- pagination_contract: cursor tokens are opaque strings; limit respected; deterministic order.
Type-state pattern (where to apply)
- TxState: Created -> Posted -> Reversed; compile-time guarded transitions for posting and reversing.
- FxQuoteState: Created -> Active -> Expired -> Executed; execute requires Active.
- AccountState: Active -> Closed; posting requires Active unless admin correction flag.
Property-based tests (ledger invariants)
- prop_double_entry_sum_zero: for every tx, sum(entries.signed_amount) == 0 per currency.
- prop_no_negative_available: available >= 0 after any sequence of valid operations.
- prop_idempotency_replay_same_result: same key + same payload returns identical tx_id and balances.
