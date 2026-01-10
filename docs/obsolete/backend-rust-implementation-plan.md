# backend-rust 実装計画

## 概要

backend-golangと同様の機能を持つRust製APIサーバーを実装する。
同じAPI仕様、同じデータベーススキーマを使用し、Go実装と互換性のあるサーバーを構築する。

## 技術スタック

### Webフレームワーク

**Axum 0.7.x**
- Tokioエコシステムとの親和性が高い
- 型安全なルーティング
- Towerミドルウェアとの統合
- 非同期処理のパフォーマンスが高い
- エラーハンドリングが洗練されている

代替案:
- Actix-web: 高速だがAPI設計がやや複雑
- Rocket: シンプルだが非同期サポートがAxumより弱い

### データベースアクセス

**SQLx 0.8.x**
- コンパイル時にSQLをチェック
- PostgreSQL対応
- マクロによる型安全なクエリ
- 接続プールの管理が容易
- 非同期対応

```rust
sqlx::query_as!(Stream, "SELECT * FROM streams WHERE stream_id = $1", id)
```

代替案:
- Diesel: ORMとして強力だがコンパイル時間が長い
- SeaORM: 非同期対応のORMだが学習コストがやや高い

### OpenAPI連携

**OpenAPI Generator (openapi-generator-cli)**
- OpenAPI仕様からRustのスキーマ定義を生成
- serdeと統合してJSON serialization/deserializationに対応
- バリデーション用の型定義を生成

生成方法:
```bash
openapi-generator-cli generate \
  -i docs/api.yml \
  -g rust \
  -o backend-rust/generated \
  --additional-properties=packageName=openapi_types
```

代替案:
- 手動で型を定義: メンテナンスコストが高い
- openapi: クレートの選択肢が多く迷いやすい

### シリアライゼーション

**serde 1.x + serde_json**
- Rustのデファクトスタンダード
- 高速なJSON処理
- derive macroで簡単に実装

### UUID処理

**uuid 1.x**
- UUID v4の生成
- PostgreSQLのUUID型との互換性
- serdeとの統合

### 環境変数管理

**dotenvy 0.15.x**
- `.env`ファイルの読み込み
- Go版のgodotenvと同等の機能

### 日時処理

**chrono 0.4.x**
- タイムゾーン対応
- PostgreSQLのTIMESTAMPTZ型との互換性
- serdeとの統合

### CORS対応

**tower-http 0.6.x**
- Axumと統合されたミドルウェア
- CORSヘッダーの設定が簡単

### エラーハンドリング

**thiserror 2.x + anyhow 1.x**
- thiserror: カスタムエラー型の定義
- anyhow: エラーの伝搬と文脈の追加

### 非同期ランタイム

**tokio 1.x**
- 非同期処理のランタイム
- Axumの必須依存

## アーキテクチャ設計

### ディレクトリ構成

```
backend-rust/
├── Cargo.toml
├── .env.example
├── src/
│   ├── main.rs              # エントリーポイント、サーバー起動
│   ├── config.rs            # 環境変数の読み込み
│   ├── error.rs             # エラー型定義
│   ├── handler/
│   │   ├── mod.rs           # ハンドラーモジュール
│   │   └── stream.rs        # ストリーム関連のハンドラー
│   ├── repository/
│   │   ├── mod.rs           # リポジトリモジュール
│   │   ├── db.rs            # DB接続管理
│   │   └── stream.rs        # ストリームリポジトリ
│   ├── model/
│   │   ├── mod.rs           # モデルモジュール
│   │   └── stream.rs        # ストリームのドメインモデル
│   └── schema/
│       ├── mod.rs           # スキーマモジュール
│       └── api.rs           # OpenAPIから生成されたスキーマ
└── migrations/              # SQLxのマイグレーションファイル
    └── 20250101000000_init.sql
```

### レイヤー構成

