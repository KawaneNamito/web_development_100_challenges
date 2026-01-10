# backend-rust

Rust製のYouTube配信情報管理APIサーバー

## 技術スタック

- **Axum 0.7**: Webフレームワーク
- **SQLx 0.8**: データベースアクセス（PostgreSQL）
- **Tokio**: 非同期ランタイム
- **serde**: JSON シリアライゼーション
- **tower-http**: CORS、トレーシング

## 前提条件

- Rust 1.75以上
- PostgreSQL 16
- Docker / Docker Compose（ローカルDB起動用）

## セットアップ

### 1. 環境変数の設定

```bash
cp .env.example .env
```

`.env`ファイルの内容:
```
DATABASE_URL=postgresql://devuser:devpassword@localhost:5432/devdb
PORT=8080
```

### 2. データベースの起動

プロジェクトルートで:

```bash
docker compose up -d
```

### 3. ビルドと実行

```bash
# ビルド
cargo build --release

# 実行
cargo run --release
```

サーバーは `http://localhost:8080` で起動します。

## API エンドポイント

### 配信情報の登録

```bash
POST /api/v1/streams
Content-Type: application/json

{
  "userId": "11111111-1111-1111-1111-111111111111",
  "title": "今日のライブ配信",
  "description": "ゲーム実況をします"
}
```

### 配信情報の一覧取得

```bash
GET /api/v1/streams
```

### 配信情報の詳細取得

```bash
GET /api/v1/streams/{streamId}
```

### 配信情報の削除

```bash
DELETE /api/v1/streams/{streamId}
```

## 開発

### フォーマット

```bash
cargo fmt
```

### リンター

```bash
cargo clippy
```

### テスト

```bash
cargo test
```

## ディレクトリ構成

```
backend-rust/
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── config.rs        # 環境変数読み込み
│   ├── error.rs         # エラーハンドリング
│   ├── handler/         # HTTPハンドラー
│   ├── repository/      # データベースアクセス
│   ├── model/           # ドメインモデル
│   └── schema/          # リクエスト/レスポンス型
├── Cargo.toml
└── README.md
```

## トラブルシューティング

### データベース接続エラー

PostgreSQLが起動しているか確認:

```bash
docker compose ps
```

### ポートが使用中

`.env`の`PORT`を変更してください。

## ライセンス

MIT
