use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Stream {
    pub stream_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