```
main.rs (サーバー起動)
    ↓
handler (HTTPリクエスト/レスポンス)
    ↓
repository (データベースアクセス)
    ↓
PostgreSQL
```

1. **main.rs**: サーバーの初期化、ルーティング設定、グレースフルシャットダウン
2. **handler**: HTTPリクエストを受け取り、バリデーション、レスポンス生成
3. **repository**: SQLクエリの実行、トランザクション管理
4. **model**: ドメインモデル（Stream構造体など）
5. **schema**: OpenAPI仕様から生成されたリクエスト/レスポンス型

### 依存性注入パターン

Go版と同様に、リポジトリをハンドラーに注入する設計:

```rust
struct AppState {
    stream_repo: Arc<dyn StreamRepository>,
}

// ルーティング時にAppStateを共有
Router::new()
    .route("/api/v1/streams", get(get_streams))
    .with_state(Arc::new(app_state))
```

### エラーハンドリング戦略

```rust
// カスタムエラー型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

// AxumのIntoResponseを実装
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // エラーコードとメッセージを返す
    }
}
```

## 実装段取り

### フェーズ1: プロジェクト初期化とDB接続 (基盤構築)

1. **Cargo.tomlの依存関係設定**
   - Axum, Tokio, SQLx, serde, uuid, chrono, dotenvy, tower-http, thiserror, anyhow
   - `sqlx`の`runtime-tokio`, `postgres`, `macros`, `chrono`, `uuid`フィーチャーを有効化

2. **環境変数とDB接続の実装**
   - `config.rs`: DATABASE_URL, PORTの読み込み
   - `repository/db.rs`: SQLxのPgPoolを使った接続プール管理
   - `.env.example`の作成

3. **動作確認**
   - サーバーが起動しDBに接続できることを確認

### フェーズ2: モデルとリポジトリの実装 (データ層)

1. **ドメインモデルの定義**
   - `model/stream.rs`: Stream構造体
   - UUID, DateTime<Utc>を使用
   - serdeのderive(Serialize, Deserialize)

2. **リポジトリインターフェースと実装**
   - `repository/stream.rs`
   - StreamRepositoryトレイト定義
   - CRUD操作の実装:
     - `create()`: INSERT
     - `find_by_id()`: SELECT (単一)
     - `find_all()`: SELECT (全件)
     - `delete()`: DELETE
   - SQLx macroを使用: `sqlx::query_as!`

3. **動作確認**
   - 単体テストまたは手動でのDB操作確認

### フェーズ3: OpenAPIスキーマ生成とハンドラー実装 (API層)

1. **OpenAPIスキーマの生成**
   - openapi-generator-cliでRustの型定義を生成
   - `schema/api.rs`に配置
   - Stream, StreamSummary, ValidationError, ServerErrorの型

2. **エラーハンドリングの実装**
   - `error.rs`: AppError定義
   - IntoResponseの実装
   - ValidationError, ServerError, NotFoundのJSON変換

3. **ハンドラーの実装**
   - `handler/stream.rs`:
     - `POST /api/v1/streams`: 配信情報の登録
     - `GET /api/v1/streams`: 配信情報の一覧取得
     - `GET /api/v1/streams/{streamId}`: 配信情報の詳細取得
     - `DELETE /api/v1/streams/{streamId}`: 配信情報の削除
   - リクエストのバリデーション
   - レスポンスの型変換

4. **ルーティングの設定**
   - `main.rs`でAxumのRouterを設定
   - CORSミドルウェアの追加
   - ロギングミドルウェアの追加

5. **動作確認**
   - curlまたはPostmanでAPIをテスト

### フェーズ4: 本番対応機能の実装 (運用対応)

1. **グレースフルシャットダウン**
   - SIGINTやSIGTERMシグナルのハンドリング
   - 既存のリクエストを完了させてからサーバー停止

2. **ロギングの追加**
   - `tracing`と`tracing-subscriber`の導入
   - リクエストログ、エラーログの出力

