mod config;
mod error;
mod model;
mod repository;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // ロギングの初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 設定の読み込み
    let config = config::Config::from_env()?;
    tracing::info!("Starting server with config: {:?}", config);

    // データベース接続
    let pool = repository::db::create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // 接続テスト
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    tracing::info!("Database connection test successful");

    tracing::info!("Server initialization complete");

    Ok(())
}
