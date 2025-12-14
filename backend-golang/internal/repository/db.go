package repository

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5/pgxpool"
)

// DB はデータベース接続プールを保持する構造体
type DB struct {
	Pool *pgxpool.Pool
}

// NewDB は新しいデータベース接続プールを作成する
func NewDB(ctx context.Context, connString string) (*DB, error) {
	pool, err := pgxpool.New(ctx, connString)
	if err != nil {
		return nil, fmt.Errorf("failed to create connection pool: %w", err)
	}

	// 接続テスト
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("failed to ping database: %w", err)
	}

	return &DB{Pool: pool}, nil
}

// Close はデータベース接続プールを閉じる
func (db *DB) Close() {
	if db.Pool != nil {
		db.Pool.Close()
	}
}
