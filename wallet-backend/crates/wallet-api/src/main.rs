use tracing_subscriber::EnvFilter;

use wallet_infra::db as infra_db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let db_file = std::env::var("DB_FILE").unwrap_or_else(|_| "wallet.db".to_string());
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let pool = infra_db::create_pool(&db_file).await?;
    infra_db::migrate(&pool).await?;
    infra_db::ensure_system_accounts(&pool).await?;

    let app = wallet_api::app::build_app(pool);

    tracing::info!("listening on http://{bind_addr}");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
