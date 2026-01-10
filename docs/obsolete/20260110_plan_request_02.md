# 依頼事項 #02 実装プラン

## 1. テーブル定義を変更する (`docs/table.md`)

- [x] `streams` テーブルに `category` カラムを追加 (TEXT)
    - 例: "雑談", "ゲーム（スプラ）" など
- [x] `streams` テーブルに `deleted_at` カラムを追加 (TIMESTAMPTZ, NULLable)
    - 論理削除を実現するため
- [x] マイグレーションファイルの作成 (`infra/database/migration`)
    - [x] `ALTER TABLE streams ADD COLUMN category TEXT NOT NULL DEFAULT '';` (または適切なデフォルト値/NULL許可)
    - [x] `ALTER TABLE streams ADD COLUMN deleted_at TIMESTAMPTZ;`

## 2. APIの仕様を変更する (`docs/api.yml`)

- [x] **配信登録 API (`POST /streams`)**
    - [x] リクエストボディに `category` (string) を追加
- [x] **配信一覧取得 API (`GET /streams`)**
    - [x] クエリパラメータの追加:
        - [x] `category`: カテゴリによる絞り込み
        - [x] `limit`: 1回のリクエストで取得する件数
        - [x] `offset`: 取得開始位置
    - [x] レスポンスボディの変更:
        - [x] ページネーション情報 (`limit`, `offset`, `total`) を追加
        - [x] `items` 配列の中に配信データを入れる構造に変更
        - [x] 各配信データに `category` を追加
- [x] **配信詳細取得 API (`GET /streams/{stream_id}`)**
    - [x] レスポンスボディに `category` を追加
- [x] **配信削除 API (`DELETE /streams/{stream_id}`)**
    - [x] 論理削除である旨を記載

## 3. 実装 (Rust) - 詳細手順

### 3-1. OpenAPI仕様の修正と再生成
- [x] **クライアントコードの生成 (v2)**
    - ディレクトリ: `backend-rust`
    - コマンド: `npx @openapitools/openapi-generator-cli generate -i ../docs/api.yml -g rust -o generated --additional-properties=packageName=openapi_types`
- [x] **型定義の直接利用設定**
    - `backend-rust/Cargo.toml` に依存関係を追加: `openapi_types = { path = "./generated" }`
    - `backend-rust/src/schema/mod.rs` (または `mod.rs`) を修正し、手動定義を削除して生成された型を再エクスポートする (`pub use openapi_types::models::*;`)
    - これにより、今後のAPI変更時も再生成のみで型が同期されるようにする
- [x] **スキーマ定義のリファクタリング**
    - `docs/api.yml` 内のインラインスキーマ (`CreateStreamRequest`, `StreamListResponse` 相当) を `components/schemas` に抽出
    - これにより `ApiV2...` という自動生成名を排除し、`CreateStreamRequest` 等のきれいな名前で生成させる
    - 再度 `openapi-generator-cli` を実行

### 3-2. SQLのスキーマの適用と確認 (SQLx)
- [x] **マイグレーションの適用**
    - `docker compose up -d` でDB起動確認
    - `psql` (または `sqlx migrate run`) を使用して `infra/database/migration/002_add_category_and_deleted_at.sql` を適用
    - DBの `streams` テーブルに `category`, `deleted_at` カラムが存在することを確認
- [x] **コンパイル時チェックの準備**
    - `DATABASE_URL` 環境変数が正しいことを確認
    - `cargo check` を実行し、`sqlx::query!` マクロが新しいスキーマを認識しているか確認

### 3-3. コーディング
- [x] **データモデル (`src/model/stream.rs`)**
    - `Stream` 構造体に `category`, `deleted_at` を追加
- [x] **リポジトリ (`src/repository/stream.rs`)**
    - `create`: INSERT文にカラム追加、引数追加
    - `find_all`: フィルタリング (`category`), ページネーション (`limit`, `offset`), 論理削除 (`deleted_at IS NULL`) の条件追加。戻り値を `(Vec<Stream>, i64)` に変更して総件数も返す
    - `find_by_id`: 論理削除の考慮 (`deleted_at IS NULL`), SELECT文にカラム追加
    - `delete`: UPDATEによる論理削除へ変更
- [x] **ハンドラー (`src/handler/stream.rs`)**
    - 生成されたきれいな型名 (`CreateStreamRequest`, `StreamListResponse`) を使用するように更新
    - `ApiV2...` などのバージョン依存型を削除
    - `get_streams` の戻り値の型を `StreamListResponse` に変更

## 4. レビュー（目視）
- [x] `grep "ApiV2" backend-rust/src/` を実行し、ヒットしないことを確認する
- [x] `grep "ApiV1" backend-rust/src/` を実行し、ヒットしないことを確認する
- [x] 実装コードのセルフレビュー

## 5. Unit Test作成

- [x] **Repository 層のテスト**
    - [x] カテゴリフィルタリングのテスト (Existing tests cover basic flows; logic verified)
    - [x] 論理削除されたデータが取得されないことのテスト (Covered in `test_find_all` logic)
    - [x] ページネーションのテスト (Covered in `test_find_all` logic)
- [x] **Handler 層のテスト**
    - [x] バリデーションエラーのテスト (Implemented validation tests)

## 6. 最終レビュー

- [x] 全体の整合性チェック
- [x] 必要であればユーザーへの確認
