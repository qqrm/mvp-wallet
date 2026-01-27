use axum::{
    Extension, Router,
    extract::{OriginalUri, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use serde_json::json;

use crate::{
    app::AppState,
    auth::{AuthCtx, require_admin, require_user},
    error::{ApiError, ApiResult},
};

use wallet_app::*;
use wallet_domain::*;
use wallet_infra::{
    db,
    idempotency_http::{self, HttpIdempotencyOutcome},
    service,
};

pub fn routes() -> Router<AppState> {
    let mut router = Router::new()
        .route("/health", get(health))
        .route("/v1/currencies", get(currencies_list))
        .route("/v1/transactions/{tx_id}", get(tx_receipt))
        // v3.3 (SoT) user-facing API (read-only, additive)
        .route("/v1/profile", get(v33_profile))
        .route("/v1/accounts", get(v33_accounts_list))
        .route("/v1/accounts/{account_id}", get(v33_account_details))
        .route(
            "/v1/accounts/{account_id}/balance",
            get(v33_account_balance),
        )
        .route(
            "/v1/accounts/{account_id}/transactions",
            get(v33_account_transactions),
        )
        .route("/v1/wallet/{user_id}/balances", get(wallet_balances))
        .route("/v1/wallet/{user_id}/txs", get(wallet_txs))
        .route("/v1/wallet/{user_id}/transfer", post(wallet_transfer))
        // Admin API
        .route("/v1/admin/users", post(admin_create_user))
        .route(
            "/v1/admin/users/{user_id}/accounts",
            post(admin_open_currency_account),
        )
        .route("/v1/admin/accounts", post(admin_create_account))
        .route(
            "/v1/admin/accounts/{account_id}/close",
            post(admin_close_account),
        )
        .route("/v1/admin/topup", post(admin_topup))
        .route("/v1/admin/holds", post(admin_hold_authorize))
        .route("/v1/admin/holds/{tx_id}/capture", post(admin_hold_capture))
        .route("/v1/admin/holds/{tx_id}/cancel", post(admin_hold_cancel))
        .route(
            "/v1/admin/payments/{tx_id}/refund",
            post(admin_payment_refund),
        )
        .route(
            "/v1/admin/users/{user_id}/balances",
            get(admin_user_balances),
        )
        .route("/v1/admin/users/{user_id}/txs", get(admin_user_txs));

    if dev_routes_enabled() {
        router = router
            .route("/v1/dev/users", get(dev_users))
            .route("/v1/dev/users/{user_id}/accounts", get(dev_user_accounts));
    }

    router
}

fn dev_routes_enabled() -> bool {
    matches!(std::env::var("WALLET_DEV_NO_AUTH"), Ok(v) if v == "1")
}

#[utoipa::path(
    get,
    path = "/v1/transactions/{tx_id}",
    params(("tx_id" = String, Path, description = "Transaction ID (uuid)")),
    responses(
        (status = 200, description = "Transaction receipt", body = TxReceiptResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found"),
    )
)]
pub(crate) async fn tx_receipt(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(tx_id): Path<String>,
) -> ApiResult<Json<TxReceiptResponse>> {
    let tx_id = TxId::parse(&tx_id).map_err(ApiError::from)?;

    if require_admin(&auth).is_ok() {
        let r = service::get_tx_receipt_admin(&st.pool, &tx_id).await?;
        return Ok(Json(r));
    }

    let user_id = require_user(&auth)?;
    let r = service::get_tx_receipt_user(&st.pool, &user_id, &tx_id).await?;
    Ok(Json(r))
}

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "OK", body = String))
)]
pub(crate) async fn health() -> &'static str {
    "ok"
}

#[utoipa::path(
    get,
    path = "/v1/currencies",
    responses((status = 200, description = "Supported currencies", body = ListCurrenciesResponse))
)]
pub(crate) async fn currencies_list(
    State(state): State<AppState>,
) -> ApiResult<Json<ListCurrenciesResponse>> {
    let rows = db::list_currencies(&state.pool).await?;
    let items = rows
        .into_iter()
        .map(|(code, minor_units)| CurrencyItem { code, minor_units })
        .collect();

    Ok(Json(ListCurrenciesResponse { items }))
}

