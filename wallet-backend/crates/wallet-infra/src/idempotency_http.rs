use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::{future::Future, pin::Pin};

use crate::db;
use wallet_app::{AppError, AppResult};
use wallet_domain::{IdempotencyKey, TxId};

const IN_PROGRESS_TTL_SECS: i64 = 30;

#[derive(Debug)]
pub enum HttpIdempotencyOutcome<T> {
    Replay { result_tx_id: TxId },
    Live(T),
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

enum LookupOutcome {
    Replay(TxId),
    Reserved,
}

fn normalize_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            let mut normalized = serde_json::Map::with_capacity(map.len());
            for key in keys {
                let entry = map
                    .get(key)
                    .map(normalize_value)
                    .unwrap_or(serde_json::Value::Null);
                normalized.insert(key.clone(), entry);
            }
            serde_json::Value::Object(normalized)
        }
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(normalize_value).collect())
        }
        other => other.clone(),
    }
}

fn normalize_json(value: &serde_json::Value) -> String {
    let normalized = normalize_value(value);
    serde_json::to_string(&normalized).unwrap_or_else(|_| "null".to_string())
}

pub fn request_hash(body: &serde_json::Value, path: &str, actor: &str) -> String {
    let normalized = normalize_json(body);
    let payload = format!("{normalized}|{path}|{actor}");
    let digest = Sha256::digest(payload.as_bytes());
    format!("{digest:x}")
}

fn parse_rfc3339(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

async fn reserve_or_load(
    tx: &mut Transaction<'_, Sqlite>,
    scope: &str,
    key: &IdempotencyKey,
    request_hash: &str,
) -> AppResult<LookupOutcome> {
    let now = db::now_rfc3339();

    let inserted = sqlx::query(
        "INSERT OR IGNORE INTO http_idempotency(
            scope, key, request_hash, status_code, response_json, status, reserved_at, created_at
         )
         VALUES (?1, ?2, ?3, 0, '', 'reserved', ?4, ?4)",
    )
    .bind(scope)
    .bind(key.as_str())
    .bind(request_hash)
    .bind(&now)
    .execute(&mut **tx)
    .await?
    .rows_affected()
        > 0;

    let row = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, String)>(
        "SELECT request_hash, status, result_tx_id, reserved_at, created_at
         FROM http_idempotency
         WHERE scope = ?1 AND key = ?2",
    )
    .bind(scope)
    .bind(key.as_str())
    .fetch_one(&mut **tx)
    .await?;

    let (stored_hash, status, result_tx_id, reserved_at, created_at) = row;
    if stored_hash != request_hash {
        return Err(AppError::Conflict(
            "idempotency key reused with different request",
        ));
    }

    match status.as_str() {
        "completed" => {
            let result_tx_id = result_tx_id
                .filter(|value| !value.is_empty())
                .ok_or(AppError::Internal("idempotency record missing result"))?;
            let parsed = TxId::parse(&result_tx_id).map_err(AppError::from)?;
            return Ok(LookupOutcome::Replay(parsed));
        }
        "reserved" | "failed" | "" => {}
        _ => {
            return Err(AppError::Internal("invalid idempotency status"));
        }
    }

    let now_dt = Utc::now();
    let lease_source = reserved_at.as_deref().unwrap_or(&created_at);
    if let Some(reserved_dt) = parse_rfc3339(lease_source) {
        let age = now_dt.timestamp() - reserved_dt.timestamp();
        if age > IN_PROGRESS_TTL_SECS {
            sqlx::query(
                "UPDATE http_idempotency
                 SET reserved_at = ?1,
                     status = 'reserved',
                     result_tx_id = NULL,
                     completed_at = NULL,
                     error_code = NULL,
                     error_message = NULL
                 WHERE scope = ?2 AND key = ?3",
            )
            .bind(db::now_rfc3339())
            .bind(scope)
            .bind(key.as_str())
            .execute(&mut **tx)
            .await?;

            return Ok(LookupOutcome::Reserved);
        }
    }

    if inserted {
        return Ok(LookupOutcome::Reserved);
    }

    Err(AppError::Conflict("idempotency request in progress"))
}

async fn finalize_idempotency(
    tx: &mut Transaction<'_, Sqlite>,
    scope: &str,
    key: &IdempotencyKey,
    result_tx_id: &TxId,
) -> AppResult<()> {
    let now = db::now_rfc3339();
    sqlx::query(
        "UPDATE http_idempotency
         SET status = 'completed',
             result_tx_id = ?1,
             completed_at = ?2
         WHERE scope = ?3 AND key = ?4",
    )
    .bind(result_tx_id.to_string_hyphenated())
    .bind(now)
    .bind(scope)
    .bind(key.as_str())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn execute<T, F>(
    pool: &SqlitePool,
    scope: &str,
    key: &IdempotencyKey,
    request_hash: &str,
    op: F,
) -> AppResult<HttpIdempotencyOutcome<T>>
where
    F: for<'a> FnOnce(&'a mut Transaction<'_, Sqlite>) -> BoxFuture<'a, AppResult<(T, TxId)>>,
{
    let mut tx = pool.begin().await?;
    match reserve_or_load(&mut tx, scope, key, request_hash).await? {
        LookupOutcome::Replay(tx_id) => {
            tx.rollback().await?;
            return Ok(HttpIdempotencyOutcome::Replay {
                result_tx_id: tx_id,
            });
        }
        LookupOutcome::Reserved => {
            // Continue within the same transaction for the monetary operation.
        }
    }

    let result = op(&mut tx).await;

    match result {
        Ok((value, result_tx_id)) => {
            finalize_idempotency(&mut tx, scope, key, &result_tx_id).await?;
            tx.commit().await?;
            Ok(HttpIdempotencyOutcome::Live(value))
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err)
        }
    }
}
