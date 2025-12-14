oapi-codegenによって自動生成されるコード（types, server, spec）

## コード生成ワークフロー

1. `docs/api.yml` を編集
2. `backend-golang` ディレクトリで `go generate ./...` を実行
3. `openapi.gen.go` が自動生成/更新される
