use crate::model::Stream;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait StreamRepository: Send + Sync {
    async fn create(&self, stream: &Stream) -> Result<Stream>;
    async fn find_by_id(&self, stream_id: Uuid) -> Result<Option<Stream>>;
    async fn find_all(
        &self,
        category: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<(Vec<Stream>, i64)>;
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
            INSERT INTO streams (stream_id, user_id, title, description, category, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING stream_id, user_id, title, description, category, created_at, deleted_at
            "#,
            stream.stream_id,
            stream.user_id,
            stream.title,
            stream.description,
            stream.category,
            stream.created_at,
            stream.deleted_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_stream)
    }

    async fn find_by_id(&self, stream_id: Uuid) -> Result<Option<Stream>> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            SELECT stream_id, user_id, title, description, category, created_at, deleted_at
            FROM streams
            WHERE stream_id = $1 AND deleted_at IS NULL
            "#,
            stream_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(stream)
    }

    async fn find_all(
        &self,
        category: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<(Vec<Stream>, i64)> {
        let limit = limit.unwrap_or(10) as i64;
        let offset = offset.unwrap_or(0) as i64;

        let streams = sqlx::query_as!(
            Stream,
            r#"
            SELECT stream_id, user_id, title, description, category, created_at, deleted_at
            FROM streams
            WHERE deleted_at IS NULL
            AND ($1::text IS NULL OR category = $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            category,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM streams
            WHERE deleted_at IS NULL
            AND ($1::text IS NULL OR category = $1)
            "#,
            category
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        Ok((streams, total))
    }

    async fn delete(&self, stream_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streams
            SET deleted_at = NOW()
            WHERE stream_id = $1
            "#,
            stream_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    async fn setup_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                "postgresql://devuser:devpassword@localhost:5432/devdb".to_string()
            });

        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_create_and_find_by_id() {
        let pool = setup_test_pool().await;
        let repo = StreamRepositoryImpl::new(pool);

        // 既存のユーザーIDを使用（テストデータで作成済み）
        let existing_user_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111")
            .expect("Invalid UUID");

        let stream = Stream {
            stream_id: Uuid::new_v4(),
            user_id: existing_user_id,
            title: "Test Stream".to_string(),
            description: "Test Description".to_string(),
            category: "".to_string(),
            created_at: Utc::now(),
            deleted_at: None,
        };

        // Create
        let created = repo.create(&stream).await.expect("Failed to create stream");
        assert_eq!(created.stream_id, stream.stream_id);
        assert_eq!(created.title, stream.title);

        // Find by ID
        let found = repo
            .find_by_id(stream.stream_id)
            .await
            .expect("Failed to find stream")
            .expect("Stream not found");
        assert_eq!(found.stream_id, stream.stream_id);
        assert_eq!(found.title, stream.title);

        // Cleanup
        repo.delete(stream.stream_id).await.expect("Failed to delete stream");
    }

    #[tokio::test]
    async fn test_find_all() {
        let pool = setup_test_pool().await;
        let repo = StreamRepositoryImpl::new(pool);

        // 既存のユーザーIDを使用
        let existing_user_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111")
            .expect("Invalid UUID");

        let stream1 = Stream {
            stream_id: Uuid::new_v4(),
            user_id: existing_user_id,
            title: "Test Stream 1".to_string(),
            description: "Description 1".to_string(),
            category: "".to_string(),
            created_at: Utc::now(),
            deleted_at: None,
        };

        let stream2 = Stream {
            stream_id: Uuid::new_v4(),
            user_id: existing_user_id,
            title: "Test Stream 2".to_string(),
            description: "Description 2".to_string(),
            category: "".to_string(),
            created_at: Utc::now(),
            deleted_at: None,
        };

        // Create test data
        repo.create(&stream1).await.expect("Failed to create stream1");
        repo.create(&stream2).await.expect("Failed to create stream2");

        // Find all
        let (streams, _) = repo.find_all(None, None, None).await.expect("Failed to find all streams");
        assert!(streams.len() >= 2);

        // Cleanup
        repo.delete(stream1.stream_id).await.expect("Failed to delete stream1");
        repo.delete(stream2.stream_id).await.expect("Failed to delete stream2");
    }

    #[tokio::test]
    async fn test_delete() {
        let pool = setup_test_pool().await;
        let repo = StreamRepositoryImpl::new(pool);

        // 既存のユーザーIDを使用
        let existing_user_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111")
            .expect("Invalid UUID");

        let stream = Stream {
            stream_id: Uuid::new_v4(),
            user_id: existing_user_id,
            title: "Test Stream to Delete".to_string(),
            description: "Will be deleted".to_string(),
            category: "".to_string(),
            created_at: Utc::now(),
            deleted_at: None,
        };

        // Create
        repo.create(&stream).await.expect("Failed to create stream");

        // Delete
        repo.delete(stream.stream_id).await.expect("Failed to delete stream");

        // Verify deletion
        let found = repo
            .find_by_id(stream.stream_id)
            .await
            .expect("Failed to query");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let pool = setup_test_pool().await;
        let repo = StreamRepositoryImpl::new(pool);

        let non_existent_id = Uuid::new_v4();
        let result = repo
            .find_by_id(non_existent_id)
            .await
            .expect("Failed to query");
        assert!(result.is_none());
    }
}
