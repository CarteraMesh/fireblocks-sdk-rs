# \ConnectedAccountsBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_connected_account**](ConnectedAccountsBetaApi.md#get_connected_account) | **GET** /connected_accounts/{accountId} | Get connected account
[**get_connected_account_balances**](ConnectedAccountsBetaApi.md#get_connected_account_balances) | **GET** /connected_accounts/{accountId}/balances | Get balances for an account
[**get_connected_account_rates**](ConnectedAccountsBetaApi.md#get_connected_account_rates) | **GET** /connected_accounts/{accountId}/rates | Get exchange rates for an account
[**get_connected_account_trading_pairs**](ConnectedAccountsBetaApi.md#get_connected_account_trading_pairs) | **GET** /connected_accounts/{accountId}/manifest/capabilities/trading/pairs | Get supported trading pairs for an account
[**get_connected_accounts**](ConnectedAccountsBetaApi.md#get_connected_accounts) | **GET** /connected_accounts | Get connected accounts



## get_connected_account

> models::ConnectedSingleAccountResponse get_connected_account(account_id)
Get connected account

Retrieve detailed information about a specific connected account by ID.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to fetch. | [required] |

### Return type

[**models::ConnectedSingleAccountResponse**](ConnectedSingleAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_account_balances

> models::ConnectedAccountBalancesResponse get_connected_account_balances(account_id, page_size, page_cursor)
Get balances for an account

Retrieve current asset balances for a specific connected account as a flat list (one row per `assetId`, `balanceType`).  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to fetch balances for. | [required] |
**page_size** | Option<**u16**> | Page size for pagination. |  |
**page_cursor** | Option<**String**> | Page cursor for pagination. |  |

### Return type

[**models::ConnectedAccountBalancesResponse**](ConnectedAccountBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_account_rates

> models::ConnectedAccountRateResponse get_connected_account_rates(account_id, base_asset_id, quote_asset_id)
Get exchange rates for an account

Retrieve current exchange rates for converting between specific assets in a connected account.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to fetch rates for. | [required] |
**base_asset_id** | **String** | The ID of the asset to fetch rates for. | [required] |
**quote_asset_id** | **String** | The ID of the asset to get the rates nominally. | [required] |

### Return type

[**models::ConnectedAccountRateResponse**](ConnectedAccountRateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_account_trading_pairs

> models::ConnectedAccountTradingPairsResponse get_connected_account_trading_pairs(account_id, page_size, page_cursor)
Get supported trading pairs for an account

Retrieve all asset trading pairs supported by a specific connected account, including the pair type (`quote`, `market`, `onOffRamp`).  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to fetch supported pairs for. | [required] |
**page_size** | Option<**u8**> | Page size for pagination. |  |[default to 100]
**page_cursor** | Option<**String**> | Page cursor for pagination. |  |

### Return type

[**models::ConnectedAccountTradingPairsResponse**](ConnectedAccountTradingPairsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connected_accounts

> models::ConnectedAccountsResponse get_connected_accounts(main_accounts, page_size, page_cursor)
Get connected accounts

Returns all connected accounts.  **Note:** This endpoint is currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main_accounts** | Option<**bool**> | Whether to include only main accounts in the response. |  |[default to false]
**page_size** | Option<**u8**> | Page size for pagination. |  |
**page_cursor** | Option<**String**> | Page cursor for pagination. |  |

### Return type

[**models::ConnectedAccountsResponse**](ConnectedAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

