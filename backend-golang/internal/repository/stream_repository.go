package repository

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
)

// Stream はstreamsテーブルのエンティティ
type Stream struct {
	StreamID    uuid.UUID
	UserID      uuid.UUID
	Title       string
	Description string
	CreatedAt   time.Time
}

// StreamRepository はストリームのリポジトリインターフェース
type StreamRepository interface {
	Create(ctx context.Context, stream *Stream) error
	FindByID(ctx context.Context, streamID uuid.UUID) (*Stream, error)
	FindByUserID(ctx context.Context, userID uuid.UUID) ([]*Stream, error)
	FindAll(ctx context.Context) ([]*Stream, error)
	Update(ctx context.Context, stream *Stream) error
	Delete(ctx context.Context, streamID uuid.UUID) error
}

// streamRepository はStreamRepositoryの実装
type streamRepository struct {
	db *DB
}

// NewStreamRepository は新しいStreamRepositoryを作成する
func NewStreamRepository(db *DB) StreamRepository {
	return &streamRepository{db: db}
}

// Create は新しいストリームを作成する
func (r *streamRepository) Create(ctx context.Context, stream *Stream) error {
	query := `
		INSERT INTO streams (stream_id, user_id, title, description, created_at)
		VALUES ($1, $2, $3, $4, $5)
	`
	_, err := r.db.Pool.Exec(ctx, query,
		stream.StreamID,
		stream.UserID,
		stream.Title,
		stream.Description,
		stream.CreatedAt,
	)
	return err
}

// FindByID はストリームIDからストリームを取得する
func (r *streamRepository) FindByID(ctx context.Context, streamID uuid.UUID) (*Stream, error) {
	query := `
		SELECT stream_id, user_id, title, description, created_at
		FROM streams
		WHERE stream_id = $1
	`
	row := r.db.Pool.QueryRow(ctx, query, streamID)

	var stream Stream
	if err := row.Scan(
		&stream.StreamID,
		&stream.UserID,
		&stream.Title,
		&stream.Description,
		&stream.CreatedAt,
	); err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &stream, nil
}

// FindByUserID はユーザーIDからストリームを取得する
func (r *streamRepository) FindByUserID(ctx context.Context, userID uuid.UUID) ([]*Stream, error) {
	query := `
		SELECT stream_id, user_id, title, description, created_at
		FROM streams
		WHERE user_id = $1
		ORDER BY created_at DESC
	`
	rows, err := r.db.Pool.Query(ctx, query, userID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var streams []*Stream
	for rows.Next() {
		var stream Stream
		if err := rows.Scan(
			&stream.StreamID,
			&stream.UserID,
			&stream.Title,
			&stream.Description,
			&stream.CreatedAt,
		); err != nil {
			return nil, err
		}
		streams = append(streams, &stream)
	}
	return streams, rows.Err()
}

// FindAll は全てのストリームを取得する
func (r *streamRepository) FindAll(ctx context.Context) ([]*Stream, error) {
	query := `
		SELECT stream_id, user_id, title, description, created_at
		FROM streams
		ORDER BY created_at DESC
	`
	rows, err := r.db.Pool.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var streams []*Stream
	for rows.Next() {
		var stream Stream
		if err := rows.Scan(
			&stream.StreamID,
			&stream.UserID,
			&stream.Title,
			&stream.Description,
			&stream.CreatedAt,
		); err != nil {
			return nil, err
		}
		streams = append(streams, &stream)
	}
	return streams, rows.Err()
}

// Update はストリームを更新する
func (r *streamRepository) Update(ctx context.Context, stream *Stream) error {
	query := `
		UPDATE streams
		SET title = $2, description = $3
		WHERE stream_id = $1
	`
	_, err := r.db.Pool.Exec(ctx, query,
		stream.StreamID,
		stream.Title,
		stream.Description,
	)
	return err
}

// Delete はストリームを削除する
func (r *streamRepository) Delete(ctx context.Context, streamID uuid.UUID) error {
	query := `DELETE FROM streams WHERE stream_id = $1`
	_, err := r.db.Pool.Exec(ctx, query, streamID)
	return err
}