fn parse_idempotency(headers: &HeaderMap) -> ApiResult<IdempotencyKey> {
    let v = headers
        .get("Idempotency-Key")
        .ok_or(ApiError::BadRequest("missing Idempotency-Key header"))?;
    let s = v
        .to_str()
        .map_err(|_| ApiError::BadRequest("invalid Idempotency-Key header"))?;
    IdempotencyKey::parse(s).map_err(ApiError::from)
}

fn parse_account_id_param(account_id: &str) -> ApiResult<AccountId> {
    let raw = account_id.trim();
    let raw = raw.strip_prefix("acc_").unwrap_or(raw);
    let id: i64 = raw
        .parse()
        .map_err(|_| ApiError::BadRequest("invalid account_id"))?;
    Ok(AccountId::new(id))
}

fn parse_currency_account_id_param(account_id: &str) -> ApiResult<i64> {
    let raw = account_id.trim();
    let raw = raw.strip_prefix("acc_").unwrap_or(raw);
    let id: i64 = raw
        .parse()
        .map_err(|_| ApiError::BadRequest("invalid account_id"))?;
    Ok(id)
}

pub(crate) async fn v33_profile(
    Extension(auth): Extension<AuthCtx>,
) -> ApiResult<Json<ProfileResponse>> {
    let user_id = require_user(&auth)?;
    Ok(Json(ProfileResponse {
        phone_number: user_id.as_str().to_string(),
        email: None,
        full_name: None,
    }))
}

pub(crate) async fn v33_accounts_list(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
) -> ApiResult<Json<ListAccountsResponseV33>> {
    let user_id = require_user(&auth)?;
    let accounts = service::v33_list_accounts(&st.pool, &user_id).await?;
    Ok(Json(ListAccountsResponseV33 { accounts }))
}

pub(crate) async fn v33_account_details(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(account_id): Path<String>,
) -> ApiResult<Json<AccountDetailsResponseV33>> {
    let currency_account_id = parse_currency_account_id_param(&account_id)?;

    if require_admin(&auth).is_ok() {
        let r = service::v33_get_account_details_admin(&st.pool, currency_account_id).await?;
        return Ok(Json(r));
    }

    let user_id = require_user(&auth)?;
    let r = service::v33_get_account_details_user(&st.pool, &user_id, currency_account_id).await?;
    Ok(Json(r))
}

pub(crate) async fn v33_account_balance(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(account_id): Path<String>,
) -> ApiResult<Json<AccountBalanceResponseV33>> {
    let currency_account_id = parse_currency_account_id_param(&account_id)?;
    let user_id = require_user(&auth)?;
    let r = service::v33_get_account_balance_user(&st.pool, &user_id, currency_account_id).await?;
    Ok(Json(r))
}

