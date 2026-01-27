use serde_json::json;
use sqlx::{Sqlite, SqlitePool, Transaction};

use wallet_domain::core;

use crate::db;
use wallet_app::{AppError, AppResult};
use wallet_app::*;
use wallet_domain::*;


async fn ensure_currency_supported_tx(
    tx: &mut Transaction<'_, Sqlite>,
    currency: &Currency,
) -> AppResult<()> {
    let exists = sqlx::query_scalar::<_, i64>("SELECT 1 FROM currencies WHERE code = ?1 LIMIT 1")
        .bind(currency.as_str())
        .fetch_optional(&mut **tx)
        .await?
        .is_some();

    if !exists {
        return Err(AppError::UnsupportedCurrency);
    }

    Ok(())
}

pub async fn list_balances(pool: &SqlitePool, user_id: &UserId) -> AppResult<Vec<BalanceItem>> {
    let account_id = AccountId::new(db::get_user_account_id_or_404(pool, user_id.as_str()).await?);

    let rows = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT currency, available_minor, hold_minor\n         FROM balance_projection\n         WHERE account_id = ?1\n         ORDER BY currency ASC",
    )
    .bind(account_id.get())
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(currency, available_minor, hold_minor)| BalanceItem {
            currency,
            available_minor,
            hold_minor,
        })
        .collect())
}

pub async fn list_txs(pool: &SqlitePool, user_id: &UserId, limit: u32) -> AppResult<Vec<TxItem>> {
    let account_id = AccountId::new(db::get_user_account_id_or_404(pool, user_id.as_str()).await?);

    // Show txs from the user's perspective: sign is derived from entry direction.
    let rows = sqlx::query_as::<
        _,
        (
            String,         // tx_id
            String,         // tx_type
            String,         // state
            String,         // currency
            i64,            // amount_minor (absolute)
            String,         // direction
            String,         // created_at
            String,         // posted_at
            Option<String>, // metadata_json
        ),
    >(
        "SELECT t.id, t.tx_type, t.state, e.currency, e.amount_minor, e.direction,\n                t.created_at, t.posted_at, t.metadata_json\n         FROM ledger_entries e\n         JOIN ledger_transactions t ON t.id = e.tx_id\n         WHERE e.account_id = ?1\n         ORDER BY t.created_at DESC\n         LIMIT ?2",
    )
    .bind(account_id.get())
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    let mut items: Vec<TxItem> = rows
        .into_iter()
        .map(
            |(
                tx_id,
                tx_type,
                state,
                currency,
                amount_minor,
                direction,
                created_at,
                posted_at,
                meta,
            )|
             -> AppResult<TxItem> {
                let dir = EntryDirection::from_db_str(&direction).map_err(AppError::from)?;
                let signed_amount = dir.apply_sign(amount_minor);

                let description = match TxType::from_db_str_lossy(&tx_type) {
                    Some(TxType::Topup) => "Top up".to_string(),
                    Some(TxType::Transfer) => {
                        // metadata_json contains both ends (best effort)
                        if let Some(m) = meta.as_deref() {
                            if let Ok(v) = serde_json::from_str::<serde_json::Value>(m) {
                                let from = v.get("from_user_id").and_then(|x| x.as_str());
                                let to = v.get("to_user_id").and_then(|x| x.as_str());

                                match dir {
                                    EntryDirection::Debit => to
                                        .map(|t| format!("Transfer to {}", t))
                                        .unwrap_or_else(|| "Transfer".to_string()),
                                    EntryDirection::Credit => from
                                        .map(|f| format!("Transfer from {}", f))
                                        .unwrap_or_else(|| "Incoming transfer".to_string()),
                                }
                            } else if matches!(dir, EntryDirection::Debit) {
                                "Transfer".to_string()
                            } else {
                                "Incoming transfer".to_string()
                            }
                        } else if matches!(dir, EntryDirection::Debit) {
                            "Transfer".to_string()
                        } else {
                            "Incoming transfer".to_string()
                        }
                    }
                    Some(TxType::Payment) => "Payment".to_string(),
                    Some(TxType::Refund) => "Refund".to_string(),
                    None => "Operation".to_string(),
                };

                Ok(TxItem {
                    tx_id,
                    tx_type,
                    state,
                    currency,
                    amount_minor: signed_amount,
                    created_at,
                    posted_at,
                    description,
                })
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    // Include authorized/reversed holds (no ledger entries yet) so the user can see reserved funds.
    let hold_rows = sqlx::query_as::<_, (String, String, String, String, i64, String, String, Option<String>)>(
        "SELECT t.id, t.tx_type, t.state, t.currency, t.amount_minor, t.created_at, t.posted_at, t.metadata_json
         FROM ledger_transactions t
         WHERE t.user_account_id = ?1 AND t.tx_type = 'payment' AND t.state IN ('authorized','reversed')
           AND NOT EXISTS (
                SELECT 1 FROM ledger_entries e WHERE e.tx_id = t.id AND e.account_id = ?1
           )
         ORDER BY t.created_at DESC
         LIMIT ?2",
    )
    .bind(account_id.get())
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    for (tx_id, tx_type, state, currency, amount_minor, created_at, posted_at, _) in hold_rows {
        let desc = match state.as_str() {
            "authorized" => "Hold (authorized)",
            "reversed" => "Hold (reversed)",
            _ => "Hold",
        };

        items.push(TxItem {
            tx_id,
            tx_type,
            state,
            currency,
            amount_minor: -amount_minor,
            created_at,
            posted_at,
            description: desc.to_string(),
        });
    }

    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    items.truncate(limit as usize);
    Ok(items)
}

// ------------------------ v3.3 (SoT) read API helpers ------------------------

fn fmt_currency_account_id(id: i64) -> String {
    format!("acc_{}", id)
}

pub async fn v33_list_accounts(
    pool: &SqlitePool,
    user_id: &UserId,
) -> AppResult<Vec<AccountItemV33>> {
    let root_account_id = db::get_user_account_id_or_404(pool, user_id.as_str()).await?;
    // currency_accounts is an API-facing mapping layer: one row per (root_account_id, currency).
    let rows = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, currency, status, label
         FROM currency_accounts
         WHERE root_account_id = ?1
         ORDER BY currency ASC",
    )
    .bind(root_account_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, currency, status, label)| AccountItemV33 {
            account_id: fmt_currency_account_id(id),
            currency,
            status,
            label,
        })
        .collect())
}

async fn v33_resolve_currency_account_for_user(
    pool: &SqlitePool,
    user_id: &UserId,
    currency_account_id: i64,
) -> AppResult<(i64, String, String, String, String, Option<String>)> {
    // Returns: (root_account_id, currency, status, label, created_at, closed_at)
    let row = sqlx::query_as::<_, (i64, String, String, String, String, Option<String>)>(
        "SELECT ca.root_account_id, ca.currency, ca.status, ca.label, ca.created_at, ca.closed_at
         FROM currency_accounts ca
         JOIN accounts a ON a.id = ca.root_account_id
         WHERE ca.id = ?1 AND a.owner_type = 'user' AND a.owner_id = ?2
         LIMIT 1",
    )
    .bind(currency_account_id)
    .bind(user_id.as_str())
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound("account not found"))
}

async fn v33_resolve_currency_account_admin(
    pool: &SqlitePool,
    currency_account_id: i64,
) -> AppResult<(i64, String, String, String, String, Option<String>, String)> {
    // Returns: (root_account_id, currency, status, label, created_at, closed_at, owner_id)
    let row = sqlx::query_as::<_, (i64, String, String, String, String, Option<String>, String)>(
        "SELECT ca.root_account_id, ca.currency, ca.status, ca.label, ca.created_at, ca.closed_at, a.owner_id
         FROM currency_accounts ca
         JOIN accounts a ON a.id = ca.root_account_id
         WHERE ca.id = ?1 AND a.owner_type = 'user'
         LIMIT 1",
    )
    .bind(currency_account_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound("account not found"))
}

