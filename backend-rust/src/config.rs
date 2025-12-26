use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // .envファイルを読み込む（存在しない場合はスキップ）
        let _ = dotenvy::from_filename(".env")
            .or_else(|_| dotenvy::from_filename("../.env"))
            .or_else(|_| dotenvy::from_filename("../../.env"));

        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .context("PORT must be a valid number")?;

        Ok(Self {
            database_url,
            port,
        })
    }
}
