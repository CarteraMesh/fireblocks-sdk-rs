# ConnectedAccountBalances

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | **String** | Asset identifier (e.g., BTC, ETH, USDC). | 
**available_amount** | **String** | Amount available for use. | 
**total_amount** | **String** | Total amount including locked/held balances. | 
**locked_amount** | Option<**String**> | Amount currently locked/held. | [optional]
**credit_amount** | Option<**String**> | Credit line amount, if applicable (0 when not used). | [optional]
**balance_type** | **String** | Wallet type/category (e.g., SPOT, MARGIN, FUNDING). | 
**balance_name** | Option<**String**> | Display name for the balance type (at the provider) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