pub async fn v33_get_account_balance_user(
    pool: &SqlitePool,
    user_id: &UserId,
    currency_account_id: i64,
) -> AppResult<AccountBalanceResponseV33> {
    let (root_account_id, currency, _status, _label, _created_at, _closed_at) =
        v33_resolve_currency_account_for_user(pool, user_id, currency_account_id).await?;

    let bal = sqlx::query_as::<_, (i64, i64)>(
        "SELECT available_minor, hold_minor
         FROM balance_projection
         WHERE account_id = ?1 AND currency = ?2
         LIMIT 1",
    )
    .bind(root_account_id)
    .bind(&currency)
    .fetch_optional(pool)
    .await?
    .unwrap_or((0, 0));

    Ok(AccountBalanceResponseV33 {
        account_id: fmt_currency_account_id(currency_account_id),
        currency,
        available_minor: bal.0,
        hold_minor: bal.1,
    })
}

pub async fn v33_get_account_details_user(
    pool: &SqlitePool,
    user_id: &UserId,
    currency_account_id: i64,
) -> AppResult<AccountDetailsResponseV33> {
    let (root_account_id, currency, status, label, created_at, closed_at) =
        v33_resolve_currency_account_for_user(pool, user_id, currency_account_id).await?;

    let bal = sqlx::query_as::<_, (i64, i64)>(
        "SELECT available_minor, hold_minor
         FROM balance_projection
         WHERE account_id = ?1 AND currency = ?2
         LIMIT 1",
    )
    .bind(root_account_id)
    .bind(&currency)
    .fetch_optional(pool)
    .await?
    .unwrap_or((0, 0));

    Ok(AccountDetailsResponseV33 {
        account_id: fmt_currency_account_id(currency_account_id),
        currency: currency.clone(),
        status,
        label,
        created_at,
        closed_at,
        balance: AccountBalanceResponseV33 {
            account_id: fmt_currency_account_id(currency_account_id),
            currency,
            available_minor: bal.0,
            hold_minor: bal.1,
        },
        owner_phone_number: None,
    })
}

pub async fn v33_get_account_details_admin(
    pool: &SqlitePool,
    currency_account_id: i64,
) -> AppResult<AccountDetailsResponseV33> {
    let (root_account_id, currency, status, label, created_at, closed_at, owner_id) =
        v33_resolve_currency_account_admin(pool, currency_account_id).await?;

    let bal = sqlx::query_as::<_, (i64, i64)>(
        "SELECT available_minor, hold_minor
         FROM balance_projection
         WHERE account_id = ?1 AND currency = ?2
         LIMIT 1",
    )
    .bind(root_account_id)
    .bind(&currency)
    .fetch_optional(pool)
    .await?
    .unwrap_or((0, 0));

    Ok(AccountDetailsResponseV33 {
        account_id: fmt_currency_account_id(currency_account_id),
        currency: currency.clone(),
        status,
        label,
        created_at,
        closed_at,
        balance: AccountBalanceResponseV33 {
            account_id: fmt_currency_account_id(currency_account_id),
            currency,
            available_minor: bal.0,
            hold_minor: bal.1,
        },
        owner_phone_number: Some(owner_id),
    })
}

pub async fn v33_list_account_txs_user(
    pool: &SqlitePool,
    user_id: &UserId,
    currency_account_id: i64,
    before: Option<String>,
    limit: u32,
) -> AppResult<Vec<TxItem>> {
    let (root_account_id, currency, _status, _label, _created_at, _closed_at) =
        v33_resolve_currency_account_for_user(pool, user_id, currency_account_id).await?;

    let limit = limit.min(200).max(1);

    let rows = if let Some(before_ts) = before {
        sqlx::query_as::<
            _,
            (
                String,         // tx_id
                String,         // tx_type
                String,         // state
                String,         // currency
                i64,            // amount_minor (absolute)
                String,         // direction
                String,         // created_at
                String,         // posted_at
                Option<String>, // metadata_json
            ),
        >(
            "SELECT t.id, t.tx_type, t.state, e.currency, e.amount_minor, e.direction,
                    t.created_at, t.posted_at, t.metadata_json
             FROM ledger_entries e
             JOIN ledger_transactions t ON t.id = e.tx_id
             WHERE e.account_id = ?1 AND e.currency = ?2 AND t.created_at < ?3
             ORDER BY t.created_at DESC
             LIMIT ?4",
        )
        .bind(root_account_id)
        .bind(&currency)
        .bind(before_ts)
        .bind(limit as i64)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<
            _,
            (
                String,         // tx_id
                String,         // tx_type
                String,         // state
                String,         // currency
                i64,            // amount_minor (absolute)
                String,         // direction
                String,         // created_at
                String,         // posted_at
                Option<String>, // metadata_json
            ),
        >(
            "SELECT t.id, t.tx_type, t.state, e.currency, e.amount_minor, e.direction,
                    t.created_at, t.posted_at, t.metadata_json
             FROM ledger_entries e
             JOIN ledger_transactions t ON t.id = e.tx_id
             WHERE e.account_id = ?1 AND e.currency = ?2
             ORDER BY t.created_at DESC
             LIMIT ?3",
        )
        .bind(root_account_id)
        .bind(&currency)
        .bind(limit as i64)
        .fetch_all(pool)
        .await?
    };

    let items: Vec<TxItem> = rows
        .into_iter()
        .map(
            |(tx_id, tx_type, state, currency, amount_minor, direction, created_at, posted_at, meta)| {
                let dir = EntryDirection::from_db_str(&direction).map_err(AppError::from)?;
                let signed_amount = dir.apply_sign(amount_minor);
                let description = match TxType::from_db_str_lossy(&tx_type) {
                    Some(TxType::Topup) => "Top up".to_string(),
                    Some(TxType::Transfer) => {
                        if let Some(m) = meta.as_deref() {
                            if let Ok(v) = serde_json::from_str::<serde_json::Value>(m) {
                                let from = v.get("from_user_id").and_then(|x| x.as_str());
                                let to = v.get("to_user_id").and_then(|x| x.as_str());

                                match dir {
                                    EntryDirection::Debit => to
                                        .map(|t| format!("Transfer to {}", t))
                                        .unwrap_or_else(|| "Transfer".to_string()),
                                    EntryDirection::Credit => from
                                        .map(|f| format!("Transfer from {}", f))
                                        .unwrap_or_else(|| "Transfer".to_string()),
                                }
                            } else {
                                "Transfer".to_string()
                            }
                        } else {
                            "Transfer".to_string()
                        }
                    }
                    Some(TxType::Payment) => match state.as_str() {
                        "authorized" => "Hold (authorized)".to_string(),
                        "posted" => "Card payment".to_string(),
                        "reversed" => "Hold (reversed)".to_string(),
                        "refunded" => "Refund".to_string(),
                        _ => "Payment".to_string(),
                    },
                    Some(TxType::Refund) => "Refund".to_string(),
                    None => tx_type.clone(),
                };


                Ok(TxItem {
                    tx_id,
                    tx_type,
                    state,
                    currency,
                    amount_minor: signed_amount,
                    created_at,
                    posted_at,
                    description,
                })
            },
        )
        .collect::<AppResult<Vec<_>>>()?;

    Ok(items)
}


// ------------------------ tx receipt ------------------------

#[derive(Debug, Clone)]
struct TxHeaderRow {
    pub tx_id: String,
    pub tx_type: String,
    pub state: String,
    pub user_account_id: i64,
    pub currency: String,
    pub amount_minor: i64,
    pub created_at: String,
    pub posted_at: String,
    pub metadata_json: Option<String>,
}

