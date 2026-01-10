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

- [ ] **配信登録 API (`POST /streams`)**
    - [ ] リクエストボディに `category` (string) を追加
- [ ] **配信一覧取得 API (`GET /streams`)**
    - [ ] クエリパラメータの追加:
        - [ ] `category`: カテゴリによる絞り込み
        - [ ] `limit`: 1回のリクエストで取得する件数
        - [ ] `offset`: 取得開始位置
    - [ ] レスポンスボディの変更:
        - [ ] ページネーション情報 (`limit`, `offset`, `total`) を追加
        - [ ] `items` 配列の中に配信データを入れる構造に変更
        - [ ] 各配信データに `category` を追加
- [ ] **配信詳細取得 API (`GET /streams/{stream_id}`)**
    - [ ] レスポンスボディに `category` を追加
- [ ] **配信削除 API (`DELETE /streams/{stream_id}`)**
    - [ ] 論理削除である旨を記載

## 3. 実装 (Rust)

- [ ] **OpenAPI スキーマ定義の同期**
    - [ ] `openapi-generator-cli` で Rust コードを再生成
- [ ] **データモデル (`backend-rust/src/model/stream.rs`)**
    - [ ] `Stream` 構造体に `category`, `deleted_at` 追加
- [ ] **データベース操作 (`backend-rust/src/repository/stream.rs`)**
    - [ ] SQL クエリの修正 (INSERT, SELECT, UPDATE for delete)
    - [ ] ページネーションとフィルタリングの実装
- [ ] **ハンドラー (`backend-rust/src/handler/stream.rs`)**
    - [ ] リクエスト/レスポンスの変換処理修正
    - [ ] ページネーションパラメータの処理

## 4. レビュー（目視）

- [ ] `docs/table.md` と `backend-rust/migrations` の整合性確認
- [ ] `docs/api.yml` の変更内容確認
- [ ] 実装コードのセルフレビュー

## 5. Unit Test作成

- [ ] **Repository 層のテスト**
    - [ ] カテゴリフィルタリングのテスト
    - [ ] 論理削除されたデータが取得されないことのテスト
    - [ ] ページネーションのテスト
- [ ] **Handler 層のテスト**
    - [ ] バリデーションエラーのテスト

## 6. 最終レビュー

- [ ] 全体の整合性チェック
- [ ] 必要であればユーザーへの確認
