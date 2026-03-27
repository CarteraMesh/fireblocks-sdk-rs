# \WebhooksV2Api

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksV2Api.md#create_webhook) | **POST** /webhooks | Create a new webhook
[**delete_webhook**](WebhooksV2Api.md#delete_webhook) | **DELETE** /webhooks/{webhookId} | Delete a webhook
[**get_notification**](WebhooksV2Api.md#get_notification) | **GET** /webhooks/{webhookId}/notifications/{notificationId} | Get notification by ID
[**get_notification_attempts**](WebhooksV2Api.md#get_notification_attempts) | **GET** /webhooks/{webhookId}/notifications/{notificationId}/attempts | Get notification attempts
[**get_notifications**](WebhooksV2Api.md#get_notifications) | **GET** /webhooks/{webhookId}/notifications | Get all notifications by webhook ID
[**get_resend_job_status**](WebhooksV2Api.md#get_resend_job_status) | **GET** /webhooks/{webhookId}/notifications/resend_failed/jobs/{jobId} | Get resend job status
[**get_webhook**](WebhooksV2Api.md#get_webhook) | **GET** /webhooks/{webhookId} | Get webhook by id
[**get_webhooks**](WebhooksV2Api.md#get_webhooks) | **GET** /webhooks | Get all webhooks
[**resend_failed_notifications**](WebhooksV2Api.md#resend_failed_notifications) | **POST** /webhooks/{webhookId}/notifications/resend_failed | Resend failed notifications
[**resend_notification_by_id**](WebhooksV2Api.md#resend_notification_by_id) | **POST** /webhooks/{webhookId}/notifications/{notificationId}/resend | Resend notification by ID
[**resend_notifications_by_resource_id**](WebhooksV2Api.md#resend_notifications_by_resource_id) | **POST** /webhooks/{webhookId}/notifications/resend_by_resource | Resend notifications by resource ID
[**update_webhook**](WebhooksV2Api.md#update_webhook) | **PATCH** /webhooks/{webhookId} | Update webhook



## create_webhook

> models::Webhook create_webhook(create_webhook_request, idempotency_key)
Create a new webhook

Creates a new webhook, which will be triggered on the specified events  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> models::Webhook delete_webhook(webhook_id)
Delete a webhook

Delete a webhook by its ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> models::NotificationWithData get_notification(webhook_id, notification_id, include_data)
Get notification by ID

Get a notification by its notification ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook to fetch | [required] |
**notification_id** | **String** | The ID of the notification to fetch | [required] |
**include_data** | Option<**bool**> | Include the data of the notification |  |

### Return type

[**models::NotificationWithData**](NotificationWithData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_attempts

> models::NotificationAttemptsPaginatedResponse get_notification_attempts(webhook_id, notification_id, page_cursor, page_size)
Get notification attempts

Get notification attempts by notification ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook to fetch | [required] |
**notification_id** | **String** | The ID of the notification to fetch | [required] |
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 10]

### Return type

[**models::NotificationAttemptsPaginatedResponse**](NotificationAttemptsPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> models::NotificationPaginatedResponse get_notifications(webhook_id, order, sort_by, page_cursor, page_size, start_time, end_time, statuses, events, resource_id)
Get all notifications by webhook ID

Get all notifications by webhook ID (paginated).  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** |  | [required] |
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**sort_by** | Option<**String**> | Sort by field (id, createdAt, updatedAt, status, eventType, resourceId) |  |[default to id]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items on the page |  |[default to 100]
**start_time** | Option<**String**> | Start time in milliseconds since epoch to filter by notifications created after this time (default 31 days ago) |  |
**end_time** | Option<**String**> | End time in milliseconds since epoch to filter by notifications created before this time (default current time) |  |
**statuses** | Option<[**Vec<models::NotificationStatus>**](Models__NotificationStatus.md)> | List of notification statuses to filter by |  |
**events** | Option<[**Vec<models::WebhookEvent>**](Models__WebhookEvent.md)> | List of webhook event types to filter by |  |
**resource_id** | Option<**String**> | Resource ID to filter by |  |

### Return type

[**models::NotificationPaginatedResponse**](NotificationPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resend_job_status

> models::ResendFailedNotificationsJobStatusResponse get_resend_job_status(webhook_id, job_id)
Get resend job status

Get the status of a resend job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**job_id** | **String** | The ID of the resend job | [required] |

### Return type

[**models::ResendFailedNotificationsJobStatusResponse**](ResendFailedNotificationsJobStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> models::Webhook get_webhook(webhook_id)
Get webhook by id

Retrieve a webhook by its ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::WebhookPaginatedResponse get_webhooks(order, page_cursor, page_size)
Get all webhooks

Get all webhooks (paginated).  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items on the page |  |[default to 10]

### Return type

[**models::WebhookPaginatedResponse**](WebhookPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_failed_notifications

> models::ResendFailedNotificationsResponse resend_failed_notifications(webhook_id, resend_failed_notifications_request, idempotency_key)
Resend failed notifications

Resend all failed notifications for a webhook in the last 24 hours.  **Endpoint Permission:** Owner, Admin, Non-Signing Admin, Signer, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**resend_failed_notifications_request** | [**ResendFailedNotificationsRequest**](ResendFailedNotificationsRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ResendFailedNotificationsResponse**](ResendFailedNotificationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_notification_by_id

> resend_notification_by_id(webhook_id, notification_id, idempotency_key)
Resend notification by ID

Resend a notification by its ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**notification_id** | **String** | The ID of the notification | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_notifications_by_resource_id

> resend_notifications_by_resource_id(webhook_id, resend_notifications_by_resource_id_request, idempotency_key)
Resend notifications by resource ID

Resend notifications by their resource ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**resend_notifications_by_resource_id_request** | [**ResendNotificationsByResourceIdRequest**](ResendNotificationsByResourceIdRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::Webhook update_webhook(webhook_id, update_webhook_request)
Update webhook

Update a webhook by its ID.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |
**update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md) |  | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