3. **ヘルスチェックエンドポイント**
   - `GET /health`: サーバーの稼働状態確認
   - DB接続のチェック

4. **READMEの作成**
   - 環境構築手順
   - ビルド・実行方法
   - APIの使用例

### フェーズ5: テストとドキュメント (品質保証)

1. **統合テストの作成**
   - `tests/`ディレクトリにAPIテスト
   - テスト用のDBセットアップ

2. **エラーハンドリングのテスト**
   - バリデーションエラー
   - 404エラー
   - 500エラー

3. **ドキュメントの充実**
   - コード内のdocコメント
   - 設計判断の記録

## Go実装との対応表

| Go | Rust |
|---|---|
| chi/v5 | Axum |
| pgx/v5 | SQLx |
| oapi-codegen | openapi-generator-cli (手動生成) |
| godotenv | dotenvy |
| google/uuid | uuid |
| time.Time | chrono::DateTime<Utc> |
| context.Context | 不要 (Tokioの非同期コンテキスト) |
| http.ResponseWriter | axum::response::IntoResponse |
| json.Marshal/Unmarshal | serde_json::to_string/from_str |

## 実装のポイント

### 1. 型安全性の活用

Rustの強力な型システムを活用:
- UUIDは`uuid::Uuid`型で厳密に扱う
- Option<T>でNULL値を明示
- Result<T, E>でエラーを型で表現

### 2. 非同期処理

全てのI/O処理を非同期化:
```rust
async fn create_stream(
    State(repo): State<Arc<dyn StreamRepository>>,
    Json(req): Json<CreateStreamRequest>,
) -> Result<Json<Stream>, AppError> {
    // 非同期処理
}
```

### 3. SQLxのコンパイル時チェック

開発時に`DATABASE_URL`を設定し、`sqlx::query_as!`でSQLの型チェック:
```bash
export DATABASE_URL=postgresql://devuser:devpassword@localhost:5432/devdb
cargo build
```

### 4. エラーメッセージの一貫性

Go版と同じエラーコードとメッセージ形式を保つ:
```json
{
  "error": "validation_error",
  "message": "Invalid userId format"
}
```

### 5. CORSとミドルウェア

Go版と同じCORS設定:
```rust
CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::DELETE])
    .allow_headers([ACCEPT, CONTENT_TYPE])
```

## 期待される成果物

1. **動作するAPIサーバー**
   - Go版と同一のAPI仕様を実装
   - PostgreSQLに接続してCRUD操作を実行
   - エラーハンドリングが適切

2. **ドキュメント**
   - README.md (セットアップ手順、使用方法)
   - コード内のdocコメント

3. **テストコード**
   - 統合テスト
   - エラーケースのテスト

4. **パフォーマンス**
   - Go版と同等以上のレスポンスタイム
   - メモリ使用量の最適化

## 追加の検討事項

### OpenAPI自動バリデーション

現状は手動でバリデーションを実装するが、将来的には:
- `garde`や`validator`クレートでバリデーションを自動化
- OpenAPI仕様のmaxLength制約をコードで反映

### マイグレーション管理

SQLxの`sqlx-cli`を使用してマイグレーション管理:
```bash
sqlx migrate add init
sqlx migrate run
```

### CI/CD対応

- GitHub Actionsでビルド・テスト
- Dockerイメージのビルド
- リンターとフォーマッターの実行 (clippy, rustfmt)

### パフォーマンス最適化

- リリースビルドでのLTO (Link Time Optimization)
- コネクションプールのチューニング
- Jemalloc allocatorの使用検討

## まとめ

この実装計画に従うことで、backend-golangと同等の機能を持つRust製APIサーバーを段階的に構築できる。
Rustの型安全性と非同期処理の性能を活かしつつ、既存のAPI仕様とデータベーススキーマに準拠したサーバーを実現する。