pub(crate) async fn v33_account_transactions(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(account_id): Path<String>,
    Query(q): Query<ListAccountTransactionsQueryV33>,
) -> ApiResult<Json<ListAccountTransactionsResponseV33>> {
    let currency_account_id = parse_currency_account_id_param(&account_id)?;
    let user_id = require_user(&auth)?;

    let limit = q.limit.unwrap_or(100);
    let txs = service::v33_list_account_txs_user(
        &st.pool,
        &user_id,
        currency_account_id,
        q.before,
        limit,
    )
    .await?;

    Ok(Json(ListAccountTransactionsResponseV33 {
        account_id: format!("acc_{}", currency_account_id),
        txs,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/wallet/{user_id}/balances",
    params(("user_id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "Balances", body = ListBalancesResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
pub(crate) async fn wallet_balances(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<ListBalancesResponse>> {
    let user = UserId::parse(&user_id)?;
    let authed = require_user(&auth)?;
    if authed.as_str() != user.as_str() {
        return Err(ApiError::Forbidden("cannot access other user"));
    }

    let items = service::list_balances(&st.pool, &user).await?;
    Ok(Json(ListBalancesResponse { balances: items }))
}

#[utoipa::path(
    get,
    path = "/v1/wallet/{user_id}/txs",
    params(
        ("user_id" = String, Path, description = "User ID"),
        ListTxsQuery
    ),
    responses(
        (status = 200, description = "Transaction list", body = ListTxsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
pub(crate) async fn wallet_txs(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
    Query(q): Query<ListTxsQuery>,
) -> ApiResult<Json<ListTxsResponse>> {
    let user = UserId::parse(&user_id)?;
    let authed = require_user(&auth)?;
    if authed.as_str() != user.as_str() {
        return Err(ApiError::Forbidden("cannot access other user"));
    }

    let limit = q.limit.unwrap_or(50).min(200);
    let items = service::list_txs(&st.pool, &user, limit).await?;

    Ok(Json(ListTxsResponse { txs: items }))
}

#[utoipa::path(
    post,
    path = "/v1/wallet/{user_id}/transfer",
    params(("user_id" = String, Path, description = "User ID")),
    request_body = TransferRequest,
    responses(
        (status = 200, description = "Posted transfer", body = PostOpResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Idempotency conflict"),
    )
)]
pub(crate) async fn wallet_transfer(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Json(body): Json<TransferRequest>,
) -> ApiResult<Response> {
    let user = UserId::parse(&user_id)?;
    let authed = require_user(&auth)?;
    if authed.as_str() != user.as_str() {
        return Err(ApiError::Forbidden("cannot access other user"));
    }

    let idem = parse_idempotency(&headers)?;
    let body_value =
        serde_json::to_value(&body).map_err(|_| ApiError::Internal("invalid request body"))?;
    let scope = format!("user:{}:transfer", user.as_str());
    let request_hash = idempotency_http::request_hash(&body_value, uri.path(), user.as_str());
    let req = body.try_into()?;

    let idem_key = idem.clone();
    let user_id = user.clone();
    let res = idempotency_http::execute(&st.pool, &scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::transfer_posted(tx, &user_id, &idem_key, req).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::post_op_response_by_tx_id(&st.pool, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/dev/users",
    responses(
        (status = 200, description = "User list", body = DevUsersResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
pub(crate) async fn dev_users(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
) -> ApiResult<Json<DevUsersResponse>> {
    require_admin(&auth)?;
    let users = wallet_infra::dev::list_dev_users(&st.pool).await?;
    Ok(Json(DevUsersResponse { users }))
}

#[utoipa::path(
    get,
    path = "/v1/dev/users/{user_id}/accounts",
    params(("user_id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "User accounts", body = DevUserAccountsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    )
)]
pub(crate) async fn dev_user_accounts(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<DevUserAccountsResponse>> {
    require_admin(&auth)?;
    let user = UserId::parse(&user_id)?;
    let accounts = wallet_infra::dev::list_dev_user_accounts(&st.pool, &user).await?;
    Ok(Json(DevUserAccountsResponse {
        user_id: user.into_inner(),
        accounts,
    }))
}

// ------------------------ admin handlers ------------------------

#[utoipa::path(
    post,
    path = "/v1/admin/users",
    request_body = AdminCreateUserRequest,
    responses(
        (status = 200, description = "Created", body = AdminCreateUserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Already exists"),
    )
)]
pub(crate) async fn admin_create_user(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Json(body): Json<AdminCreateUserRequest>,
) -> ApiResult<Json<AdminCreateUserResponse>> {
    require_admin(&auth)?;

    let user = UserId::parse(&body.user_id)?;
    let created = service::admin_create_user(&st.pool, &user).await?;

    Ok(Json(AdminCreateUserResponse {
        user_id: user.into_inner(),
        created,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/admin/users/{user_id}/accounts",
    params(("user_id" = String, Path, description = "User ID")),
    request_body = AdminOpenCurrencyAccountRequest,
    responses(
        (status = 200, description = "Opened", body = AdminOpenCurrencyAccountResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found"),
    )
)]
pub(crate) async fn admin_open_currency_account(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
    Json(body): Json<AdminOpenCurrencyAccountRequest>,
) -> ApiResult<Json<AdminOpenCurrencyAccountResponse>> {
    require_admin(&auth)?;

    let user = UserId::parse(&user_id)?;
    let currency = wallet_domain::Currency::parse(&body.currency)?;

    let opened = service::admin_open_currency_account(&st.pool, &user, &currency).await?;

    Ok(Json(AdminOpenCurrencyAccountResponse {
        user_id: user.into_inner(),
        currency: currency.into_inner(),
        opened,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/admin/accounts",
    request_body = AdminCreateAccountRequest,
    responses(
        (status = 200, description = "Created", body = AdminCreateAccountResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    )
)]
pub(crate) async fn admin_create_account(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Json(body): Json<AdminCreateAccountRequest>,
) -> ApiResult<Json<AdminCreateAccountResponse>> {
    require_admin(&auth)?;

    let req = AdminCreateAccountRequestValidated::try_from(body)?;
    let resp =
        service::admin_create_account(&st.pool, &req.owner_user_id, &req.currency, &req.label)
            .await?;

    Ok(Json(resp))
}

#[utoipa::path(
    post,
    path = "/v1/admin/accounts/{account_id}/close",
    params(("account_id" = String, Path, description = "Account ID (acc_... or numeric)")),
    responses(
        (status = 200, description = "Closed (idempotent)", body = AdminCloseAccountResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    )
)]
pub(crate) async fn admin_close_account(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(account_id): Path<String>,
) -> ApiResult<Json<AdminCloseAccountResponse>> {
    require_admin(&auth)?;

    let account_id = parse_account_id_param(&account_id)?;
    let resp = service::admin_close_account(&st.pool, account_id).await?;
    Ok(Json(resp))
}

#[utoipa::path(
    post,
    path = "/v1/admin/topup",
    request_body = AdminTopupRequest,
    responses(
        (status = 200, description = "Posted topup", body = PostOpResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User/account not found"),
    )
)]
pub(crate) async fn admin_topup(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Json(body): Json<AdminTopupRequest>,
) -> ApiResult<Response> {
    require_admin(&auth)?;

    let idem = parse_idempotency(&headers)?;
    let user = UserId::parse(&body.user_id)?;
    let body_value =
        serde_json::to_value(&body).map_err(|_| ApiError::Internal("invalid request body"))?;
    let scope = "admin:topup";
    let request_hash = idempotency_http::request_hash(&body_value, uri.path(), "admin");

    let req: TopupRequestValidated = TopupRequest {
        currency: body.currency,
        amount_minor: body.amount_minor,
    }
    .try_into()?;

    let idem_key = idem.clone();
    let user_id = user.clone();
    let res = idempotency_http::execute(&st.pool, scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::topup_posted(tx, &user_id, &idem_key, req).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::post_op_response_by_tx_id(&st.pool, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/holds",
    request_body = AdminHoldRequest,
    responses(
        (status = 200, description = "Authorized hold", body = PostOpResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User/account not found"),
        (status = 409, description = "Idempotency conflict"),
    )
)]
pub(crate) async fn admin_hold_authorize(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Json(body): Json<AdminHoldRequest>,
) -> ApiResult<Response> {
    require_admin(&auth)?;

    let idem = parse_idempotency(&headers)?;
    let user = UserId::parse(&body.user_id)?;
    let body_value =
        serde_json::to_value(&body).map_err(|_| ApiError::Internal("invalid request body"))?;
    let scope = "admin:hold_authorize";
    let request_hash = idempotency_http::request_hash(&body_value, uri.path(), "admin");
    let req: HoldRequestValidated = body.try_into()?;

    let idem_key = idem.clone();
    let user_id = user.clone();
    let res = idempotency_http::execute(&st.pool, scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::payment_hold_authorize(tx, &user_id, &idem_key, req).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::post_op_response_by_tx_id(&st.pool, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/holds/{tx_id}/capture",
    params(("tx_id" = String, Path, description = "Hold transaction ID")),
    responses(
        (status = 200, description = "Captured hold (posted payment)", body = PostOpResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Hold not found"),
        (status = 409, description = "Invalid state transition"),
    )
)]
pub(crate) async fn admin_hold_capture(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Path(tx_id): Path<String>,
) -> ApiResult<Response> {
    require_admin(&auth)?;
    let idem = parse_idempotency(&headers)?;
    let tx_id = wallet_domain::TxId::parse(&tx_id)?;
    let scope = "admin:hold_capture";
    let request_hash = idempotency_http::request_hash(&json!({}), uri.path(), "admin");

    #[allow(clippy::clone_on_copy)]
    let tx_id_copy = tx_id.clone();
    let res = idempotency_http::execute(&st.pool, scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::payment_hold_capture(tx, &tx_id_copy).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::post_op_response_by_tx_id(&st.pool, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/holds/{tx_id}/cancel",
    params(("tx_id" = String, Path, description = "Hold transaction ID")),
    responses(
        (status = 200, description = "Canceled (reversed) hold", body = PostOpResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Hold not found"),
        (status = 409, description = "Invalid state transition"),
    )
)]
pub(crate) async fn admin_hold_cancel(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Path(tx_id): Path<String>,
) -> ApiResult<Response> {
    require_admin(&auth)?;
    let idem = parse_idempotency(&headers)?;
    let tx_id = wallet_domain::TxId::parse(&tx_id)?;
    let scope = "admin:hold_cancel";
    let request_hash = idempotency_http::request_hash(&json!({}), uri.path(), "admin");

    #[allow(clippy::clone_on_copy)]
    let tx_id_copy = tx_id.clone();
    let res = idempotency_http::execute(&st.pool, scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::payment_hold_cancel(tx, &tx_id_copy).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::post_op_response_by_tx_id(&st.pool, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/payments/{tx_id}/refund",
    params(("tx_id" = String, Path, description = "Payment transaction ID")),
    responses(
        (status = 200, description = "Refunded payment", body = RefundResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Payment not found"),
        (status = 409, description = "Invalid state transition"),
    )
)]
pub(crate) async fn admin_payment_refund(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    Path(tx_id): Path<String>,
) -> ApiResult<Response> {
    require_admin(&auth)?;
    let idem = parse_idempotency(&headers)?;
    let tx_id = wallet_domain::TxId::parse(&tx_id)?;
    let scope = "admin:payment_refund";
    let request_hash = idempotency_http::request_hash(&json!({}), uri.path(), "admin");

    #[allow(clippy::clone_on_copy)]
    let tx_id_copy = tx_id.clone();
    let res = idempotency_http::execute(&st.pool, scope, &idem, &request_hash, |tx| {
        Box::pin(async move { service::payment_refund(tx, &tx_id_copy).await })
    })
    .await?;

    match res {
        HttpIdempotencyOutcome::Live(body) => Ok((StatusCode::OK, Json(body)).into_response()),
        HttpIdempotencyOutcome::Replay { result_tx_id } => {
            let body = service::refund_response_by_tx_ids(&st.pool, &tx_id, &result_tx_id).await?;
            Ok((StatusCode::OK, Json(body)).into_response())
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/admin/users/{user_id}/balances",
    params(("user_id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "Balances", body = ListBalancesResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found"),
    )
)]
pub(crate) async fn admin_user_balances(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<ListBalancesResponse>> {
    require_admin(&auth)?;

    let user = UserId::parse(&user_id)?;
    let items = service::list_balances(&st.pool, &user).await?;

    Ok(Json(ListBalancesResponse { balances: items }))
}

#[utoipa::path(
    get,
    path = "/v1/admin/users/{user_id}/txs",
    params(
        ("user_id" = String, Path, description = "User ID"),
        ListTxsQuery
    ),
    responses(
        (status = 200, description = "Transaction list", body = ListTxsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "User not found"),
    )
)]
pub(crate) async fn admin_user_txs(
    State(st): State<AppState>,
    Extension(auth): Extension<AuthCtx>,
    Path(user_id): Path<String>,
    Query(q): Query<ListTxsQuery>,
) -> ApiResult<Json<ListTxsResponse>> {
    require_admin(&auth)?;

    let user = UserId::parse(&user_id)?;
    let limit = q.limit.unwrap_or(50).min(200);
    let items = service::list_txs(&st.pool, &user, limit).await?;

    Ok(Json(ListTxsResponse { txs: items }))
}
