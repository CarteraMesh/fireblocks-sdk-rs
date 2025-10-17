# ConnectedAccountManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_types** | [**Vec<models::ConnectedAccountAssetType>**](ConnectedAccountAssetType.md) | Asset types supported by the connected account. | 
**capabilities** | [**Vec<models::ConnectedAccountCapability>**](ConnectedAccountCapability.md) | Features supported for the connected account. Logic:   - If account capabilities include ramp -> TRADING   - If account capabilities include transfers -> DEPOSITS   - If account capabilities include transfersBlockchain / transfersFiat / transfersPeerAccounts / transfersInternal -> WITHDRAWALS  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


