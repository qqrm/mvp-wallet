use tracing_subscriber::EnvFilter;

use wallet_infra::{db as infra_db, dev as infra_dev};

fn demo_seed_mode() -> bool {
    match std::env::var("WALLET_DEV_SEED") {
        Ok(v) if v == "1" => true,
        Ok(v) if v == "0" => false,
        _ => cfg!(debug_assertions),
    }
}

fn remove_sqlite_files(path: &std::path::Path) -> std::io::Result<()> {
    // SQLite WAL mode may create -wal / -shm siblings.
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension(format!(
        "{}-wal",
        path.extension().and_then(|e| e.to_str()).unwrap_or("db")
    )));
    let _ = std::fs::remove_file(path.with_extension(format!(
        "{}-shm",
        path.extension().and_then(|e| e.to_str()).unwrap_or("db")
    )));
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let dev_seed = demo_seed_mode();
    let db_file = if dev_seed {
        "wallet-dev.db"
    } else {
        "wallet.db"
    };

    if dev_seed {
        let path = infra_db::db_path(db_file);
        // Deterministic demo DB on every backend restart.
        // Safe by construction: only affects wallet-dev.db.
        if path.exists() {
            remove_sqlite_files(&path)?;
        }
    }

    let pool = infra_db::create_pool(db_file).await?;
    infra_db::migrate(&pool).await?;
    infra_db::ensure_system_accounts(&pool).await?;
    if dev_seed {
        infra_dev::seed_demo_data(&pool).await?;
    }

    let app = wallet_api::app::build_app(pool);

    let addr = "127.0.0.1:3000";
    tracing::info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}
