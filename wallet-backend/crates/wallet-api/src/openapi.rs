use utoipa::OpenApi;

use wallet_app::{
    AdminCloseAccountResponse, AdminCreateAccountRequest, AdminCreateAccountResponse,
    AdminCreateUserRequest, AdminCreateUserResponse, AdminHoldRequest, AdminListUsersResponse,
    AdminOpenCurrencyAccountRequest, AdminOpenCurrencyAccountResponse, AdminTopupRequest,
    AdminUserItem, BalanceItem, CurrencyItem, DevAccountItem, DevUserAccountsResponse, DevUserItem,
    DevUsersResponse, ListBalancesResponse, ListCurrenciesResponse, ListTxsQuery, ListTxsResponse,
    PostOpResponse, RefundResponse, TopupRequest, TransferRequest, TxItem, TxReceiptEntryItem,
    TxReceiptResponse,
};

/// OpenAPI document for the MVP.
///
/// Notes:
/// - This is intentionally minimal and reflects the current MVP endpoints.
/// - Non-localhost auth is a simple `Authorization: Bearer ...` gate.
/// - Localhost requests bypass tokens and use `X-Dev-User` or `?as=` (default `u01`).
#[derive(OpenApi)]
#[openapi(
    info(
        title = "wallet-backend",
        version = "0.1.0",
        description = "Wallet MVP API (ledger + balances projection + RBAC). Dev endpoints under /v1/dev are available only on localhost."
    ),
    paths(
        crate::api::health,
        crate::api::currencies_list,
        crate::api::tx_receipt,
        crate::api::wallet_balances,
        crate::api::wallet_txs,
        crate::api::wallet_transfer,
        crate::api::admin_list_users,
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
        crate::api::dev_users,
        crate::api::dev_user_accounts,
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
        AdminUserItem,
        AdminListUsersResponse,
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
        DevUserItem,
        DevUsersResponse,
        DevAccountItem,
        DevUserAccountsResponse,
    ))
)]
pub struct ApiDoc;
