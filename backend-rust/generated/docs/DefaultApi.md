# \DefaultApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v2_streams_get**](DefaultApi.md#api_v2_streams_get) | **GET** /api/v2/streams | YouTube配信情報の一覧取得
[**api_v2_streams_post**](DefaultApi.md#api_v2_streams_post) | **POST** /api/v2/streams | YouTube配信情報の登録
[**api_v2_streams_stream_id_delete**](DefaultApi.md#api_v2_streams_stream_id_delete) | **DELETE** /api/v2/streams/{streamId} | YouTube配信情報の削除（論理削除）
[**api_v2_streams_stream_id_get**](DefaultApi.md#api_v2_streams_stream_id_get) | **GET** /api/v2/streams/{streamId} | YouTube配信情報の詳細取得



## api_v2_streams_get

> models::StreamListResponse api_v2_streams_get(category, limit, offset)
YouTube配信情報の一覧取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> | カテゴリによる絞り込み（部分一致） |  |
**limit** | Option<**i32**> | 1回のリクエストで取得する件数 |  |[default to 10]
**offset** | Option<**i32**> | 取得開始位置 |  |[default to 0]

### Return type

[**models::StreamListResponse**](StreamListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_streams_post

> models::Stream api_v2_streams_post(create_stream_request)
YouTube配信情報の登録

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_stream_request** | [**CreateStreamRequest**](CreateStreamRequest.md) |  | [required] |

### Return type

[**models::Stream**](Stream.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_streams_stream_id_delete

> api_v2_streams_stream_id_delete(stream_id)
YouTube配信情報の削除（論理削除）

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | 配信ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_streams_stream_id_get

> models::Stream api_v2_streams_stream_id_get(stream_id)
YouTube配信情報の詳細取得

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | 配信ID | [required] |

### Return type

[**models::Stream**](Stream.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