async fn load_tx_header(pool: &SqlitePool, tx_id: &TxId) -> AppResult<TxHeaderRow> {
    let row = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            i64,
            String,
            i64,
            String,
            String,
            Option<String>,
        ),
    >(
        "SELECT id, tx_type, state, user_account_id, currency, amount_minor, created_at, posted_at, metadata_json\n         FROM ledger_transactions\n         WHERE id = ?1",
    )
    .bind(tx_id.to_string_hyphenated())
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("tx not found"))?;

    Ok(TxHeaderRow {
        tx_id: row.0,
        tx_type: row.1,
        state: row.2,
        user_account_id: row.3,
        currency: row.4,
        amount_minor: row.5,
        created_at: row.6,
        posted_at: row.7,
        metadata_json: row.8,
    })
}

fn parse_receipt_metadata(
    raw: Option<&str>,
) -> (Option<String>, Option<String>, Option<String>, Option<i64>) {
    let Some(s) = raw else {
        return (None, None, None, None);
    };

    let v: serde_json::Value = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => return (None, None, None, None),
    };

    let memo = v.get("memo").and_then(|x| x.as_str()).map(|x| x.to_string());
    let merchant_label = v
        .get("merchant_label")
        .and_then(|x| x.as_str())
        .map(|x| x.to_string());
    let fx_rate = v
        .get("fx_rate")
        .and_then(|x| x.as_str())
        .map(|x| x.to_string());
    let fee_minor = v.get("fee_minor").and_then(|x| x.as_i64());

    (memo, merchant_label, fx_rate, fee_minor)
}

async fn load_tx_entries(pool: &SqlitePool, tx_id: &TxId) -> AppResult<Vec<TxReceiptEntryItem>> {
    let rows = sqlx::query_as::<_, (String, String, String, String, String, i64)>(
        "SELECT a.owner_type, a.owner_id, a.label, e.currency, e.direction, e.amount_minor\n         FROM ledger_entries e\n         JOIN accounts a ON a.id = e.account_id\n         WHERE e.tx_id = ?1\n         ORDER BY e.id ASC",
    )
    .bind(tx_id.to_string_hyphenated())
    .fetch_all(pool)
    .await?;

    let mut items = Vec::with_capacity(rows.len());
    for (owner_type, owner_id, label, currency, direction, amount_minor) in rows {
        let party_type = match owner_type.as_str() {
            "user" | "system" => owner_type,
            _ => return Err(AppError::Internal("unknown account owner_type")),
        };

        let direction = match direction.as_str() {
            "debit" | "credit" => direction,
            _ => return Err(AppError::Internal("unknown ledger entry direction")),
        };

        let party_label = {
            let trimmed = label.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        };

        items.push(TxReceiptEntryItem {
            party_type,
            party_id: owner_id,
            party_label,
            currency,
            direction,
            amount_minor: amount_minor.abs(),
        });
    }

    Ok(items)
}

pub async fn get_tx_receipt_admin(pool: &SqlitePool, tx_id: &TxId) -> AppResult<TxReceiptResponse> {
    let header = load_tx_header(pool, tx_id).await?;
    let entries = load_tx_entries(pool, tx_id).await?;
    let (memo, merchant_label, fx_rate, fee_minor) =
        parse_receipt_metadata(header.metadata_json.as_deref());

    Ok(TxReceiptResponse {
        tx_id: header.tx_id,
        tx_type: header.tx_type,
        state: header.state,
        currency: header.currency,
        amount_minor: header.amount_minor,
        created_at: header.created_at,
        posted_at: header.posted_at,
        memo,
        merchant_label,
        fx_rate,
        fee_minor,
        entries,
    })
}

pub async fn get_tx_receipt_user(
    pool: &SqlitePool,
    user_id: &UserId,
    tx_id: &TxId,
) -> AppResult<TxReceiptResponse> {
    let user_account_id = db::get_user_account_id_or_404(pool, user_id.as_str()).await?;

    let header = load_tx_header(pool, tx_id).await?;

    let has_entry = sqlx::query_scalar::<_, i64>(
        "SELECT 1\n         FROM ledger_entries e\n         JOIN accounts a ON a.id = e.account_id\n         WHERE e.tx_id = ?1 AND a.owner_type = 'user' AND a.owner_id = ?2\n         LIMIT 1",
    )
    .bind(tx_id.to_string_hyphenated())
    .bind(user_id.as_str())
    .fetch_optional(pool)
    .await?
    .is_some();

    let allowed = has_entry || header.user_account_id == user_account_id;
    if !allowed {
        return Err(AppError::NotFound("tx not found"));
    }

    let entries = load_tx_entries(pool, tx_id).await?;
    let (memo, merchant_label, fx_rate, fee_minor) =
        parse_receipt_metadata(header.metadata_json.as_deref());

    Ok(TxReceiptResponse {
        tx_id: header.tx_id,
        tx_type: header.tx_type,
        state: header.state,
        currency: header.currency,
        amount_minor: header.amount_minor,
        created_at: header.created_at,
        posted_at: header.posted_at,
        memo,
        merchant_label,
        fx_rate,
        fee_minor,
        entries,
    })
}

// ------------------------ admin operations ------------------------

pub async fn admin_create_user(pool: &SqlitePool, user_id: &UserId) -> AppResult<bool> {
    let existed = db::get_user_account_id(pool, user_id.as_str())
        .await?
        .is_some();
    db::create_user(pool, user_id.as_str()).await?;
    Ok(!existed)
}

pub async fn admin_open_currency_account(
    pool: &SqlitePool,
    user_id: &UserId,
    currency: &Currency,
) -> AppResult<bool> {
    let account_id = db::get_user_account_id_or_404(pool, user_id.as_str()).await?;
    db::ensure_currency_supported(pool, currency).await?;
    // For wallet user currencies: allow_hold=1 for card/payment holds.
    let existed = db::has_account_currency(pool, account_id, currency.as_str()).await?;
    db::ensure_account_currency_with_flags(pool, account_id, currency.as_str(), 0, 1).await?;
    db::ensure_projection_row(pool, account_id, currency.as_str()).await?;
    // v3.3 mapping layer (API account_id per currency).
    let _ = db::ensure_currency_account(pool, account_id, currency.as_str()).await?;
    Ok(!existed)
}

pub async fn admin_create_account(
    pool: &SqlitePool,
    owner_user_id: &UserId,
    currency: &Currency,
    label: &str,
) -> AppResult<AdminCreateAccountResponse> {
    db::ensure_currency_supported(pool, currency).await?;
    // For current MVP schema: one account per user, many currencies per account.
    // This endpoint is intentionally shaped like "create account" to support future multi-account work,
    // but is implemented as a safe upsert over the existing schema.
    let account_id = db::ensure_user_account(pool, owner_user_id.as_str()).await?;

    if !label.trim().is_empty() {
        db::set_account_label(pool, account_id, label.trim()).await?;
    }

    // Wallet user currencies: allow_hold=1 for card/payment holds.
    db::ensure_account_currency_with_flags(pool, account_id, currency.as_str(), 0, 1).await?;
    db::ensure_projection_row(pool, account_id, currency.as_str()).await?;

    let row = db::get_account_by_id(pool, account_id).await?;
    let status = AccountStatus::from_db_str_lossy(&row.status).unwrap_or(AccountStatus::Active);

    Ok(AdminCreateAccountResponse {
        account_id: format!("acc_{}", row.id),
        status: status.as_api_str().to_string(),
        currency: currency.as_str().to_string(),
        owner_user_id: owner_user_id.as_str().to_string(),
        created_at: row.created_at,
    })
}

pub async fn admin_close_account(
    pool: &SqlitePool,
    account_id: AccountId,
) -> AppResult<AdminCloseAccountResponse> {
    let row = db::close_account(pool, account_id.get()).await?;
    let status = AccountStatus::from_db_str_lossy(&row.status).unwrap_or(AccountStatus::Closed);

    Ok(AdminCloseAccountResponse {
        account_id: format!("acc_{}", row.id),
        status: status.as_api_str().to_string(),
        closed_at: row.closed_at.unwrap_or_else(db::now_rfc3339),
    })
}

