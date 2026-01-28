use axum::body::Body;
use axum::http::{Request, StatusCode};
use bytes::Bytes;
use http_body_util::BodyExt;
use proptest::prelude::*;
use serde_json::json;
use tempfile::TempDir;
use tower::ServiceExt;

use wallet_api::{app, auth};
use wallet_infra::db;

type TestApp = axum::Router; // Router<()> в axum 0.8

struct TestCtx {
    _dir: TempDir,
    pool: sqlx::SqlitePool,
    app: TestApp,
}

async fn setup() -> TestCtx {
    // Test tokens
    unsafe { std::env::set_var("ADMIN_TOKEN", "admin-test-token") };
    unsafe { std::env::set_var("USER_TOKEN_SECRET", "user-test-secret") };

    let dir = tempfile::tempdir().expect("tempdir");
    let db_path = dir.path().join("wallet.db");

    let opts = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(opts)
        .await
        .expect("connect sqlite");

    // cheap, safe defaults
    sqlx::query("PRAGMA temp_store = MEMORY;")
        .execute(&pool)
        .await
        .unwrap();

    db::migrate(&pool).await.expect("migrate");
    db::ensure_system_accounts(&pool)
        .await
        .expect("ensure_system_accounts");

    let app = app::build_app(pool.clone());

    TestCtx {
        _dir: dir,
        pool,
        app,
    }
}

async fn call(app: TestApp, req: Request<Body>) -> (StatusCode, Bytes) {
    let res = app.oneshot(req).await.expect("oneshot");
    let status = res.status();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, body)
}

fn admin_token() -> &'static str {
    "admin-test-token"
}

fn user_token(user_id: &str) -> String {
    auth::sign_user_token_str(user_id)
}

fn req_get(uri: &str, bearer: Option<String>) -> Request<Body> {
    let mut b = Request::builder().method("GET").uri(uri);
    if let Some(t) = bearer {
        b = b.header("Authorization", format!("Bearer {t}"));
    }
    b.body(Body::empty()).unwrap()
}

fn req_get_local(uri: &str, dev_user: Option<&str>) -> Request<Body> {
    let mut b = Request::builder()
        .method("GET")
        .uri(uri)
        .header("Host", "localhost");
    if let Some(dev_user) = dev_user {
        b = b.header("X-Dev-User", dev_user);
    }
    b.body(Body::empty()).unwrap()
}

fn req_post_json(
    uri: &str,
    bearer: Option<String>,
    idem: Option<&str>,
    body_json: serde_json::Value,
) -> Request<Body> {
    let mut b = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json");

    if let Some(t) = bearer {
        b = b.header("Authorization", format!("Bearer {t}"));
    }

    if let Some(idem) = idem {
        b = b.header("Idempotency-Key", idem);
    }

    b.body(Body::from(body_json.to_string())).unwrap()
}

