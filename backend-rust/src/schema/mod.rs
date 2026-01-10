pub use openapi_types::models::{
    Stream as StreamResponse, StreamSummary as StreamSummaryResponse, ValidationError, ServerError,
    CreateStreamRequest,
};

// StreamListResponseはOpenAPI Generatorで単純なモデルとして生成されない場合があるため（inline定義の場合）、
// 必要に応じてここで定義するか、生成された型を確認してaliasesを設定する。
// 今回の生成結果では `_api_v1_streams_get_200_response.rs` のような名前になっている可能性がある。
// 一旦 generated/src/models 下の構造を確認したほうが良いが、
// 前回の手順で api.rs を使わない方針にしたため、ここでの re-export を調整する。
// 確認のため、まずは単純な re-export にしておく。
pub use openapi_types::models::_api_v1_streams_get_200_response::ApiV1StreamsGet200Response as StreamListResponse;
