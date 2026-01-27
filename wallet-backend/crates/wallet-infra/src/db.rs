use chrono::Utc;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::{path::PathBuf, time::Duration};

use wallet_app::{AppError, AppResult};
use wallet_domain::*;


/// Creates SQLite pool for file `db_file` placed next to Cargo.toml.
///
/// Why so strict:
/// - Windows path parsing for `sqlite://...` is easy to get wrong.
/// - We want stable location: `<repo>/wallet-backend/wallet.db`.
/// - We enable WAL + busy_timeout to reduce "database is locked" during concurrent writes.
pub async fn create_pool(db_file: &str) -> anyhow::Result<SqlitePool> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
let workspace_root = manifest_dir
    .parent()
    .and_then(|p| p.parent())
    .map(|p| p.to_path_buf())
    .unwrap_or_else(|| manifest_dir.clone());

let abs_path: PathBuf = workspace_root.join(db_file);

    let opts = SqliteConnectOptions::new()
        .filename(&abs_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(opts)
        .await?;

    // Extra pragmas (cheap + safe)
    sqlx::query("PRAGMA temp_store = MEMORY;")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

#[derive(Debug, Clone)]
pub struct AccountRow {
    pub id: i64,
    pub owner_type: String,
    pub owner_id: String,
    pub status: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub label: String,
}

pub async fn ensure_system_accounts(pool: &SqlitePool) -> AppResult<()> {
    // System account for topups source.
    ensure_account(pool, "system", "cash").await?;

    // System accounts for later features (sink/spend), one pair per supported currency.
    ensure_system_accounts_for_all_currencies(pool).await?;

    // Demo users for local MVP smoke-testing (optional, safe):
    // monetary operations must not create new users implicitly.
    ensure_account(pool, "user", "bob").await?;
    ensure_account(pool, "user", "alice").await?;
    Ok(())
}

/// Ensures SYSTEM_SINK:<CUR> and SYSTEM_SPEND:<CUR> accounts exist for every currency in the registry.
///
/// Deterministic: iterates currencies in alphabetical order.
pub async fn ensure_system_accounts_for_all_currencies(pool: &SqlitePool) -> AppResult<()> {
    let codes = sqlx::query_scalar::<_, String>("SELECT code FROM currencies ORDER BY code ASC")
        .fetch_all(pool)
        .await?;

    for code in codes {
        let currency = Currency::parse(&code).map_err(AppError::from)?;
        ensure_system_account_for_currency(pool, SYSTEM_ACCOUNT_SINK, &currency).await?;
        ensure_system_account_for_currency(pool, SYSTEM_ACCOUNT_SPEND, &currency).await?;
    }

    Ok(())
}

/// Returns supported currencies from registry in deterministic (alphabetical) order.
pub async fn list_currencies(pool: &SqlitePool) -> AppResult<Vec<(String, i64)>> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT code, minor_units FROM currencies ORDER BY code ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn is_currency_supported(pool: &SqlitePool, currency: &Currency) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM currencies WHERE code = ?1 LIMIT 1",
    )
    .bind(currency.as_str())
    .fetch_optional(pool)
    .await?
    .is_some();

    Ok(exists)
}

pub async fn ensure_currency_supported(pool: &SqlitePool, currency: &Currency) -> AppResult<()> {
    if !is_currency_supported(pool, currency).await? {
        return Err(AppError::UnsupportedCurrency);
    }
    Ok(())
}

async fn ensure_system_account_for_currency(
    pool: &SqlitePool,
    kind: &str,
    currency: &Currency,
) -> AppResult<i64> {
    let owner_id = system_account_owner_id(kind, currency);
    let id = ensure_account(pool, "system", &owner_id).await?;
    // System accounts may go negative later (e.g. burn/sink), so allow_negative=1.
    ensure_account_currency_with_flags(pool, id, currency.as_str(), 1, 0).await?;
    ensure_projection_row(pool, id, currency.as_str()).await?;
    Ok(id)
}

pub async fn create_user(pool: &SqlitePool, user_id: &str) -> AppResult<i64> {
    let id = ensure_account(pool, "user", user_id).await?;
    Ok(id)
}

pub async fn get_user_account_id(pool: &SqlitePool, user_id: &str) -> AppResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM accounts WHERE owner_type = 'user' AND owner_id = ?1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_user_account_id_or_404(pool: &SqlitePool, user_id: &str) -> AppResult<i64> {
    let id = get_user_account_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound("user not found"))?;
    Ok(id)
}

pub async fn ensure_user_account(pool: &SqlitePool, user_id: &str) -> AppResult<i64> {
    let id = ensure_account(pool, "user", user_id).await?;
    Ok(id)
}

