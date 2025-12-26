use crate::{
    error::AppError,
    model::Stream,
    repository::StreamRepository,
    schema::{CreateStreamRequest, StreamListResponse, StreamResponse, StreamSummaryResponse},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

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
        created_at: Utc::now(),
    };

    let created = repo.create(&stream).await?;

    let response = StreamResponse {
        stream_id: created.stream_id.to_string(),
        user_id: created.user_id.to_string(),
        title: created.title,
        description: created.description,
        created_at: created.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_streams(
    State(repo): State<Arc<dyn StreamRepository>>,
) -> Result<Json<StreamListResponse>, AppError> {
    let streams = repo.find_all().await?;

    let response = StreamListResponse {
        streams: streams
            .into_iter()
            .map(|s| StreamSummaryResponse {
                stream_id: s.stream_id.to_string(),
                user_id: s.user_id.to_string(),
                title: s.title,
                created_at: s.created_at,
            })
            .collect(),
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
        stream_id: stream.stream_id.to_string(),
        user_id: stream.user_id.to_string(),
        title: stream.title,
        description: stream.description,
        created_at: stream.created_at,
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
