# ローカルDB開発環境

## 概要

Docker Composeを使用してPostgreSQLをローカルで起動し、Rustから接続する開発環境です。

## 前提条件

- Docker / Docker Compose がインストールされていること
- psql クライアント（接続テスト用、オプション）

## ファイル構成

| ファイル | 説明 |
|---------|------|
| `docker-compose.yml` | PostgreSQL 16 コンテナ定義 |
| `.env` | 環境変数（ローカル用、gitignore対象） |
| `.env.example` | 環境変数のテンプレート |
| `init.sql` | DB初期化SQL |

## 環境変数

`.env.example` をコピーして `.env` を作成してください。

```sh
cp .env.example .env
```

| 変数名 | デフォルト値 | 説明 |
|--------|-------------|------|
| `POSTGRES_USER` | devuser | DBユーザー名 |
| `POSTGRES_PASSWORD` | devpassword | DBパスワード |
| `POSTGRES_DB` | devdb | データベース名 |
| `DATABASE_URL` | postgresql://devuser:devpassword@localhost:5432/devdb | Rust用接続文字列 |

## 使い方

```sh
# 起動
docker compose up -d

# ログ確認
docker compose logs -f postgres

# 接続テスト
psql -h localhost -U devuser -d devdb
# パスワード: devpassword

# 停止
docker compose down

# データも含めて完全削除
docker compose down -v
```

## Rustからの接続

```rust
// 接続文字列
let url = "postgresql://devuser:devpassword@localhost:5432/devdb";
// または環境変数から
let url = std::env::var("DATABASE_URL").unwrap();
```

### 推奨クレート

`Cargo.toml` に追加：

```toml
[dependencies]
# 同期版
postgres = "0.19"

# 非同期版
tokio-postgres = "0.7"
tokio = { version = "1", features = ["full"] }

# ORM（どちらか選択）
# diesel = { version = "2.1", features = ["postgres"] }
# sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
```

## トラブルシューティング

### ポート5432が既に使用されている

```sh
# 使用中のプロセスを確認
lsof -i :5432

# ローカルのPostgreSQLを停止
sudo systemctl stop postgresql
```

### コンテナが起動しない

```sh
# ログを確認
docker compose logs postgres

# ボリュームを削除して再作成
docker compose down -v
docker compose up -d
```

### 接続できない

```sh
# コンテナの状態を確認
docker compose ps

# ヘルスチェック状態を確認
docker inspect dev-postgres --format='{{.State.Health.Status}}'
```
