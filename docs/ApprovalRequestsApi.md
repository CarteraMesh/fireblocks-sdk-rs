# \ApprovalRequestsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_approval_request**](ApprovalRequestsApi.md#cancel_approval_request) | **POST** /tags/approval_requests/{id}/cancel | Cancel an approval request by ID
[**get_approval_request**](ApprovalRequestsApi.md#get_approval_request) | **GET** /tags/approval_requests/{id} | Get an approval request by id



## cancel_approval_request

> cancel_approval_request(id, idempotency_key)
Cancel an approval request by ID

Cancel an approval request by id. - Can only cancel requests in the `PENDING` status. - Returns `202 Accepted` when the cancellation is processed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_request

> models::ApprovalRequest get_approval_request(id)
Get an approval request by id

Get an approval request by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ApprovalRequest**](ApprovalRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

