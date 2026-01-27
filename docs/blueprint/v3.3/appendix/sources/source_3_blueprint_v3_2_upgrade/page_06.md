# SOURCE_3__Blueprint_v3_2_Upgrade

## Page 06

type, timestamp, and a list of the entries (each with account info, amount, and direction debit/credit relative
to that account)
. It also includes human-readable annotations: for internal transfers, it will show the
other party’s name/identifier; for FX, the exchange rate and fee applied are included
; for simulated
spends, the merchant label is shown
. A short optional memo field is returned if the user attached a
note or if the transaction has a relevant label (e.g. “Uzum Market purchase”). The receipt is the source of
truth for transaction confirmation and is shareable (in-app users can generate a PDF or image of it)
. (No
idempotency for GET.)
- POST /v1/transfers  – Peer-to-Peer Transfer. Moves funds from one user’s account to another user’s
account. Request includes source account_id  (debitor), a way to specify the recipient, and an amount .
In v3.2, the recipient can be identified in three ways: by specifying their  account_id  directly, by their
user_id , or by their phone number. If user_id  or phone is provided, the backend will resolve it to the
target account_id (choosing the appropriate default account, e.g. in the same currency, or failing if none).
This resolution occurs within the transaction processing – after resolution, the service locks both the source
and target accounts (using a consistent order by user_id to avoid deadlock) and then validates balances. If
the source account has sufficient available funds and both accounts are ACTIVE, a JournalTx is posted with
two JournalEntries: one debiting the source and one crediting the destination. On success, a 201 response is
returned with the new transaction’s receipt. Idempotent: Yes – duplicate transfers with same Idempotency-
Key will return the same result or a conflict if payload differs.
-  POST  /v1/spend/simulate  –  Simulate  Spend. Deducts  an  amount  from  a  user’s  account  as  if
spending at a merchant, crediting the SYSTEM_SPEND account. Request includes account_id , amount ,
and a merchant_label  or description. Used for testing or demonstration (e.g. simulating a purchase).
The system processes it similar to a transfer (debit user, credit system account) and records the label in the
transaction metadata. Idempotent: Yes – to prevent double charges in simulation.
- POST /v1/fx/quote  – Create FX Quote. Initiates a two-step currency conversion. The user provides a
source account_id , target account_id  (must belong to the same user and be a different currency),
and an amount  to convert from the source. The system looks up the applicable exchange rate from the
fx_rates table for the currency pair and calculates the converted amount, applying any fee or markup. It
creates an FxQuote record with a unique quote_id , embedding the rate, fee, source/target accounts, and
an expiration time (e.g. 60 seconds from now)
. The response returns the quote details: the offered rate,
fee, the amount that would be credited to the target account, and the expires_at  timestamp. The quote
remains pending until executed or until it expires. Idempotent: Yes (clients can retry quote creation safely,
though a new quote will be generated each time; a repeat with the same key could return the same quote
or an equivalent one).
- POST /v1/fx/execute  – Execute FX Conversion. Commits a previously obtained FX quote. The request
includes the quote_id . The server will retrieve the FxQuote, validate that it is still active and not expired,
then perform the currency conversion: a JournalTx is posted that debits the source account (for the original
quote amount + fee) and credits the target account (for the quoted converted amount), and credits the fee
to the system fee account (if a fee was configured)
. If the quote has expired or already been used, the
execution is rejected with an error. On success, the response is a transaction receipt of the completed
conversion. Idempotent: Yes (only one execution will succeed for a given quote; duplicate calls will return the
same result or an error if already executed).
Note: All POST endpoints return standard JSON results on success (typically including the new resource ID
or transaction details). Error handling follows a unified model – a JSON with an "error"  object containing
a machine-readable code  and a human-readable message
. For example, if a transfer is attempted
with  insufficient  funds,  the  API  returns  a  422  or  409  error  with  code  INSUFFICIENT_FUNDS
.
23
24
25
26
14
27
28
29
6