async fn assert_invariants(pool: &sqlx::SqlitePool) {
    // Invariant 1: For each tx_id, sum(credits) - sum(debits) == 0
    let bad: Option<String> = sqlx::query_scalar(
        r#"
        SELECT e.tx_id
        FROM (
            SELECT
              tx_id,
              SUM(CASE WHEN direction = 'credit' THEN amount_minor ELSE -amount_minor END) AS net
            FROM ledger_entries
            GROUP BY tx_id
        ) e
        WHERE e.net != 0
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    assert!(
        bad.is_none(),
        "double-entry invariant broken, tx_id={:?}",
        bad
    );

    // Invariant 2: Projection total equals sum of signed ledger entries per account+currency.
    // Total = available + hold.
    let mismatch: Option<(i64, String)> = sqlx::query_as(
        r#"
        SELECT p.account_id, p.currency
        FROM balance_projection p
        JOIN (
            SELECT
              account_id,
              currency,
              SUM(CASE WHEN direction = 'credit' THEN amount_minor ELSE -amount_minor END) AS sum_entries
            FROM ledger_entries
            GROUP BY account_id, currency
        ) s
        ON p.account_id = s.account_id AND p.currency = s.currency
        WHERE (p.available_minor + p.hold_minor) != s.sum_entries
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    assert!(
        mismatch.is_none(),
        "projection invariant broken, account_id/currency={:?}",
        mismatch
    );
}

async fn admin_create_user(app: TestApp, user: &str) {
    let (st, _) = call(
        app,
        req_post_json(
            "/v1/admin/users",
            Some(admin_token().to_string()),
            None,
            json!({"user_id": user}),
        ),
    )
    .await;
    assert!(st == StatusCode::OK || st == StatusCode::CONFLICT);
}

/// Compatibility helper for older/newer tests.
///
/// Some test cases were written against `*_ok` helpers, while the file currently defines
/// `admin_create_user` (idempotent: OK or CONFLICT). Both meanings are fine for our e2e.
async fn admin_create_user_ok(app: TestApp, user: &str) {
    admin_create_user(app, user).await;
}

/// Compatibility helper for tests that used the older name.
async fn admin_open_currency_account_ok(app: TestApp, user: &str, currency: &str) {
    admin_open_currency(app, user, currency).await;
}

/// Compatibility helper for tests that used a different response shape.
///
/// Current `/v1/wallet/{user}/balances` returns `{ balances: [...] }`.
/// Some new tests expect `{ items: [...] }`.
async fn wallet_balances_ok(app: TestApp, user: &str) -> serde_json::Value {
    let v = balances(app, user).await;
    if v.get("items").is_some() {
        v
    } else {
        json!({"items": v["balances"].clone()})
    }
}

async fn admin_open_currency(app: TestApp, user: &str, currency: &str) {
    let (st, _) = call(
        app,
        req_post_json(
            &format!("/v1/admin/users/{user}/accounts"),
            Some(admin_token().to_string()),
            None,
            json!({"currency": currency}),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
}

async fn admin_topup(
    app: TestApp,
    user: &str,
    currency: &str,
    amount: i64,
    idem: &str,
) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            "/v1/admin/topup",
            Some(admin_token().to_string()),
            Some(idem),
            json!({"user_id": user, "currency": currency, "amount_minor": amount}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn admin_hold(
    app: TestApp,
    user: &str,
    currency: &str,
    amount: i64,
    idem: &str,
) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            "/v1/admin/holds",
            Some(admin_token().to_string()),
            Some(idem),
            json!({"user_id": user, "currency": currency, "amount_minor": amount}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn admin_capture(app: TestApp, tx_id: &str, idem: &str) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            &format!("/v1/admin/holds/{tx_id}/capture"),
            Some(admin_token().to_string()),
            Some(idem),
            json!({}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn admin_cancel(app: TestApp, tx_id: &str, idem: &str) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            &format!("/v1/admin/holds/{tx_id}/cancel"),
            Some(admin_token().to_string()),
            Some(idem),
            json!({}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn admin_refund(app: TestApp, tx_id: &str, idem: &str) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            &format!("/v1/admin/payments/{tx_id}/refund"),
            Some(admin_token().to_string()),
            Some(idem),
            json!({}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn transfer(
    app: TestApp,
    from: &str,
    to: &str,
    currency: &str,
    amount: i64,
    idem: &str,
) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            &format!("/v1/wallet/{from}/transfer"),
            Some(user_token(from)),
            Some(idem),
            json!({"to_user_id": to, "currency": currency, "amount_minor": amount}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn balances(app: TestApp, user: &str) -> serde_json::Value {
    let (st, body) = call(
        app,
        req_get(
            &format!("/v1/wallet/{user}/balances"),
            Some(user_token(user)),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    serde_json::from_slice(&body).unwrap()
}

async fn txs(app: TestApp, user: &str, limit: usize) -> serde_json::Value {
    let (st, body) = call(
        app,
        req_get(
            &format!("/v1/wallet/{user}/txs?limit={limit}"),
            Some(user_token(user)),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    serde_json::from_slice(&body).unwrap()
}

async fn tx_receipt(
    app: TestApp,
    bearer: Option<String>,
    tx_id: &str,
) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(app, req_get(&format!("/v1/transactions/{tx_id}"), bearer)).await;
    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

#[tokio::test]
async fn health_ok() {
    let ctx = setup().await;

    let (st, body) = call(ctx.app.clone(), req_get("/health", None)).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body, Bytes::from_static(b"ok"));
}

#[tokio::test]
async fn topup_updates_balance_and_history() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "42").await;
    admin_open_currency(ctx.app.clone(), "42", "UZS").await;

    let (st, top) = admin_topup(ctx.app.clone(), "42", "UZS", 100_000, "topup-42-1").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(top["state"], "posted");
    assert_eq!(top["amount_minor"], 100_000);

    let b = balances(ctx.app.clone(), "42").await;
    assert_eq!(b["balances"][0]["available_minor"], 100_000);

    let t = txs(ctx.app.clone(), "42", 50).await;
    assert_eq!(t["txs"][0]["tx_type"], "topup");
    assert_eq!(t["txs"][0]["amount_minor"], 100_000);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn topup_is_idempotent() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "101").await;
    admin_open_currency(ctx.app.clone(), "101", "UZS").await;

    let (st1, top1) = admin_topup(ctx.app.clone(), "101", "UZS", 25_000, "topup-101-1").await;
    assert_eq!(st1, StatusCode::OK);

    let (st2, top2) = admin_topup(ctx.app.clone(), "101", "UZS", 25_000, "topup-101-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(top1, top2);

    let b = balances(ctx.app.clone(), "101").await;
    assert_eq!(b["balances"][0]["available_minor"], 25_000);

    let t = txs(ctx.app.clone(), "101", 10).await;
    assert_eq!(t["txs"].as_array().unwrap().len(), 1);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn currency_is_normalized_to_uppercase() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "42").await;
    admin_open_currency(ctx.app.clone(), "42", "UZS").await;

    let (st, top) = admin_topup(ctx.app.clone(), "42", "uzs", 10_000, "topup-42-lower").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(top["currency"], "UZS");

    let b = balances(ctx.app.clone(), "42").await;
    assert_eq!(b["balances"][0]["currency"], "UZS");

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn transfer_moves_funds_and_is_idempotent() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "alice").await;
    admin_create_user(ctx.app.clone(), "bob").await;
    admin_open_currency(ctx.app.clone(), "alice", "UZS").await;
    admin_open_currency(ctx.app.clone(), "bob", "UZS").await;

    // Seed alice
    let _ = admin_topup(ctx.app.clone(), "alice", "UZS", 50_000, "seed-alice").await;

    let (st, tr) = transfer(ctx.app.clone(), "alice", "bob", "UZS", 20_000, "t-1").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(tr["state"], "posted");
    assert_eq!(tr["amount_minor"], 20_000);

    // Idempotent repeat
    let (st2, tr2) = transfer(ctx.app.clone(), "alice", "bob", "UZS", 20_000, "t-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(tr2, tr);

    let a = balances(ctx.app.clone(), "alice").await;
    let b = balances(ctx.app.clone(), "bob").await;

    assert_eq!(a["balances"][0]["available_minor"], 30_000);
    assert_eq!(b["balances"][0]["available_minor"], 20_000);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn transfer_receipt_visible_to_both_parties() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "alice").await;
    admin_create_user(ctx.app.clone(), "bob").await;
    admin_create_user(ctx.app.clone(), "charlie").await;
    admin_open_currency(ctx.app.clone(), "alice", "UZS").await;
    admin_open_currency(ctx.app.clone(), "bob", "UZS").await;
    admin_open_currency(ctx.app.clone(), "charlie", "UZS").await;

    let _ = admin_topup(
        ctx.app.clone(),
        "alice",
        "UZS",
        50_000,
        "seed-alice-receipt",
    )
    .await;

    let (st, tr) = transfer(
        ctx.app.clone(),
        "alice",
        "bob",
        "UZS",
        20_000,
        "t-receipt-1",
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    let tx_id = tr["tx_id"].as_str().unwrap().to_string();

    let (st_a, r_a) = tx_receipt(ctx.app.clone(), Some(user_token("alice")), &tx_id).await;
    assert_eq!(st_a, StatusCode::OK);
    assert_eq!(r_a["tx_type"], "transfer");
    assert_eq!(r_a["tx_id"], tx_id);
    assert_eq!(r_a["entries"].as_array().unwrap().len(), 2);

    let mut seen_debit_alice = false;
    let mut seen_credit_bob = false;
    for e in r_a["entries"].as_array().unwrap() {
        let party_id = e["party_id"].as_str().unwrap_or("");
        let direction = e["direction"].as_str().unwrap_or("");
        let amount_minor = e["amount_minor"].as_i64().unwrap_or(0);
        assert_eq!(amount_minor, 20_000);

        if party_id == "alice" && direction == "debit" {
            seen_debit_alice = true;
        }
        if party_id == "bob" && direction == "credit" {
            seen_credit_bob = true;
        }
    }
    assert!(seen_debit_alice, "expected debit entry for alice");
    assert!(seen_credit_bob, "expected credit entry for bob");

    let (st_b, _r_b) = tx_receipt(ctx.app.clone(), Some(user_token("bob")), &tx_id).await;
    assert_eq!(st_b, StatusCode::OK);

    let (st_c, _r_c) = tx_receipt(ctx.app.clone(), Some(user_token("charlie")), &tx_id).await;
    assert_eq!(st_c, StatusCode::NOT_FOUND);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn http_idempotency_conflict_on_payload_change() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "99").await;
    admin_open_currency(ctx.app.clone(), "99", "UZS").await;

    let (st1, body1) = admin_topup(ctx.app.clone(), "99", "UZS", 10_000, "topup-99-1").await;
    assert_eq!(st1, StatusCode::OK);

    let (st2, body2) = admin_topup(ctx.app.clone(), "99", "UZS", 10_000, "topup-99-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(body2, body1);

    let (st3, body3) = admin_topup(ctx.app.clone(), "99", "UZS", 20_000, "topup-99-1").await;
    assert_eq!(st3, StatusCode::CONFLICT);
    assert_eq!(
        body3["error"],
        "idempotency key reused with different request"
    );
}

#[tokio::test]
async fn hold_capture_refund_flow_updates_available_and_hold() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u1").await;
    admin_open_currency(ctx.app.clone(), "u1", "UZS").await;

    let _ = admin_topup(ctx.app.clone(), "u1", "UZS", 100_000, "seed-u1").await;

    // Authorize hold
    let (st, h) = admin_hold(ctx.app.clone(), "u1", "UZS", 30_000, "hold-u1-1").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(h["state"], "authorized");
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let b1 = balances(ctx.app.clone(), "u1").await;
    assert_eq!(b1["balances"][0]["available_minor"], 70_000);
    assert_eq!(b1["balances"][0]["hold_minor"], 30_000);

    // Capture
    let (st2, c) = admin_capture(ctx.app.clone(), &tx_id, "cap-u1-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(c["state"], "posted");
    assert_eq!(c["tx_id"], tx_id);

    let b2 = balances(ctx.app.clone(), "u1").await;
    assert_eq!(b2["balances"][0]["available_minor"], 70_000);
    assert_eq!(b2["balances"][0]["hold_minor"], 0);

    // Refund
    let (st3, r) = admin_refund(ctx.app.clone(), &tx_id, "refund-u1-1").await;
    assert_eq!(st3, StatusCode::OK);
    assert_eq!(r["state"], "refunded");
    assert_eq!(r["original_tx_id"], tx_id);
    assert!(!r["refund_tx_id"].as_str().unwrap().is_empty());

    let b3 = balances(ctx.app.clone(), "u1").await;
    assert_eq!(b3["balances"][0]["available_minor"], 100_000);
    assert_eq!(b3["balances"][0]["hold_minor"], 0);

    let t = txs(ctx.app.clone(), "u1", 10).await;
    // Latest should be refund credit, then original payment (refunded)
    assert_eq!(t["txs"][0]["tx_type"], "refund");
    assert_eq!(t["txs"][0]["amount_minor"], 30_000);
    assert_eq!(t["txs"][1]["tx_type"], "payment");
    assert_eq!(t["txs"][1]["state"], "refunded");

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn hold_authorize_is_idempotent() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u3").await;
    admin_open_currency(ctx.app.clone(), "u3", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u3", "UZS", 40_000, "seed-u3").await;

    let (st1, h1) = admin_hold(ctx.app.clone(), "u3", "UZS", 15_000, "hold-u3-1").await;
    assert_eq!(st1, StatusCode::OK);

    let (st2, h2) = admin_hold(ctx.app.clone(), "u3", "UZS", 15_000, "hold-u3-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(h1, h2);

    let b = balances(ctx.app.clone(), "u3").await;
    assert_eq!(b["balances"][0]["available_minor"], 25_000);
    assert_eq!(b["balances"][0]["hold_minor"], 15_000);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn authorized_hold_receipt_is_visible_to_user_even_without_entries() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u1").await;
    admin_open_currency(ctx.app.clone(), "u1", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u1", "UZS", 100_000, "seed-u1-receipt").await;

    let (st, h) = admin_hold(ctx.app.clone(), "u1", "UZS", 30_000, "hold-u1-receipt-1").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(h["state"], "authorized");
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let (st_r, r) = tx_receipt(ctx.app.clone(), Some(user_token("u1")), &tx_id).await;
    assert_eq!(st_r, StatusCode::OK);
    assert_eq!(r["tx_id"], tx_id);
    assert_eq!(r["tx_type"], "payment");
    assert_eq!(r["state"], "authorized");
    assert_eq!(r["currency"], "UZS");
    assert_eq!(r["amount_minor"], 30_000);

    // In MVP: authorized holds may have no ledger_entries yet.
    assert!(r["entries"].as_array().unwrap().is_empty());

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn hold_can_be_canceled_without_posting_entries() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u2").await;
    admin_open_currency(ctx.app.clone(), "u2", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u2", "UZS", 50_000, "seed-u2").await;

    let (st, h) = admin_hold(ctx.app.clone(), "u2", "UZS", 10_000, "hold-u2-1").await;
    assert_eq!(st, StatusCode::OK);
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let b1 = balances(ctx.app.clone(), "u2").await;
    assert_eq!(b1["balances"][0]["available_minor"], 40_000);
    assert_eq!(b1["balances"][0]["hold_minor"], 10_000);

    let (st2, c) = admin_cancel(ctx.app.clone(), &tx_id, "cancel-u2-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(c["state"], "reversed");

    let b2 = balances(ctx.app.clone(), "u2").await;
    assert_eq!(b2["balances"][0]["available_minor"], 50_000);
    assert_eq!(b2["balances"][0]["hold_minor"], 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn idempotency_parallel_requests_apply_once() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "par").await;
    admin_open_currency(ctx.app.clone(), "par", "UZS").await;

    let app1 = ctx.app.clone();
    let app2 = ctx.app.clone();
    let (r1, r2) = tokio::join!(
        admin_topup(app1, "par", "UZS", 12_000, "topup-par-1"),
        admin_topup(app2, "par", "UZS", 12_000, "topup-par-1")
    );

    let statuses = [r1.0, r2.0];
    assert!(statuses.contains(&StatusCode::OK));
    if statuses.iter().all(|s| *s == StatusCode::OK) {
        assert_eq!(r1.1, r2.1);
    } else {
        assert!(statuses.contains(&StatusCode::CONFLICT));
        let ok_body = if r1.0 == StatusCode::OK { r1.1 } else { r2.1 };
        let (st_retry, retry_body) =
            admin_topup(ctx.app.clone(), "par", "UZS", 12_000, "topup-par-1").await;
        assert_eq!(st_retry, StatusCode::OK);
        assert_eq!(retry_body, ok_body);
    }

    let b = balances(ctx.app.clone(), "par").await;
    assert_eq!(b["balances"][0]["available_minor"], 12_000);

    let t = txs(ctx.app.clone(), "par", 10).await;
    assert_eq!(t["txs"].as_array().unwrap().len(), 1);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn transfer_rejects_insufficient_funds() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "alice2").await;
    admin_create_user(ctx.app.clone(), "bob2").await;
    admin_open_currency(ctx.app.clone(), "alice2", "UZS").await;
    admin_open_currency(ctx.app.clone(), "bob2", "UZS").await;

    let _ = admin_topup(ctx.app.clone(), "alice2", "UZS", 10_000, "seed-alice2").await;

    let (st, err) = transfer(
        ctx.app.clone(),
        "alice2",
        "bob2",
        "UZS",
        20_000,
        "t-alice2-1",
    )
    .await;
    assert_eq!(st, StatusCode::CONFLICT);
    assert_eq!(err["error"], "insufficient funds");

    let a = balances(ctx.app.clone(), "alice2").await;
    let b = balances(ctx.app.clone(), "bob2").await;

    assert_eq!(a["balances"][0]["available_minor"], 10_000);
    assert_eq!(b["balances"][0]["available_minor"], 0);

    let ta = txs(ctx.app.clone(), "alice2", 10).await;
    let tb = txs(ctx.app.clone(), "bob2", 10).await;
    assert_eq!(ta["txs"].as_array().unwrap().len(), 1);
    assert_eq!(tb["txs"].as_array().unwrap().len(), 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn transfer_idempotency_conflict_on_payload_change() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "alice3").await;
    admin_create_user(ctx.app.clone(), "bob3").await;
    admin_open_currency(ctx.app.clone(), "alice3", "UZS").await;
    admin_open_currency(ctx.app.clone(), "bob3", "UZS").await;

    let _ = admin_topup(ctx.app.clone(), "alice3", "UZS", 50_000, "seed-alice3").await;

    let (st1, ok1) = transfer(
        ctx.app.clone(),
        "alice3",
        "bob3",
        "UZS",
        20_000,
        "t-alice3-1",
    )
    .await;
    assert_eq!(st1, StatusCode::OK);

    let (st2, ok2) = transfer(
        ctx.app.clone(),
        "alice3",
        "bob3",
        "UZS",
        20_000,
        "t-alice3-1",
    )
    .await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(ok2, ok1);

    let (st3, err) = transfer(
        ctx.app.clone(),
        "alice3",
        "bob3",
        "UZS",
        25_000,
        "t-alice3-1",
    )
    .await;
    assert_eq!(st3, StatusCode::CONFLICT);
    assert_eq!(
        err["error"],
        "idempotency key reused with different request"
    );

    let a = balances(ctx.app.clone(), "alice3").await;
    let b = balances(ctx.app.clone(), "bob3").await;

    assert_eq!(a["balances"][0]["available_minor"], 30_000);
    assert_eq!(b["balances"][0]["available_minor"], 20_000);

    let ta = txs(ctx.app.clone(), "alice3", 10).await;
    assert_eq!(ta["txs"].as_array().unwrap().len(), 2);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn hold_capture_is_idempotent() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u5").await;
    admin_open_currency(ctx.app.clone(), "u5", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u5", "UZS", 100_000, "seed-u5").await;

    let (st, h) = admin_hold(ctx.app.clone(), "u5", "UZS", 30_000, "hold-u5-1").await;
    assert_eq!(st, StatusCode::OK);
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let (st2, c1) = admin_capture(ctx.app.clone(), &tx_id, "cap-u5-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(c1["state"], "posted");

    let (st3, c2) = admin_capture(ctx.app.clone(), &tx_id, "cap-u5-1").await;
    assert_eq!(st3, StatusCode::OK);
    assert_eq!(c2, c1);

    let b = balances(ctx.app.clone(), "u5").await;
    assert_eq!(b["balances"][0]["available_minor"], 70_000);
    assert_eq!(b["balances"][0]["hold_minor"], 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn hold_cannot_be_canceled_after_capture() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u6").await;
    admin_open_currency(ctx.app.clone(), "u6", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u6", "UZS", 50_000, "seed-u6").await;

    let (st, h) = admin_hold(ctx.app.clone(), "u6", "UZS", 10_000, "hold-u6-1").await;
    assert_eq!(st, StatusCode::OK);
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let (st2, c) = admin_capture(ctx.app.clone(), &tx_id, "cap-u6-1").await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(c["state"], "posted");

    let (st3, err) = admin_cancel(ctx.app.clone(), &tx_id, "cancel-u6-1").await;
    assert_eq!(st3, StatusCode::CONFLICT);
    assert_eq!(err["error"], "tx is not in authorized state");

    let b = balances(ctx.app.clone(), "u6").await;
    assert_eq!(b["balances"][0]["available_minor"], 40_000);
    assert_eq!(b["balances"][0]["hold_minor"], 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn refund_requires_posted_payment() {
    let ctx = setup().await;

    admin_create_user(ctx.app.clone(), "u7").await;
    admin_open_currency(ctx.app.clone(), "u7", "UZS").await;
    let _ = admin_topup(ctx.app.clone(), "u7", "UZS", 60_000, "seed-u7").await;

    // only authorize hold (payment not posted yet)
    let (st, h) = admin_hold(ctx.app.clone(), "u7", "UZS", 5_000, "hold-u7-1").await;
    assert_eq!(st, StatusCode::OK);
    let tx_id = h["tx_id"].as_str().unwrap().to_string();

    let (st2, err) = admin_refund(ctx.app.clone(), &tx_id, "refund-u7-1").await;
    assert_eq!(st2, StatusCode::CONFLICT);
    assert_eq!(err["error"], "tx is not in posted state");

    let b = balances(ctx.app.clone(), "u7").await;
    assert_eq!(b["balances"][0]["available_minor"], 55_000);
    assert_eq!(b["balances"][0]["hold_minor"], 5_000);

    assert_invariants(&ctx.pool).await;
}

async fn admin_create_account_v2(
    app: TestApp,
    owner_user_id: &str,
    currency: &str,
    label: &str,
) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            "/v1/admin/accounts",
            Some(admin_token().to_string()),
            None,
            json!({"owner_user_id": owner_user_id, "currency": currency, "label": label}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn admin_close_account_v2(app: TestApp, account_id: &str) -> (StatusCode, serde_json::Value) {
    let (st, body) = call(
        app,
        req_post_json(
            &format!("/v1/admin/accounts/{account_id}/close"),
            Some(admin_token().to_string()),
            None,
            json!({}),
        ),
    )
    .await;

    let v: serde_json::Value = serde_json::from_slice(&body)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body)}));
    (st, v)
}

async fn count_ledger_txs(pool: &sqlx::SqlitePool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM ledger_transactions")
        .fetch_one(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn admin_create_account_ok() {
    let ctx = setup().await;

    let (st, acc) =
        admin_create_account_v2(ctx.app.clone(), "user_123", "UZS", "My UZS Wallet").await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(acc["status"], "ACTIVE");
    assert_eq!(acc["currency"], "UZS");
    assert_eq!(acc["owner_user_id"], "user_123");
    assert!(acc["account_id"].as_str().unwrap().starts_with("acc_"));

    let b = balances(ctx.app.clone(), "user_123").await;
    assert_eq!(b["balances"][0]["currency"], "UZS");
    assert_eq!(b["balances"][0]["available_minor"], 0);
    assert_eq!(b["balances"][0]["hold_minor"], 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn admin_close_account_ok() {
    let ctx = setup().await;

    let (_st, acc) = admin_create_account_v2(ctx.app.clone(), "user_124", "UZS", "Wallet").await;
    let account_id = acc["account_id"].as_str().unwrap().to_string();

    let (st2, closed1) = admin_close_account_v2(ctx.app.clone(), &account_id).await;
    assert_eq!(st2, StatusCode::OK);
    assert_eq!(closed1["status"], "CLOSED");
    let closed_at_1 = closed1["closed_at"].as_str().unwrap().to_string();

    // idempotent close
    let (st3, closed2) = admin_close_account_v2(ctx.app.clone(), &account_id).await;
    assert_eq!(st3, StatusCode::OK);
    assert_eq!(closed2["status"], "CLOSED");
    let closed_at_2 = closed2["closed_at"].as_str().unwrap().to_string();
    assert_eq!(closed_at_1, closed_at_2);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn closed_account_rejects_transfer() {
    let ctx = setup().await;

    let (_st1, _a1) = admin_create_account_v2(ctx.app.clone(), "alice_x", "UZS", "A").await;
    let (_st2, _a2) = admin_create_account_v2(ctx.app.clone(), "bob_x", "UZS", "B").await;

    // Seed funds BEFORE closing.
    let (st3, _) = admin_topup(ctx.app.clone(), "alice_x", "UZS", 50_000, "seed-alice-x").await;
    assert_eq!(st3, StatusCode::OK);

    // Close sender.
    let (_st_acc, acc) = admin_create_account_v2(ctx.app.clone(), "alice_x", "UZS", "A").await;
    let alice_account_id = acc["account_id"].as_str().unwrap().to_string();
    let (st_close, _) = admin_close_account_v2(ctx.app.clone(), &alice_account_id).await;
    assert_eq!(st_close, StatusCode::OK);

    let before = count_ledger_txs(&ctx.pool).await;

    // Transfer must fail with ACCOUNT_CLOSED, and must not create new ledger tx.
    let (st4, body4) = call(
        ctx.app.clone(),
        req_post_json(
            "/v1/wallet/alice_x/transfer",
            Some(user_token("alice_x")),
            Some("transfer-closed-1"),
            json!({"to_user_id": "bob_x", "currency": "UZS", "amount_minor": 10_000}),
        ),
    )
    .await;
    assert_eq!(st4, StatusCode::CONFLICT);
    let v: serde_json::Value = serde_json::from_slice(&body4)
        .unwrap_or_else(|_| json!({"raw": String::from_utf8_lossy(&body4)}));
    assert_eq!(v["code"], "ACCOUNT_CLOSED");

    let after = count_ledger_txs(&ctx.pool).await;
    assert_eq!(before, after);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn system_accounts_exist() {
    let ctx = setup().await;

    let sink_id: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM accounts WHERE owner_type = 'system' AND owner_id = 'SYSTEM_SINK:UZS'",
    )
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    let spend_id: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM accounts WHERE owner_type = 'system' AND owner_id = 'SYSTEM_SPEND:UZS'",
    )
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    assert!(sink_id.is_some(), "SYSTEM_SINK:UZS missing");
    assert!(spend_id.is_some(), "SYSTEM_SPEND:UZS missing");

    for id in [sink_id.unwrap(), spend_id.unwrap()] {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(1) FROM account_currency WHERE account_id = ?1 AND currency = 'UZS'",
        )
        .bind(id)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
        assert_eq!(n, 1);
    }

    assert_invariants(&ctx.pool).await;
}

proptest! {
    #[test]
    fn user_token_format_is_stable(user_id in "[a-zA-Z0-9_-]{1,16}") {
        unsafe{ std::env::set_var("USER_TOKEN_SECRET", "user-test-secret"); }
        let t = auth::sign_user_token_str(&user_id);
        // NOTE: use positional formatting here because `proptest!` expands the format string from a macro.
        let prefix = format!("user:{}:", user_id);
        prop_assert!(t.starts_with(&prefix));

    }
}

#[tokio::test]
async fn currencies_list_ok() {
    let ctx = setup().await;

    let (st, body) = call(ctx.app.clone(), req_get("/v1/currencies", None)).await;
    assert_eq!(st, StatusCode::OK);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = v["items"].as_array().unwrap();

    // Deterministic ordering: alphabetical by code
    let codes: Vec<String> = items
        .iter()
        .map(|it| it["code"].as_str().unwrap().to_string())
        .collect();
    let mut sorted = codes.clone();
    sorted.sort();
    assert_eq!(codes, sorted);

    // Seeded currency must exist with correct minor units
    let uzs = items
        .iter()
        .find(|it| it["code"].as_str().unwrap() == "UZS")
        .expect("UZS missing");
    assert_eq!(uzs["minor_units"].as_i64().unwrap(), 0);

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn unsupported_currency_rejected() {
    let ctx = setup().await;

    admin_create_user_ok(ctx.app.clone(), "u-unsupported").await;
    admin_open_currency_account_ok(ctx.app.clone(), "u-unsupported", "UZS").await;

    let before = count_ledger_txs(&ctx.pool).await;

    let (st, body) = call(
        ctx.app.clone(),
        req_post_json(
            "/v1/admin/users/u-unsupported/accounts",
            Some(admin_token().to_string()),
            None,
            json!({"currency":"XXX"}),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::BAD_REQUEST);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["code"].as_str().unwrap(), "UNSUPPORTED_CURRENCY");

    let after = count_ledger_txs(&ctx.pool).await;
    assert_eq!(
        before, after,
        "ledger must not change on validation failure"
    );

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn system_accounts_seeded_for_each_currency() {
    let ctx = setup().await;

    let codes: Vec<String> = sqlx::query_scalar("SELECT code FROM currencies ORDER BY code ASC")
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

    assert!(!codes.is_empty());

    for code in codes {
        let sink_owner = format!("SYSTEM_SINK:{code}");
        let spend_owner = format!("SYSTEM_SPEND:{code}");

        let sink_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM accounts WHERE owner_type = 'system' AND owner_id = ?1",
        )
        .bind(&sink_owner)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();

        let spend_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM accounts WHERE owner_type = 'system' AND owner_id = ?1",
        )
        .bind(&spend_owner)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();

        assert!(sink_id.is_some(), "{sink_owner} missing");
        assert!(spend_id.is_some(), "{spend_owner} missing");
    }

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn currency_registry_contains_core_codes() {
    let ctx = setup().await;

    let codes: Vec<String> = sqlx::query_scalar("SELECT code FROM currencies ORDER BY code ASC")
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

    for code in ["RUB", "UZS", "USD", "EUR"] {
        assert!(
            codes.iter().any(|row| row == code),
            "missing currency {code}"
        );
    }
}

#[tokio::test]
async fn supported_currency_allows_money_op() {
    let ctx = setup().await;

    admin_create_user_ok(ctx.app.clone(), "u-money").await;
    admin_open_currency_account_ok(ctx.app.clone(), "u-money", "UZS").await;

    let (st, _v) = admin_topup(ctx.app.clone(), "u-money", "UZS", 1000, "idem-topup-uzs").await;
    assert_eq!(st, StatusCode::OK);

    let balances = wallet_balances_ok(ctx.app.clone(), "u-money").await;
    assert_eq!(balances["items"][0]["currency"].as_str().unwrap(), "UZS");
    assert_eq!(
        balances["items"][0]["available_minor"].as_i64().unwrap(),
        1000
    );

    assert_invariants(&ctx.pool).await;
}

#[tokio::test]
async fn admin_list_users_returns_users() {
    let ctx = setup().await;

    admin_create_user_ok(ctx.app.clone(), "u02").await;
    admin_create_user_ok(ctx.app.clone(), "u01").await;

    let u02_account_id = db::get_user_account_id_or_404(&ctx.pool, "u02")
        .await
        .unwrap();
    db::set_account_label(&ctx.pool, u02_account_id, "User Two")
        .await
        .unwrap();

    let (st, body) = call(
        ctx.app.clone(),
        req_get("/v1/admin/users", Some(admin_token().to_string())),
    )
    .await;
    assert_eq!(st, StatusCode::OK);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let users = v["users"].as_array().unwrap();
    assert!(users.windows(2).all(|pair| {
        let left = pair[0]["user_id"].as_str().unwrap();
        let right = pair[1]["user_id"].as_str().unwrap();
        left <= right
    }));

    let u01 = users
        .iter()
        .find(|user| user["user_id"].as_str() == Some("u01"))
        .unwrap();
    let u02 = users
        .iter()
        .find(|user| user["user_id"].as_str() == Some("u02"))
        .unwrap();

    assert_eq!(u01["display_name"].as_str().unwrap(), "u01");
    assert_eq!(u02["display_name"].as_str().unwrap(), "User Two");
}

#[tokio::test]
async fn admin_list_users_forbidden_for_user_token() {
    let ctx = setup().await;

    let (st, _body) = call(
        ctx.app.clone(),
        req_get("/v1/admin/users", Some(user_token("u01"))),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn localhost_wallet_path_user_uses_selected_user() {
    let ctx = setup().await;

    admin_create_user_ok(ctx.app.clone(), "u02").await;
    admin_open_currency_account_ok(ctx.app.clone(), "u02", "UZS").await;
    let (st, _) = admin_topup(ctx.app.clone(), "u02", "UZS", 700, "idem-dev-topup").await;
    assert_eq!(st, StatusCode::OK);

    let (st, body) = call(
        ctx.app.clone(),
        req_get_local("/v1/wallet/u02/balances?as=u02", None),
    )
    .await;
    assert_eq!(st, StatusCode::OK);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["balances"][0]["currency"].as_str().unwrap(), "UZS");
}

#[tokio::test]
async fn localhost_wallet_path_user_mismatch_rejected() {
    let ctx = setup().await;

    let (st, body) = call(
        ctx.app.clone(),
        req_get_local("/v1/wallet/u02/balances", Some("u01")),
    )
    .await;
    assert_eq!(st, StatusCode::BAD_REQUEST);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        v["error"].as_str().unwrap(),
        "dev user mismatch: path user_id vs selected"
    );
}

#[tokio::test]
async fn localhost_profile_uses_selected_user() {
    let ctx = setup().await;

    let (st, body) = call(ctx.app.clone(), req_get_local("/v1/profile", Some("u03"))).await;
    assert_eq!(st, StatusCode::OK);

    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["phone_number"].as_str().unwrap(), "u03");
}

#[tokio::test]
async fn dev_endpoints_unavailable_off_localhost() {
    let ctx = setup().await;

    let (st, _body) = call(ctx.app.clone(), req_get("/v1/dev/users", None)).await;
    assert_eq!(st, StatusCode::NOT_FOUND);

    let (st, _body) = call(
        ctx.app.clone(),
        req_get("/v1/dev/users", Some(admin_token().to_string())),
    )
    .await;
    assert_eq!(st, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn dev_endpoints_available_on_localhost() {
    let ctx = setup().await;

    let (st, _body) = call(ctx.app.clone(), req_get_local("/v1/dev/users", None)).await;
    assert_eq!(st, StatusCode::OK);
}
