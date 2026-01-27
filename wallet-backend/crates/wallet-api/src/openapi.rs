use utoipa::OpenApi;

use wallet_app::{
    AdminCloseAccountResponse, AdminCreateAccountRequest, AdminCreateAccountResponse,
    AdminCreateUserRequest, AdminCreateUserResponse, AdminHoldRequest,
    AdminOpenCurrencyAccountRequest, AdminOpenCurrencyAccountResponse, AdminTopupRequest,
    BalanceItem, CurrencyItem, ListBalancesResponse, ListCurrenciesResponse, ListTxsQuery,
    ListTxsResponse, PostOpResponse, RefundResponse, TopupRequest, TransferRequest, TxItem,
    TxReceiptEntryItem, TxReceiptResponse,
};

/// OpenAPI document for the MVP.
///
/// Notes:
/// - This is intentionally minimal and reflects the current MVP endpoints.
/// - Auth is a simple `Authorization: Bearer ...` gate (see `agents.md`).
#[derive(OpenApi)]
#[openapi(
    info(
        title = "wallet-backend",
        version = "0.1.0",
        description = "Wallet MVP API (ledger + balances projection + RBAC)."
    ),
    paths(
        crate::api::health,
        crate::api::currencies_list,
        crate::api::tx_receipt,
        crate::api::wallet_balances,
        crate::api::wallet_txs,
        crate::api::wallet_transfer,
        crate::api::admin_create_user,
        crate::api::admin_open_currency_account,
        crate::api::admin_create_account,
        crate::api::admin_close_account,
        crate::api::admin_topup,
        crate::api::admin_hold_authorize,
        crate::api::admin_hold_capture,
        crate::api::admin_hold_cancel,
        crate::api::admin_payment_refund,
        crate::api::admin_user_balances,
        crate::api::admin_user_txs,
    ),
    components(schemas(
        BalanceItem,
        TxItem,
        TxReceiptEntryItem,
        TxReceiptResponse,
        TopupRequest,
        TransferRequest,
        AdminCreateUserRequest,
        AdminCreateUserResponse,
        AdminOpenCurrencyAccountRequest,
        AdminOpenCurrencyAccountResponse,
        AdminCreateAccountRequest,
        AdminCreateAccountResponse,
        AdminCloseAccountResponse,
        AdminTopupRequest,
        AdminHoldRequest,
        CurrencyItem,
        ListCurrenciesResponse,
        ListBalancesResponse,
        ListTxsQuery,
        ListTxsResponse,
        PostOpResponse,
        RefundResponse,
    ))
)]
pub struct ApiDoc;
