# \TradingBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_order**](TradingBetaApi.md#create_order) | **POST** /trading/orders | Create an order
[**create_quote**](TradingBetaApi.md#create_quote) | **POST** /trading/quotes | Create a quote
[**get_order**](TradingBetaApi.md#get_order) | **GET** /trading/orders/{orderId} | Get order details
[**get_orders**](TradingBetaApi.md#get_orders) | **GET** /trading/orders | Get orders
[**get_trading_providers**](TradingBetaApi.md#get_trading_providers) | **GET** /trading/providers | Get providers



## create_order

> models::OrderDetails create_order(create_order_request, idempotency_key)
Create an order

Create an order to buy or sell an asset. If no source is given, an external source will be use.  **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Trading, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::OrderDetails**](OrderDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_quote

> models::QuotesResponse create_quote(create_quote, idempotency_key)
Create a quote

Generate a time-limited quote for asset conversion, providing exchange rate and amount calculations.  **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Trading, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Editor. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_quote** | [**CreateQuote**](CreateQuote.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::QuotesResponse**](QuotesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::OrderDetails get_order(order_id)
Get order details

Retrieve detailed information about a specific order by its ID.  **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Trading, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | The ID of the order to fetch. | [required] |

### Return type

[**models::OrderDetails**](OrderDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> models::GetOrdersResponse get_orders(page_size, page_cursor, order, account_id, provider_id, statuses, start_time, end_time, asset_conversion_type)
Get orders

Retrieve a paginated list of orders with optional filtering by account, provider, status, and time range.  **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Trading, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  **Endpoint Permissions:** Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | **u8** | pageSize for pagination. | [required] |
**page_cursor** | Option<**String**> |  |  |
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**account_id** | Option<[**Vec<String>**](String.md)> | Filter by accountId. |  |
**provider_id** | Option<[**Vec<String>**](String.md)> | Filter by providerId. |  |
**statuses** | Option<[**Vec<models::OrderStatus>**](Models__OrderStatus.md)> | Filter by order status. |  |
**start_time** | Option<**u32**> |  |  |
**end_time** | Option<**u32**> |  |  |
**asset_conversion_type** | Option<**String**> |  |  |

### Return type

[**models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trading_providers

> models::ProvidersListResponse get_trading_providers(page_size, page_cursor)
Get providers

Retrieve a list of all available external providers supporting trading activities through the platform.  **Note:** These endpoints are currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Trading, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  **Endpoint Permission:** Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**u8**> | Page size for pagination. |  |[default to 20]
**page_cursor** | Option<**String**> | Page cursor for pagination. |  |

### Return type

[**models::ProvidersListResponse**](ProvidersListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

