package repository

import (
	"context"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
)

// User はusersテーブルのエンティティ
type User struct {
	UserID uuid.UUID
}

// UserRepository はユーザーのリポジトリインターフェース
type UserRepository interface {
	Create(ctx context.Context, user *User) error
	FindByID(ctx context.Context, userID uuid.UUID) (*User, error)
	FindAll(ctx context.Context) ([]*User, error)
	Delete(ctx context.Context, userID uuid.UUID) error
}

// userRepository はUserRepositoryの実装
type userRepository struct {
	db *DB
}

// NewUserRepository は新しいUserRepositoryを作成する
func NewUserRepository(db *DB) UserRepository {
	return &userRepository{db: db}
}

// Create は新しいユーザーを作成する
func (r *userRepository) Create(ctx context.Context, user *User) error {
	query := `INSERT INTO users (user_id) VALUES ($1)`
	_, err := r.db.Pool.Exec(ctx, query, user.UserID)
	return err
}

// FindByID はユーザーIDからユーザーを取得する
func (r *userRepository) FindByID(ctx context.Context, userID uuid.UUID) (*User, error) {
	query := `SELECT user_id FROM users WHERE user_id = $1`
	row := r.db.Pool.QueryRow(ctx, query, userID)

	var user User
	if err := row.Scan(&user.UserID); err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &user, nil
}

// FindAll は全てのユーザーを取得する
func (r *userRepository) FindAll(ctx context.Context) ([]*User, error) {
	query := `SELECT user_id FROM users`
	rows, err := r.db.Pool.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		var user User
		if err := rows.Scan(&user.UserID); err != nil {
			return nil, err
		}
		users = append(users, &user)
	}
	return users, rows.Err()
}

// Delete はユーザーを削除する
func (r *userRepository) Delete(ctx context.Context, userID uuid.UUID) error {
	query := `DELETE FROM users WHERE user_id = $1`
	_, err := r.db.Pool.Exec(ctx, query, userID)
	return err
}
