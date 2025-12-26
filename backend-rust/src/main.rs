mod config;
mod error;
mod handler;
mod model;
mod repository;
mod schema;

use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
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
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Database connection test successful");

    // リポジトリの初期化
    let stream_repo: Arc<dyn repository::StreamRepository> =
        Arc::new(repository::StreamRepositoryImpl::new(pool));

    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ルーターの設定
    let app = Router::new()
        .route("/api/v1/streams", post(handler::create_stream))
        .route("/api/v1/streams", get(handler::get_streams))
        .route("/api/v1/streams/:stream_id", get(handler::get_stream))
        .route("/api/v1/streams/:stream_id", delete(handler::delete_stream))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(stream_repo);

    // サーバー起動
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server stopped");

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}
