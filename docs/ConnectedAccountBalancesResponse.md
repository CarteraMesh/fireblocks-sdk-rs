# ConnectedAccountBalancesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**Vec<models::ConnectedAccountBalances>**](ConnectedAccountBalances.md) | Flat balance row for a single asset within an account and wallet type. One row per (assetId, balanceType). | 
**total** | Option<**i32**> | Total number of balance rows by query. | [optional]
**next** | Option<**String**> | A cursor for the next page of results, if available. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