mod flow {
    use super::*;

    pub struct Started;
    pub struct TxInserted;
    pub struct Applied;

    pub enum IdempotencyOutcome<T> {
        Existing(PostOpResponse, TxId),
        Proceed(T),
    }

    pub enum HoldOutcome<T> {
        Existing(PostOpResponse, TxId),
        Proceed(T),
    }

    pub enum RefundOutcome<T> {
        Existing(RefundResponse, TxId),
        Proceed(T),
    }

    pub(super) async fn get_account_tx(
        tx: &mut Transaction<'_, Sqlite>,
        owner_type: &str,
        owner_id: &str,
    ) -> AppResult<AccountId> {
        let id = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM accounts WHERE owner_type = ?1 AND owner_id = ?2",
        )
        .bind(owner_type)
        .bind(owner_id)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(AppError::NotFound("account not found"))?;
        Ok(AccountId::new(id))
    }

    pub(super) async fn require_account_active_tx(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
    ) -> AppResult<()> {
        let status: Option<String> =
            sqlx::query_scalar("SELECT status FROM accounts WHERE id = ?1")
                .bind(account_id.get())
                .fetch_optional(&mut **tx)
                .await?;

        let status = status.ok_or(AppError::NotFound("account not found"))?;
        if status == AccountStatus::Closed.as_db_str() {
            return Err(AppError::AccountClosed);
        }
        Ok(())
    }

    pub(super) async fn ensure_system_currency_setup_tx(
        tx: &mut Transaction<'_, Sqlite>,
        system_account_id: AccountId,
        currency: &Currency,
    ) -> AppResult<()> {
        let now = db::now_rfc3339();

        // allow_negative=1 for system account
        sqlx::query(
            "INSERT OR IGNORE INTO account_currency(account_id, currency, allow_negative, allow_hold, opened_at)\n             VALUES (?1, ?2, 1, 0, ?3)",
        )
        .bind(system_account_id.get())
        .bind(currency.as_str())
        .bind(&now)
        .execute(&mut **tx)
        .await?;

        sqlx::query(
            "INSERT OR IGNORE INTO balance_projection(account_id, currency, available_minor, hold_minor, updated_at)\n             VALUES (?1, ?2, 0, 0, ?3)",
        )
        .bind(system_account_id.get())
        .bind(currency.as_str())
        .bind(&now)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub(super) async fn require_currency_open_tx(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
    ) -> AppResult<()> {
        let ok: Option<i64> = sqlx::query_scalar(
            "SELECT 1 FROM account_currency WHERE account_id = ?1 AND currency = ?2",
        )
        .bind(account_id.get())
        .bind(currency.as_str())
        .fetch_optional(&mut **tx)
        .await?;

        if ok.is_none() {
            return Err(AppError::Conflict("currency account not opened"));
        }

        let ok_proj: Option<i64> = sqlx::query_scalar(
            "SELECT 1 FROM balance_projection WHERE account_id = ?1 AND currency = ?2",
        )
        .bind(account_id.get())
        .bind(currency.as_str())
        .fetch_optional(&mut **tx)
        .await?;

        if ok_proj.is_none() {
            return Err(AppError::Internal(
                "projection row missing for opened currency",
            ));
        }

        Ok(())
    }

    pub(super) async fn debit_available_checked(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET available_minor = available_minor - ?1, updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4 AND available_minor >= ?1",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() == 0 {
            return Err(AppError::Conflict("insufficient funds"));
        }
        Ok(())
    }

    pub(super) async fn debit_available(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET available_minor = available_minor - ?1, updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() == 0 {
            return Err(AppError::Internal("projection row missing"));
        }
        Ok(())
    }

    pub(super) async fn credit_available(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET available_minor = available_minor + ?1, updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() == 0 {
            return Err(AppError::Internal("projection row missing"));
        }
        Ok(())
    }

    pub(super) async fn move_available_to_hold_checked(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET available_minor = available_minor - ?1,\n                 hold_minor = hold_minor + ?1,\n                 updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4 AND available_minor >= ?1",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() != 1 {
            return Err(AppError::Conflict("insufficient funds"));
        }
        Ok(())
    }

    pub(super) async fn release_hold_to_available_checked(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET hold_minor = hold_minor - ?1,\n                 available_minor = available_minor + ?1,\n                 updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4 AND hold_minor >= ?1",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() != 1 {
            return Err(AppError::Conflict("hold is smaller than amount"));
        }
        Ok(())
    }

    pub(super) async fn debit_hold_checked(
        tx: &mut Transaction<'_, Sqlite>,
        account_id: AccountId,
        currency: &Currency,
        amount: AmountMinor,
        updated_at: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            "UPDATE balance_projection\n             SET hold_minor = hold_minor - ?1, updated_at = ?2\n             WHERE account_id = ?3 AND currency = ?4 AND hold_minor >= ?1",
        )
        .bind(amount.as_i64())
        .bind(updated_at)
        .bind(account_id.get())
        .bind(currency.as_str())
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() != 1 {
            return Err(AppError::Conflict("hold is smaller than amount"));
        }
        Ok(())
    }

    // ------------------------------ Topup ------------------------------

    pub struct TopupFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        user_account_id: AccountId,
        system_cash_id: AccountId,
        idempotency_key: IdempotencyKey,
        currency: Currency,
        amount: AmountMinor,
        tx_id: TxId,
        now: String,
        _state: std::marker::PhantomData<S>,
    }

    impl<'a, 't> TopupFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            user_id: &UserId,
            idempotency_key: &IdempotencyKey,
            currency: Currency,
            amount: AmountMinor,
        ) -> AppResult<Self> {
            let user_account_id = get_account_tx(tx, "user", user_id.as_str()).await?;
            let system_cash_id = get_account_tx(tx, "system", "cash").await?;

            require_currency_open_tx(tx, user_account_id, &currency).await?;
            ensure_system_currency_setup_tx(tx, system_cash_id, &currency).await?;

            Ok(Self {
                tx,
                user_account_id,
                system_cash_id,
                idempotency_key: idempotency_key.clone(),
                currency,
                amount,
                tx_id: TxId::new(),
                now: db::now_rfc3339(),
                _state: std::marker::PhantomData,
            })
        }

        pub fn user_account_id(&self) -> AccountId {
            self.user_account_id
        }

        pub async fn check_idempotency(
            self,
        ) -> AppResult<IdempotencyOutcome<TopupFlow<'a, 't, Started>>> {
            if let Some(existing) =
                super::find_tx_by_idem_tx(self.tx, self.user_account_id, &self.idempotency_key)
                    .await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok(IdempotencyOutcome::Existing(existing, existing_tx_id));
            }

            require_account_active_tx(self.tx, self.user_account_id).await?;

            Ok(IdempotencyOutcome::Proceed(self))
        }

        pub async fn insert_tx(self) -> AppResult<TopupFlow<'a, 't, TxInserted>> {
            super::insert_ledger_tx(
                self.tx,
                &self.tx_id,
                &self.idempotency_key,
                TxType::Topup,
                TxState::Posted,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
                &self.now,
                None,
            )
            .await?;

            Ok(TopupFlow {
                tx: self.tx,
                user_account_id: self.user_account_id,
                system_cash_id: self.system_cash_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> TopupFlow<'a, 't, TxInserted> {
        pub async fn apply(self) -> AppResult<TopupFlow<'a, 't, Applied>> {
            let entries = core::build_double_entry(
                self.system_cash_id,
                self.user_account_id,
                &self.currency,
                self.amount,
            );

            for e in &entries {
                super::insert_entry(
                    self.tx,
                    &self.tx_id,
                    e.account_id,
                    &e.currency,
                    e.direction,
                    e.amount_minor,
                    &self.now,
                )
                .await?;
            }

            debug_assert!(core::check_zero_sum(&core::projection_deltas_from_entries(
                &entries
            )));
            debit_available(
                self.tx,
                self.system_cash_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;
            credit_available(
                self.tx,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.posted",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Topup.as_str(),
                    "state": TxState::Posted.as_str(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok(TopupFlow {
                tx: self.tx,
                user_account_id: self.user_account_id,
                system_cash_id: self.system_cash_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> TopupFlow<'a, 't, Applied> {
        pub fn finish(self) -> (PostOpResponse, TxId) {
            (
                PostOpResponse {
                    tx_id: self.tx_id.to_string_hyphenated(),
                    state: "posted".to_string(),
                    currency: self.currency.into_inner(),
                    amount_minor: self.amount.as_i64(),
                },
                self.tx_id,
            )
        }
    }

    // ----------------------------- Transfer -----------------------------

    pub struct TransferFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        from_account_id: AccountId,
        to_account_id: AccountId,
        idempotency_key: IdempotencyKey,
        currency: Currency,
        amount: AmountMinor,
        tx_id: TxId,
        now: String,
        meta_json: String,
        _state: std::marker::PhantomData<S>,
    }

    impl<'a, 't> TransferFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            from_user_id: &UserId,
            to_user_id: UserId,
            idempotency_key: &IdempotencyKey,
            currency: Currency,
            amount: AmountMinor,
        ) -> AppResult<Self> {
            let from_account_id = get_account_tx(tx, "user", from_user_id.as_str()).await?;
            let to_account_id = get_account_tx(tx, "user", to_user_id.as_str()).await?;

            require_currency_open_tx(tx, from_account_id, &currency).await?;
            require_currency_open_tx(tx, to_account_id, &currency).await?;

            let meta_json = serde_json::json!({
                "from_user_id": from_user_id.as_str(),
                "to_user_id": to_user_id.as_str(),
            })
            .to_string();

            Ok(Self {
                tx,
                from_account_id,
                to_account_id,
                idempotency_key: idempotency_key.clone(),
                currency,
                amount,
                tx_id: TxId::new(),
                now: db::now_rfc3339(),
                meta_json,
                _state: std::marker::PhantomData,
            })
        }

        pub fn source_account_id(&self) -> AccountId {
            self.from_account_id
        }

        pub async fn check_idempotency(
            self,
        ) -> AppResult<IdempotencyOutcome<TransferFlow<'a, 't, Started>>> {
            if let Some(existing) =
                super::find_tx_by_idem_tx(self.tx, self.from_account_id, &self.idempotency_key)
                    .await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok(IdempotencyOutcome::Existing(existing, existing_tx_id));
            }

            require_account_active_tx(self.tx, self.from_account_id).await?;
            require_account_active_tx(self.tx, self.to_account_id).await?;

            Ok(IdempotencyOutcome::Proceed(self))
        }

        pub async fn insert_tx(self) -> AppResult<TransferFlow<'a, 't, TxInserted>> {
            super::insert_ledger_tx(
                self.tx,
                &self.tx_id,
                &self.idempotency_key,
                TxType::Transfer,
                TxState::Posted,
                self.from_account_id,
                &self.currency,
                self.amount,
                &self.now,
                &self.now,
                Some(&self.meta_json),
            )
            .await?;

            Ok(TransferFlow {
                tx: self.tx,
                from_account_id: self.from_account_id,
                to_account_id: self.to_account_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                meta_json: self.meta_json,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> TransferFlow<'a, 't, TxInserted> {
        pub async fn apply(self) -> AppResult<TransferFlow<'a, 't, Applied>> {
            let entries = core::build_double_entry(
                self.from_account_id,
                self.to_account_id,
                &self.currency,
                self.amount,
            );

            for e in &entries {
                super::insert_entry(
                    self.tx,
                    &self.tx_id,
                    e.account_id,
                    &e.currency,
                    e.direction,
                    e.amount_minor,
                    &self.now,
                )
                .await?;
            }

            let deltas = core::projection_deltas_from_entries(&entries);
            debug_assert!(core::check_zero_sum(&deltas));

            debit_available_checked(
                self.tx,
                self.from_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;
            credit_available(
                self.tx,
                self.to_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.posted",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Transfer.as_str(),
                    "state": TxState::Posted.as_str(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                    "from_account_id": self.from_account_id.get(),
                    "to_account_id": self.to_account_id.get(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok(TransferFlow {
                tx: self.tx,
                from_account_id: self.from_account_id,
                to_account_id: self.to_account_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                meta_json: self.meta_json,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> TransferFlow<'a, 't, Applied> {
        pub fn finish(self) -> (PostOpResponse, TxId) {
            (
                PostOpResponse {
                    tx_id: self.tx_id.to_string_hyphenated(),
                    state: "posted".to_string(),
                    currency: self.currency.into_inner(),
                    amount_minor: self.amount.as_i64(),
                },
                self.tx_id,
            )
        }
    }

    // ------------------------------ Holds ------------------------------

    pub struct HoldAuthorizeFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        user_account_id: AccountId,
        idempotency_key: IdempotencyKey,
        currency: Currency,
        amount: AmountMinor,
        tx_id: TxId,
        now: String,
        user_id: String,
        _state: std::marker::PhantomData<S>,
    }

    impl<'a, 't> HoldAuthorizeFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            user_id: &UserId,
            idempotency_key: &IdempotencyKey,
            req: &HoldRequestValidated,
        ) -> AppResult<Self> {
            let user_account_id = get_account_tx(tx, "user", user_id.as_str()).await?;
            require_currency_open_tx(tx, user_account_id, &req.currency).await?;

            Ok(Self {
                tx,
                user_account_id,
                idempotency_key: idempotency_key.clone(),
                currency: req.currency.clone(),
                amount: req.amount_minor,
                tx_id: TxId::new(),
                now: db::now_rfc3339(),
                user_id: user_id.as_str().to_string(),
                _state: std::marker::PhantomData,
            })
        }

        pub fn user_account_id(&self) -> AccountId {
            self.user_account_id
        }

        pub async fn check_idempotency(
            self,
        ) -> AppResult<IdempotencyOutcome<HoldAuthorizeFlow<'a, 't, Started>>> {
            if let Some(existing) =
                super::find_tx_by_idem_tx(self.tx, self.user_account_id, &self.idempotency_key)
                    .await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok(IdempotencyOutcome::Existing(existing, existing_tx_id));
            }

            require_account_active_tx(self.tx, self.user_account_id).await?;

            Ok(IdempotencyOutcome::Proceed(self))
        }

        pub async fn insert_tx(self) -> AppResult<HoldAuthorizeFlow<'a, 't, TxInserted>> {
            super::insert_ledger_tx(
                self.tx,
                &self.tx_id,
                &self.idempotency_key,
                TxType::Payment,
                TxState::Authorized,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
                &self.now,
                None,
            )
            .await?;

            Ok(HoldAuthorizeFlow {
                tx: self.tx,
                user_account_id: self.user_account_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                user_id: self.user_id,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> HoldAuthorizeFlow<'a, 't, TxInserted> {
        pub async fn apply(self) -> AppResult<HoldAuthorizeFlow<'a, 't, Applied>> {
            move_available_to_hold_checked(
                self.tx,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.authorized",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Payment.as_str(),
                    "state": TxState::Authorized.as_str(),
                    "user_id": self.user_id,
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok(HoldAuthorizeFlow {
                tx: self.tx,
                user_account_id: self.user_account_id,
                idempotency_key: self.idempotency_key,
                currency: self.currency,
                amount: self.amount,
                tx_id: self.tx_id,
                now: self.now,
                user_id: self.user_id,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> HoldAuthorizeFlow<'a, 't, Applied> {
        pub fn finish(self) -> (PostOpResponse, TxId) {
            (
                PostOpResponse {
                    tx_id: self.tx_id.to_string_hyphenated(),
                    state: TxState::Authorized.as_str().to_string(),
                    currency: self.currency.into_inner(),
                    amount_minor: self.amount.as_i64(),
                },
                self.tx_id,
            )
        }
    }

    pub struct HoldCaptureFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        tx_id: TxId,
        user_account_id: AccountId,
        currency: Currency,
        amount: AmountMinor,
        now: String,
        _state: std::marker::PhantomData<S>,
    }

    impl<'a, 't> HoldCaptureFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            tx_id: &TxId,
        ) -> AppResult<HoldOutcome<HoldCaptureFlow<'a, 't, Started>>> {
            let row = sqlx::query_as::<_, (i64, String, String, i64)>(
                "SELECT user_account_id, state, currency, amount_minor FROM ledger_transactions WHERE id = ?1",
            )
            .bind(tx_id.to_string_hyphenated())
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(AppError::NotFound("tx not found"))?;

            let user_account_id = AccountId::new(row.0);
            let state = row.1;
            let currency = Currency::parse(&row.2).map_err(AppError::from)?;
            let amount = AmountMinor::try_from(row.3).map_err(AppError::from)?;

            if state == TxState::Posted.as_str() {
                #[allow(clippy::clone_on_copy)]
                let tx_id_copy = tx_id.clone();
                return Ok(HoldOutcome::Existing(
                    PostOpResponse {
                        tx_id: tx_id.to_string_hyphenated(),
                        state: TxState::Posted.as_str().to_string(),
                        currency: currency.into_inner(),
                        amount_minor: amount.as_i64(),
                    },
                    tx_id_copy,
                ));
            }
            if state != TxState::Authorized.as_str() {
                return Err(AppError::Conflict("tx is not in authorized state"));
            }

            require_account_active_tx(tx, user_account_id).await?;
            #[allow(clippy::clone_on_copy)]
            let tx_id_copy = tx_id.clone();
            Ok(HoldOutcome::Proceed(HoldCaptureFlow {
                tx,
                tx_id: tx_id_copy,
                user_account_id,
                currency,
                amount,
                now: db::now_rfc3339(),
                _state: std::marker::PhantomData,
            }))
        }
    }

    impl<'a, 't> HoldCaptureFlow<'a, 't, Started> {
        pub async fn apply(self) -> AppResult<HoldCaptureFlow<'a, 't, Applied>> {
            let system_cash_id = get_account_tx(self.tx, "system", "cash").await?;
            ensure_system_currency_setup_tx(self.tx, system_cash_id, &self.currency).await?;

            let updated = sqlx::query(
                "UPDATE ledger_transactions SET state = 'posted', posted_at = ?2 WHERE id = ?1 AND state = 'authorized'",
            )
            .bind(self.tx_id.to_string_hyphenated())
            .bind(&self.now)
            .execute(&mut **self.tx)
            .await?;

            if updated.rows_affected() != 1 {
                return Err(AppError::Conflict("concurrent state change"));
            }

            debit_hold_checked(
                self.tx,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            let entries = core::build_double_entry(
                self.user_account_id,
                system_cash_id,
                &self.currency,
                self.amount,
            );
            for e in &entries {
                super::insert_entry(
                    self.tx,
                    &self.tx_id,
                    e.account_id,
                    &e.currency,
                    e.direction,
                    e.amount_minor,
                    &self.now,
                )
                .await?;
            }

            credit_available(
                self.tx,
                system_cash_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.posted",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Payment.as_str(),
                    "state": TxState::Posted.as_str(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok(HoldCaptureFlow {
                tx: self.tx,
                tx_id: self.tx_id,
                user_account_id: self.user_account_id,
                currency: self.currency,
                amount: self.amount,
                now: self.now,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> HoldCaptureFlow<'a, 't, Applied> {
        pub fn finish(self) -> (PostOpResponse, TxId) {
            (
                PostOpResponse {
                    tx_id: self.tx_id.to_string_hyphenated(),
                    state: TxState::Posted.as_str().to_string(),
                    currency: self.currency.into_inner(),
                    amount_minor: self.amount.as_i64(),
                },
                self.tx_id,
            )
        }
    }

    pub struct HoldCancelFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        tx_id: TxId,
        user_account_id: AccountId,
        currency: Currency,
        amount: AmountMinor,
        now: String,
        _state: std::marker::PhantomData<S>,
    }

    impl<'a, 't> HoldCancelFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            tx_id: &TxId,
        ) -> AppResult<HoldOutcome<HoldCancelFlow<'a, 't, Started>>> {
            let row = sqlx::query_as::<_, (i64, String, String, i64)>(
                "SELECT user_account_id, state, currency, amount_minor FROM ledger_transactions WHERE id = ?1",
            )
            .bind(tx_id.to_string_hyphenated())
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(AppError::NotFound("tx not found"))?;

            let user_account_id = AccountId::new(row.0);
            let state = row.1;
            let currency = Currency::parse(&row.2).map_err(AppError::from)?;
            let amount = AmountMinor::try_from(row.3).map_err(AppError::from)?;

            if state == TxState::Reversed.as_str() {
                #[allow(clippy::clone_on_copy)]
                let tx_id_copy = tx_id.clone();
                return Ok(HoldOutcome::Existing(
                    PostOpResponse {
                        tx_id: tx_id.to_string_hyphenated(),
                        state: TxState::Reversed.as_str().to_string(),
                        currency: currency.into_inner(),
                        amount_minor: amount.as_i64(),
                    },
                    tx_id_copy,
                ));
            }
            if state != TxState::Authorized.as_str() {
                return Err(AppError::Conflict("tx is not in authorized state"));
            }

            require_account_active_tx(tx, user_account_id).await?;
            #[allow(clippy::clone_on_copy)]
            let tx_id_copy = tx_id.clone();
            Ok(HoldOutcome::Proceed(HoldCancelFlow {
                tx,
                tx_id: tx_id_copy,
                user_account_id,
                currency,
                amount,
                now: db::now_rfc3339(),
                _state: std::marker::PhantomData,
            }))
        }
    }

    impl<'a, 't> HoldCancelFlow<'a, 't, Started> {
        pub async fn apply(self) -> AppResult<HoldCancelFlow<'a, 't, Applied>> {
            let updated = sqlx::query(
                "UPDATE ledger_transactions SET state = 'reversed', posted_at = ?2 WHERE id = ?1 AND state = 'authorized'",
            )
            .bind(self.tx_id.to_string_hyphenated())
            .bind(&self.now)
            .execute(&mut **self.tx)
            .await?;

            if updated.rows_affected() != 1 {
                return Err(AppError::Conflict("concurrent state change"));
            }

            release_hold_to_available_checked(
                self.tx,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.reversed",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Payment.as_str(),
                    "state": TxState::Reversed.as_str(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok(HoldCancelFlow {
                tx: self.tx,
                tx_id: self.tx_id,
                user_account_id: self.user_account_id,
                currency: self.currency,
                amount: self.amount,
                now: self.now,
                _state: std::marker::PhantomData,
            })
        }
    }

    impl<'a, 't> HoldCancelFlow<'a, 't, Applied> {
        pub fn finish(self) -> (PostOpResponse, TxId) {
            (
                PostOpResponse {
                    tx_id: self.tx_id.to_string_hyphenated(),
                    state: TxState::Reversed.as_str().to_string(),
                    currency: self.currency.into_inner(),
                    amount_minor: self.amount.as_i64(),
                },
                self.tx_id,
            )
        }
    }

    // ------------------------------ Refund ------------------------------

    pub struct RefundFlow<'a, 't, S> {
        tx: &'a mut Transaction<'t, Sqlite>,
        tx_id: TxId,
        user_account_id: AccountId,
        currency: Currency,
        amount: AmountMinor,
        now: String,
        _state: std::marker::PhantomData<S>,
    }

    pub struct RefundApplied {
        refund_tx_id: TxId,
    }

    impl<'a, 't> RefundFlow<'a, 't, Started> {
        pub async fn start(
            tx: &'a mut Transaction<'t, Sqlite>,
            tx_id: &TxId,
        ) -> AppResult<RefundOutcome<RefundFlow<'a, 't, Started>>> {
            let row = sqlx::query_as::<_, (i64, String, String, i64)>(
                "SELECT user_account_id, state, currency, amount_minor FROM ledger_transactions WHERE id = ?1",
            )
            .bind(tx_id.to_string_hyphenated())
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(AppError::NotFound("tx not found"))?;

            let user_account_id = AccountId::new(row.0);
            let state = row.1;
            let currency = Currency::parse(&row.2).map_err(AppError::from)?;
            let amount = AmountMinor::try_from(row.3).map_err(AppError::from)?;

            if state == TxState::Refunded.as_str() {
                let refund_tx_id = sqlx::query_scalar::<_, String>(
                    "SELECT id FROM ledger_transactions WHERE tx_type = 'refund' AND metadata_json LIKE ?1 ORDER BY created_at DESC LIMIT 1",
                )
                .bind(format!("%{}%", tx_id.to_string_hyphenated()))
                .fetch_optional(&mut **tx)
                .await?
                .unwrap_or_else(|| "".to_string());
                if refund_tx_id.is_empty() {
                    return Err(AppError::NotFound("refund tx not found"));
                }
                let refund_tx_id = TxId::parse(&refund_tx_id).map_err(AppError::from)?;
                return Ok(RefundOutcome::Existing(
                    RefundResponse {
                        original_tx_id: tx_id.to_string_hyphenated(),
                        refund_tx_id: refund_tx_id.to_string_hyphenated(),
                        state: TxState::Refunded.as_str().to_string(),
                        currency: currency.as_str().to_string(),
                        amount_minor: amount.as_i64(),
                    },
                    refund_tx_id,
                ));
            }
            if state != TxState::Posted.as_str() {
                return Err(AppError::Conflict("tx is not in posted state"));
            }

            require_account_active_tx(tx, user_account_id).await?;
            #[allow(clippy::clone_on_copy)]
            let tx_id_copy = tx_id.clone();
            Ok(RefundOutcome::Proceed(RefundFlow {
                tx,
                tx_id: tx_id_copy,
                user_account_id,
                currency,
                amount,
                now: db::now_rfc3339(),
                _state: std::marker::PhantomData,
            }))
        }
    }

    impl<'a, 't> RefundFlow<'a, 't, Started> {
        pub async fn apply(self) -> AppResult<(RefundFlow<'a, 't, Applied>, RefundApplied)> {
            let updated = sqlx::query(
                "UPDATE ledger_transactions SET state = 'refunded' WHERE id = ?1 AND state = 'posted'",
            )
            .bind(self.tx_id.to_string_hyphenated())
            .execute(&mut **self.tx)
            .await?;
            if updated.rows_affected() != 1 {
                return Err(AppError::Conflict("concurrent state change"));
            }

            let system_cash_id = get_account_tx(self.tx, "system", "cash").await?;
            ensure_system_currency_setup_tx(self.tx, system_cash_id, &self.currency).await?;

            let refund_tx_id = TxId::new();
            let refund_idem =
                IdempotencyKey::parse(&format!("refund:{}", refund_tx_id.to_string_hyphenated()))
                    .map_err(AppError::from)?;
            let meta = json!({ "original_tx_id": self.tx_id.to_string_hyphenated() }).to_string();

            super::insert_ledger_tx(
                self.tx,
                &refund_tx_id,
                &refund_idem,
                TxType::Refund,
                TxState::Posted,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
                &self.now,
                Some(&meta),
            )
            .await?;

            let entries = core::build_double_entry(
                system_cash_id,
                self.user_account_id,
                &self.currency,
                self.amount,
            );
            for e in &entries {
                super::insert_entry(
                    self.tx,
                    &refund_tx_id,
                    e.account_id,
                    &e.currency,
                    e.direction,
                    e.amount_minor,
                    &self.now,
                )
                .await?;
            }

            debit_available(
                self.tx,
                system_cash_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;
            credit_available(
                self.tx,
                self.user_account_id,
                &self.currency,
                self.amount,
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.refunded",
                self.tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": self.tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Payment.as_str(),
                    "state": TxState::Refunded.as_str(),
                    "refund_tx_id": refund_tx_id.to_string_hyphenated(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            super::insert_outbox_event(
                self.tx,
                "tx.posted",
                refund_tx_id.to_string_hyphenated().as_str(),
                &json!({
                    "tx_id": refund_tx_id.to_string_hyphenated(),
                    "tx_type": TxType::Refund.as_str(),
                    "state": TxState::Posted.as_str(),
                    "currency": self.currency.as_str(),
                    "amount_minor": self.amount.as_i64(),
                    "original_tx_id": self.tx_id.to_string_hyphenated(),
                })
                .to_string(),
                &self.now,
            )
            .await?;

            Ok((
                RefundFlow {
                    tx: self.tx,
                    tx_id: self.tx_id,
                    user_account_id: self.user_account_id,
                    currency: self.currency,
                    amount: self.amount,
                    now: self.now,
                    _state: std::marker::PhantomData,
                },
                RefundApplied { refund_tx_id },
            ))
        }
    }

    impl<'a, 't> RefundFlow<'a, 't, Applied> {
        pub fn finish(self, applied: RefundApplied) -> (RefundResponse, TxId) {
            (
                RefundResponse {
                    original_tx_id: self.tx_id.to_string_hyphenated(),
                    refund_tx_id: applied.refund_tx_id.to_string_hyphenated(),
                    state: TxState::Refunded.as_str().to_string(),
                    currency: self.currency.as_str().to_string(),
                    amount_minor: self.amount.as_i64(),
                },
                applied.refund_tx_id,
            )
        }
    }
}

pub async fn topup_posted(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: &UserId,
    idempotency_key: &IdempotencyKey,
    req: TopupRequestValidated,
) -> AppResult<(PostOpResponse, TxId)> {
    ensure_currency_supported_tx(tx, &req.currency).await?;
    let flow = flow::TopupFlow::start(tx, user_id, idempotency_key, req.currency, req.amount_minor)
        .await?;

    let flow = match flow.check_idempotency().await? {
        flow::IdempotencyOutcome::Existing(existing, tx_id) => return Ok((existing, tx_id)),
        flow::IdempotencyOutcome::Proceed(flow) => flow,
    };

    let user_account_id = flow.user_account_id();
    let flow = match flow.insert_tx().await {
        Ok(flow) => flow,
        Err(AppError::Conflict(_)) => {
            if let Some(existing) = find_tx_by_idem_tx(tx, user_account_id, idempotency_key).await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok((existing, existing_tx_id));
            }
            return Err(AppError::Conflict("duplicate idempotency key"));
        }
        Err(e) => return Err(e),
    };

    let flow = flow.apply().await?;
    Ok(flow.finish())
}

pub async fn transfer_posted(
    tx: &mut Transaction<'_, Sqlite>,
    from_user_id: &UserId,
    idempotency_key: &IdempotencyKey,
    req: TransferRequestValidated,
) -> AppResult<(PostOpResponse, TxId)> {
    ensure_currency_supported_tx(tx, &req.currency).await?;
    if req.to_user_id.as_str() == from_user_id.as_str() {
        return Err(AppError::BadRequest("cannot transfer to self"));
    }

    let flow = flow::TransferFlow::start(
        tx,
        from_user_id,
        req.to_user_id,
        idempotency_key,
        req.currency,
        req.amount_minor,
    )
    .await?;

    let flow = match flow.check_idempotency().await? {
        flow::IdempotencyOutcome::Existing(existing, tx_id) => return Ok((existing, tx_id)),
        flow::IdempotencyOutcome::Proceed(flow) => flow,
    };

    let from_account_id = flow.source_account_id();
    let flow = match flow.insert_tx().await {
        Ok(flow) => flow,
        Err(AppError::Conflict(_)) => {
            if let Some(existing) = find_tx_by_idem_tx(tx, from_account_id, idempotency_key).await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok((existing, existing_tx_id));
            }
            return Err(AppError::Conflict("duplicate idempotency key"));
        }
        Err(e) => return Err(e),
    };

    let flow = flow.apply().await?;
    Ok(flow.finish())
}

// ------------------------ payments: hold/capture/refund ------------------------

pub async fn payment_hold_authorize(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: &UserId,
    idempotency_key: &IdempotencyKey,
    req: HoldRequestValidated,
) -> AppResult<(PostOpResponse, TxId)> {
    ensure_currency_supported_tx(tx, &req.currency).await?;
    let flow = flow::HoldAuthorizeFlow::start(tx, user_id, idempotency_key, &req).await?;

    let flow = match flow.check_idempotency().await? {
        flow::IdempotencyOutcome::Existing(existing, tx_id) => return Ok((existing, tx_id)),
        flow::IdempotencyOutcome::Proceed(flow) => flow,
    };

    let user_account_id = flow.user_account_id();
    let flow = match flow.insert_tx().await {
        Ok(flow) => flow,
        Err(AppError::Conflict(_)) => {
            if let Some(existing) = find_tx_by_idem_tx(tx, user_account_id, idempotency_key).await?
            {
                let existing_tx_id = TxId::parse(&existing.tx_id).map_err(AppError::from)?;
                return Ok((existing, existing_tx_id));
            }
            return Err(AppError::Conflict("duplicate idempotency key"));
        }
        Err(e) => return Err(e),
    };

    let flow = flow.apply().await?;
    Ok(flow.finish())
}

pub async fn payment_hold_capture(
    tx: &mut Transaction<'_, Sqlite>,
    tx_id: &TxId,
) -> AppResult<(PostOpResponse, TxId)> {
    let flow = match flow::HoldCaptureFlow::start(tx, tx_id).await? {
        flow::HoldOutcome::Existing(existing, result_tx_id) => {
            return Ok((existing, result_tx_id));
        }
        flow::HoldOutcome::Proceed(flow) => flow,
    };

    let flow = flow.apply().await?;
    Ok(flow.finish())
}

pub async fn payment_hold_cancel(
    tx: &mut Transaction<'_, Sqlite>,
    tx_id: &TxId,
) -> AppResult<(PostOpResponse, TxId)> {
    let flow = match flow::HoldCancelFlow::start(tx, tx_id).await? {
        flow::HoldOutcome::Existing(existing, result_tx_id) => {
            return Ok((existing, result_tx_id));
        }
        flow::HoldOutcome::Proceed(flow) => flow,
    };

    let flow = flow.apply().await?;
    Ok(flow.finish())
}

pub async fn payment_refund(
    tx: &mut Transaction<'_, Sqlite>,
    tx_id: &TxId,
) -> AppResult<(RefundResponse, TxId)> {
    let flow = match flow::RefundFlow::start(tx, tx_id).await? {
        flow::RefundOutcome::Existing(existing, result_tx_id) => {
            return Ok((existing, result_tx_id));
        }
        flow::RefundOutcome::Proceed(flow) => flow,
    };

    let (flow, applied) = flow.apply().await?;
    Ok(flow.finish(applied))
}

// ------------------------ helpers ------------------------

pub async fn post_op_response_by_tx_id(
    pool: &SqlitePool,
    tx_id: &TxId,
) -> AppResult<PostOpResponse> {
    let row = sqlx::query_as::<_, (String, String, String, i64)>(
        "SELECT id, state, currency, amount_minor FROM ledger_transactions WHERE id = ?1",
    )
    .bind(tx_id.to_string_hyphenated())
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("tx not found"))?;

    Ok(PostOpResponse {
        tx_id: row.0,
        state: row.1,
        currency: row.2,
        amount_minor: row.3,
    })
}

pub async fn refund_response_by_tx_ids(
    pool: &SqlitePool,
    original_tx_id: &TxId,
    refund_tx_id: &TxId,
) -> AppResult<RefundResponse> {
    let row = sqlx::query_as::<_, (String, String, i64)>(
        "SELECT state, currency, amount_minor FROM ledger_transactions WHERE id = ?1",
    )
    .bind(original_tx_id.to_string_hyphenated())
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("tx not found"))?;

    let refund_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(1) FROM ledger_transactions WHERE id = ?1 AND tx_type = 'refund'",
    )
    .bind(refund_tx_id.to_string_hyphenated())
    .fetch_one(pool)
    .await?;

    if refund_exists == 0 {
        return Err(AppError::NotFound("refund tx not found"));
    }

    Ok(RefundResponse {
        original_tx_id: original_tx_id.to_string_hyphenated(),
        refund_tx_id: refund_tx_id.to_string_hyphenated(),
        state: row.0,
        currency: row.1,
        amount_minor: row.2,
    })
}

async fn find_tx_by_idem_tx(
    tx: &mut Transaction<'_, Sqlite>,
    user_account_id: AccountId,
    idempotency_key: &IdempotencyKey,
) -> AppResult<Option<PostOpResponse>> {
    let row = sqlx::query_as::<_, (String, String, String, i64)>(
        "SELECT id, state, currency, amount_minor
         FROM ledger_transactions
         WHERE user_account_id = ?1 AND idempotency_key = ?2",
    )
    .bind(user_account_id.get())
    .bind(idempotency_key.as_str())
    .fetch_optional(&mut **tx)
    .await?;

    Ok(
        row.map(|(tx_id, state, currency, amount_minor)| PostOpResponse {
            tx_id,
            state,
            currency,
            amount_minor,
        }),
    )
}

#[allow(clippy::too_many_arguments)]
async fn insert_ledger_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &TxId,
    idempotency_key: &IdempotencyKey,
    tx_type: TxType,
    state: TxState,
    user_account_id: AccountId,
    currency: &Currency,
    amount_minor: AmountMinor,
    created_at: &str,
    posted_at: &str,
    metadata_json: Option<&str>,
) -> AppResult<()> {
    let res = sqlx::query(
        "INSERT INTO ledger_transactions(\n            id, idempotency_key, tx_type, state,\n            user_account_id, currency, amount_minor,\n            created_at, posted_at, metadata_json\n         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )
    .bind(id.to_string_hyphenated())
    .bind(idempotency_key.as_str())
    .bind(tx_type.as_str())
    .bind(state.as_str())
    .bind(user_account_id.get())
    .bind(currency.as_str())
    .bind(amount_minor.as_i64())
    .bind(created_at)
    .bind(posted_at)
    .bind(metadata_json)
    .execute(&mut **tx)
    .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                // unique constraint on idempotency_key
                if db_err.message().to_lowercase().contains("unique") {
                    return Err(AppError::Conflict("duplicate idempotency key"));
                }
            }
            Err(AppError::Db(e))
        }
    }
}

async fn insert_entry(
    tx: &mut Transaction<'_, Sqlite>,
    tx_id: &TxId,
    account_id: AccountId,
    currency: &Currency,
    direction: EntryDirection,
    amount_minor: AmountMinor,
    created_at: &str,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO ledger_entries(tx_id, account_id, currency, direction, amount_minor, created_at)\n         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .bind(tx_id.to_string_hyphenated())
    .bind(account_id.get())
    .bind(currency.as_str())
    .bind(direction.as_str())
    .bind(amount_minor.as_i64())
    .bind(created_at)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_outbox_event(
    tx: &mut Transaction<'_, Sqlite>,
    event_type: &str,
    aggregate_id: &str,
    payload_json: &str,
    created_at: &str,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO outbox_events(event_type, aggregate_id, payload_json, created_at, published_at)
         VALUES (?1, ?2, ?3, ?4, NULL)",
    )
    .bind(event_type)
    .bind(aggregate_id)
    .bind(payload_json)
    .bind(created_at)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