pub async fn get_account_by_id(pool: &SqlitePool, account_id: i64) -> AppResult<AccountRow> {
    let row = sqlx::query_as::<_, (i64, String, String, String, String, Option<String>, String)>(
        "SELECT id, owner_type, owner_id, status, created_at, closed_at, label FROM accounts WHERE id = ?1",
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("account not found"))?;

    Ok(AccountRow {
        id: row.0,
        owner_type: row.1,
        owner_id: row.2,
        status: row.3,
        created_at: row.4,
        closed_at: row.5,
        label: row.6,
    })
}

pub async fn set_account_label(pool: &SqlitePool, account_id: i64, label: &str) -> AppResult<()> {
    // Best-effort update. We keep this simple for MVP.
    sqlx::query("UPDATE accounts SET label = ?1 WHERE id = ?2")
        .bind(label)
        .bind(account_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn close_account(pool: &SqlitePool, account_id: i64) -> AppResult<AccountRow> {
    let now = now_rfc3339();

    // Idempotent: if already closed, keep the first closed_at.
    let updated = sqlx::query(
        "UPDATE accounts
         SET status = 'closed',
             closed_at = COALESCE(closed_at, ?1)
         WHERE id = ?2",
    )
    .bind(&now)
    .bind(account_id)
    .execute(pool)
    .await?;

    if updated.rows_affected() == 0 {
        return Err(AppError::NotFound("account not found"));
    }

    let row = get_account_by_id(pool, account_id).await?;
    // Defensive: ensure closed_at always present when closed.
    if AccountStatus::from_db_str_lossy(&row.status) == Some(AccountStatus::Closed)
        && row.closed_at.is_none()
    {
        sqlx::query("UPDATE accounts SET closed_at = ?1 WHERE id = ?2")
            .bind(&now)
            .bind(account_id)
            .execute(pool)
            .await?;
        return get_account_by_id(pool, account_id).await;
    }

    Ok(row)
}

async fn ensure_account(pool: &SqlitePool, owner_type: &str, owner_id: &str) -> AppResult<i64> {
    // Idempotent create with minimal locking.
    let now = now_rfc3339();
    sqlx::query(
        "INSERT OR IGNORE INTO accounts(owner_type, owner_id, status, created_at)
         VALUES (?1, ?2, 'active', ?3)",
    )
    .bind(owner_type)
    .bind(owner_id)
    .bind(now)
    .execute(pool)
    .await?;

    let id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM accounts WHERE owner_type = ?1 AND owner_id = ?2",
    )
    .bind(owner_type)
    .bind(owner_id)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn has_account_currency(pool: &SqlitePool, account_id: i64, currency: &str) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM account_currency WHERE account_id = ?1 AND currency = ?2 LIMIT 1",
    )
    .bind(account_id)
    .bind(currency)
    .fetch_optional(pool)
    .await?
    .is_some();

    Ok(exists)
}

pub async fn ensure_account_currency(pool: &SqlitePool, account_id: i64, currency: &str) -> AppResult<()> {
    ensure_account_currency_with_flags(pool, account_id, currency, 0, 0).await
}

pub async fn ensure_account_currency_with_flags(
    pool: &SqlitePool,
    account_id: i64,
    currency: &str,
    allow_negative: i64,
    allow_hold: i64,
) -> AppResult<()> {
    let now = now_rfc3339();
    sqlx::query(
        "INSERT OR IGNORE INTO account_currency(account_id, currency, allow_negative, allow_hold, opened_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(account_id)
    .bind(currency)
    .bind(allow_negative)
    .bind(allow_hold)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn ensure_currency_account(
    pool: &SqlitePool,
    root_account_id: i64,
    currency: &str,
) -> AppResult<i64> {
    let now = now_rfc3339();
    // Best-effort: if the row exists already, keep its original created_at/label.
    sqlx::query(
        "INSERT OR IGNORE INTO currency_accounts(root_account_id, currency, status, created_at, label)
         VALUES (?1, ?2, 'active', ?3, '')",
    )
    .bind(root_account_id)
    .bind(currency)
    .bind(now)
    .execute(pool)
    .await?;

    let id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM currency_accounts WHERE root_account_id = ?1 AND currency = ?2",
    )
    .bind(root_account_id)
    .bind(currency)
    .fetch_one(pool)
    .await?;

    Ok(id)
}


pub async fn ensure_projection_row(pool: &SqlitePool, account_id: i64, currency: &str) -> AppResult<()> {
    let now = now_rfc3339();
    sqlx::query(
        "INSERT OR IGNORE INTO balance_projection(account_id, currency, available_minor, hold_minor, updated_at)
         VALUES (?1, ?2, 0, 0, ?3)",
    )
    .bind(account_id)
    .bind(currency)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}
