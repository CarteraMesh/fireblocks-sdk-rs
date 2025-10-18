# \TagsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **POST** /tags | Create a tag
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /tags/{tagId} | Delete a tag
[**get_tag**](TagsApi.md#get_tag) | **GET** /tags/{tagId} | Get a tag
[**get_tags**](TagsApi.md#get_tags) | **GET** /tags | Get list of tags
[**update_tag**](TagsApi.md#update_tag) | **PATCH** /tags/{tagId} | Update a tag



## create_tag

> models::Tag create_tag(create_tag_request, idempotency_key)
Create a tag

Create a new tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tag_request** | [**CreateTagRequest**](CreateTagRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::Tag**](Tag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> delete_tag(tag_id)
Delete a tag

Delete the specified tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | The ID of the tag to retrieve | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> models::Tag get_tag(tag_id)
Get a tag

Retrieve an existing tag by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | The ID of the tag to retrieve | [required] |

### Return type

[**models::Tag**](Tag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> models::TagsPagedResponse get_tags(page_cursor, page_size, label, tag_ids)
Get list of tags

Retrieve a paged list of all tags according to filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Page cursor to get the next page. |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 100]
**label** | Option<**String**> | Label prefix to filter by. |  |
**tag_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | List of tag IDs to filter by. |  |

### Return type

[**models::TagsPagedResponse**](TagsPagedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> models::Tag update_tag(tag_id, update_tag_request, idempotency_key)
Update a tag

Update an existing specified tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **uuid::Uuid** | The ID of the tag to update | [required] |
**update_tag_request** | [**UpdateTagRequest**](UpdateTagRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::Tag**](Tag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

