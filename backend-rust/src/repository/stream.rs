use crate::model::Stream;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait StreamRepository: Send + Sync {
    async fn create(&self, stream: &Stream) -> Result<Stream>;
    async fn find_by_id(&self, stream_id: Uuid) -> Result<Option<Stream>>;
    async fn find_all(&self) -> Result<Vec<Stream>>;
    async fn delete(&self, stream_id: Uuid) -> Result<()>;
}

pub struct StreamRepositoryImpl {
    pool: PgPool,
}

impl StreamRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StreamRepository for StreamRepositoryImpl {
    async fn create(&self, stream: &Stream) -> Result<Stream> {
        let created_stream = sqlx::query_as!(
            Stream,
            r#"
            INSERT INTO streams (stream_id, user_id, title, description, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING stream_id, user_id, title, description, created_at
            "#,
            stream.stream_id,
            stream.user_id,
            stream.title,
            stream.description,
            stream.created_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_stream)
    }

    async fn find_by_id(&self, stream_id: Uuid) -> Result<Option<Stream>> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            SELECT stream_id, user_id, title, description, created_at
            FROM streams
            WHERE stream_id = $1
            "#,
            stream_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(stream)
    }

    async fn find_all(&self) -> Result<Vec<Stream>> {
        let streams = sqlx::query_as!(
            Stream,
            r#"
            SELECT stream_id, user_id, title, description, created_at
            FROM streams
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(streams)
    }

    async fn delete(&self, stream_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM streams
            WHERE stream_id = $1
            "#,
            stream_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
