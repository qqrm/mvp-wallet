use sqlx::SqlitePool;

use wallet_app::AppResult;

#[derive(Debug, sqlx::FromRow)]
pub struct AdminUserRow {
    pub user_id: String,
    pub label: String,
    pub status: String,
    pub closed_at: Option<String>,
}

pub async fn list_users(pool: &SqlitePool) -> AppResult<Vec<AdminUserRow>> {
    // Invariant: accounts has UNIQUE(owner_type, owner_id), so one user maps to one row.
    // status/closed_at are included for future filtering in the API layer.
    let rows = sqlx::query_as::<_, AdminUserRow>(
        "SELECT owner_id AS user_id, label, status, closed_at
         FROM accounts
         WHERE owner_type = 'user'
         ORDER BY owner_id ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
