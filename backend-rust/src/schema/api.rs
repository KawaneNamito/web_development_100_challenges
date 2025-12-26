use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// リクエスト型
#[derive(Debug, Deserialize)]
pub struct CreateStreamRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub title: String,
    pub description: String,
}

// レスポンス型
#[derive(Debug, Serialize)]
pub struct StreamResponse {
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StreamSummaryResponse {
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StreamListResponse {
    pub streams: Vec<StreamSummaryResponse>,
}
