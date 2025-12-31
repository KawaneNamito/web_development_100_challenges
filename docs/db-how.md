# ローカル DB 開発環境

## 概要

Docker Compose を使用して PostgreSQL をローカルで起動し、バックエンドから接続する開発環境です。

## 前提条件

- Docker / Docker Compose がインストールされていること

## ファイル構成

| ファイル                   | 説明                                   |
| -------------------------- | -------------------------------------- |
| `docker-compose.yml`       | PostgreSQL 16 コンテナ定義             |
| `.env`                     | 環境変数（ローカル用、gitignore 対象） |
| `.env.example`             | 環境変数のテンプレート                 |
| `infra/database/migration` | DB 初期化 SQL                          |

## 環境変数

`.env.example` をコピーして `.env` を作成してください。

```sh
cp .env.example .env
```

| 変数名              | デフォルト値                                          | 説明           |
| ------------------- | ----------------------------------------------------- | -------------- |
| `POSTGRES_USER`     | devuser                                               | DB ユーザー名  |
| `POSTGRES_PASSWORD` | devpassword                                           | DB パスワード  |
| `POSTGRES_DB`       | devdb                                                 | データベース名 |
| `DATABASE_URL`      | postgresql://devuser:devpassword@localhost:5432/devdb | 接続文字列     |

## 使い方

```sh
# 起動
docker compose up -d

# ログ確認
docker compose logs -f postgres

# 停止
docker compose down

# データも含めて完全削除
docker compose down -v
```

## データベース接続

### Docker コンテナ経由で psql を使用（psql のインストール不要）

```powershell
# インタラクティブシェルに接続
docker exec -it dev-postgres psql -U devuser -d devdb

# クエリを直接実行
docker exec dev-postgres psql -U devuser -d devdb -c "SELECT * FROM users;"

# テーブル一覧を表示
docker exec dev-postgres psql -U devuser -d devdb -c "\dt"

# SQLファイルを実行
docker exec -i dev-postgres psql -U devuser -d devdb < your-script.sql
```

### psql がインストール済みの場合

```sh
psql -h localhost -U devuser -d devdb
# パスワード: devpassword
```

### よく使う psql コマンド

インタラクティブシェル内で使用できるコマンド：

```sql
\dt              -- テーブル一覧
\d table_name    -- テーブル構造を表示
\l               -- データベース一覧
\du              -- ユーザー一覧
\c database      -- データベースを切り替え
\q               -- 終了
```

## トラブルシューティング

### ポート 5432 が既に使用されている

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
