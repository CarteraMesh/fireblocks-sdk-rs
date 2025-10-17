# \PolicyEditorV2BetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_active_v2_policy**](PolicyEditorV2BetaApi.md#get_active_v2_policy) | **GET** /policy/active_policy | Get the active policy and its validation by policy type
[**get_v2_draft**](PolicyEditorV2BetaApi.md#get_v2_draft) | **GET** /policy/draft | Get the active draft by policy type
[**publish_v2_draft**](PolicyEditorV2BetaApi.md#publish_v2_draft) | **POST** /policy/draft | Send publish request for a certain draft ID
[**update_v2_draft**](PolicyEditorV2BetaApi.md#update_v2_draft) | **PUT** /policy/draft | Update the draft with a new set of rules by policy types



## get_active_v2_policy

> models::PolicyAndValidationResponseV2 get_active_v2_policy(policy_type)
Get the active policy and its validation by policy type

Returns the active policy and its validation for a specific policy type.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_type** | [**PolicyTypeV2**](.md) | The policy type(s) to retrieve. Can be a single type or multiple types by repeating the parameter (e.g., ?policyType=TRANSFER&policyType=MINT). | [required] |

### Return type

[**models::PolicyAndValidationResponseV2**](PolicyAndValidationResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_draft

> models::DraftReviewAndValidationResponseV2 get_v2_draft(policy_type)
Get the active draft by policy type

Returns the active draft and its validation for a specific policy type.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_type** | [**PolicyTypeV2**](.md) | The policy type(s) to retrieve. Can be a single type or multiple types by repeating the parameter (e.g., ?policyType=TRANSFER&policyType=MINT). | [required] |

### Return type

[**models::DraftReviewAndValidationResponseV2**](DraftReviewAndValidationResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_v2_draft

> models::PublishResultV2 publish_v2_draft(publish_draft_request_v2, idempotency_key)
Send publish request for a certain draft ID

Send publish request of certain draft ID and returns the response.  **Note:** This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Policy Editor, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**publish_draft_request_v2** | [**PublishDraftRequestV2**](PublishDraftRequestV2.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::PublishResultV2**](PublishResultV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_v2_draft

> models::DraftReviewAndValidationResponseV2 update_v2_draft(update_draft_request_v2, idempotency_key)
Update the draft with a new set of rules by policy types

Update the draft and return its validation for specific policy types.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_draft_request_v2** | [**UpdateDraftRequestV2**](UpdateDraftRequestV2.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::DraftReviewAndValidationResponseV2**](DraftReviewAndValidationResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

