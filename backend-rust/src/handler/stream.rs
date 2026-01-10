use crate::{
    error::AppError,
    model::Stream,
    repository::StreamRepository,
    schema::{
        CreateStreamRequest, StreamListResponse, StreamResponse, StreamSummaryResponse,
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListStreamsQuery {
    pub category: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub async fn create_stream(
    State(repo): State<Arc<dyn StreamRepository>>,
    Json(req): Json<CreateStreamRequest>,
) -> Result<(StatusCode, Json<StreamResponse>), AppError> {
    // UUIDのバリデーション
    let user_id = Uuid::parse_str(&req.user_id)
        .map_err(|_| AppError::Validation("Invalid userId format".to_string()))?;

    // タイトルの長さチェック
    if req.title.is_empty() {
        return Err(AppError::Validation("タイトルは必須です".to_string()));
    }

    // 説明文の長さチェック
    if req.description.len() > 500 {
        return Err(AppError::Validation(
            "概要欄は500文字以内で入力してください".to_string(),
        ));
    }

    let stream = Stream {
        stream_id: Uuid::new_v4(),
        user_id,
        title: req.title,
        description: req.description,
        category: req.category.unwrap_or_default(),
        created_at: Utc::now(),
        deleted_at: None,
    };

    let created = repo.create(&stream).await?;

    let response = StreamResponse {
        stream_id: Some(created.stream_id.to_string()),
        user_id: Some(created.user_id.to_string()),
        title: Some(created.title),
        description: Some(created.description),
        category: Some(created.category),
        created_at: Some(created.created_at.to_rfc3339()),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_streams(
    State(repo): State<Arc<dyn StreamRepository>>,
    Query(query): Query<ListStreamsQuery>,
) -> Result<Json<StreamListResponse>, AppError> {
    if let Some(limit) = query.limit {
        if limit > 100 {
            return Err(AppError::Validation(
                "limitは100以下で指定してください".to_string(),
            ));
        }
    }

    let (streams, total) = repo
        .find_all(query.category, query.limit, query.offset)
        .await?;

    let response = StreamListResponse {
        total: Some(total as i32),
        limit: Some(query.limit.unwrap_or(10)),
        offset: Some(query.offset.unwrap_or(0)),
        items: Some(
            streams
                .into_iter()
                .map(|s| StreamSummaryResponse {
                    stream_id: Some(s.stream_id.to_string()),
                    user_id: Some(s.user_id.to_string()),
                    title: Some(s.title),
                    category: Some(s.category),
                    created_at: Some(s.created_at.to_rfc3339()),
                })
                .collect(),
        ),
    };

    Ok(Json(response))
}

pub async fn get_stream(
    State(repo): State<Arc<dyn StreamRepository>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StreamResponse>, AppError> {
    let id = Uuid::parse_str(&stream_id)
        .map_err(|_| AppError::Validation("Invalid streamId format".to_string()))?;

    let stream = repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Stream not found".to_string()))?;

    let response = StreamResponse {
        stream_id: Some(stream.stream_id.to_string()),
        user_id: Some(stream.user_id.to_string()),
        title: Some(stream.title),
        description: Some(stream.description),
        category: Some(stream.category),
        created_at: Some(stream.created_at.to_rfc3339()),
    };

    Ok(Json(response))
}

pub async fn delete_stream(
    State(repo): State<Arc<dyn StreamRepository>>,
    Path(stream_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let id = Uuid::parse_str(&stream_id)
        .map_err(|_| AppError::Validation("Invalid streamId format".to_string()))?;

    repo.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockStreamRepository {
        streams: Mutex<Vec<Stream>>,
    }

    impl MockStreamRepository {
        fn new() -> Self {
            Self {
                streams: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl StreamRepository for MockStreamRepository {
        async fn create(&self, stream: &Stream) -> anyhow::Result<Stream> {
            let mut streams = self.streams.lock().unwrap();
            streams.push(stream.clone());
            Ok(stream.clone())
        }

        async fn find_by_id(&self, _stream_id: Uuid) -> anyhow::Result<Option<Stream>> {
            Ok(None)
        }

        async fn find_all(
            &self,
            _category: Option<String>,
            _limit: Option<i32>,
            _offset: Option<i32>,
        ) -> anyhow::Result<(Vec<Stream>, i64)> {
            let streams = self.streams.lock().unwrap();
            Ok((streams.clone(), streams.len() as i64))
        }

        async fn delete(&self, _stream_id: Uuid) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_stream_validation() {
        let repo = Arc::new(MockStreamRepository::new());

        // Empty title
        let req = CreateStreamRequest {
            user_id: Uuid::new_v4().to_string(),
            title: "".to_string(),
            description: "desc".to_string(),
            category: None,
        };
        let result = create_stream(State(repo.clone()), Json(req)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert_eq!(msg, "タイトルは必須です"),
            _ => panic!("Unexpected error type"),
        }

        // Long description
        let req_long = CreateStreamRequest {
            user_id: Uuid::new_v4().to_string(),
            title: "title".to_string(),
            description: "a".repeat(501),
            category: None,
        };
        let result = create_stream(State(repo.clone()), Json(req_long)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert_eq!(msg, "概要欄は500文字以内で入力してください"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[tokio::test]
    async fn test_get_streams_limit_validation() {
        let repo = Arc::new(MockStreamRepository::new());
        let query = ListStreamsQuery {
            category: None,
            limit: Some(101),
            offset: None,
        };

        let result = get_streams(State(repo), Query(query)).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Validation(msg) => assert_eq!(msg, "limitは100以下で指定してください"),
            _ => panic!("Unexpected error type"),
        }
    }
}
