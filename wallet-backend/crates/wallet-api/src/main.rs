use tracing_subscriber::EnvFilter;

use wallet_infra::db as infra_db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let pool = infra_db::create_pool("wallet.db").await?;
    infra_db::migrate(&pool).await?;
    infra_db::ensure_system_accounts(&pool).await?;

    let app = wallet_api::app::build_app(pool);

    let addr = "127.0.0.1:3000";
    tracing::info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
