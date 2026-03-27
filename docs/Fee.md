# Fee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee_type** | **FeeType** | The type of fee, such as ORDER, NETWORK, or SPREAD. - `ORDER`: Fee for executing the order. - `NETWORK`: Fee for network transactions. - `SPREAD`: Fee for the difference between buy and sell prices.  (enum: ORDER, NETWORK, SPREAD) | 
**asset_id** | **String** | The asset identifier for the fee. | 
**amount_type** | **AmountType** |  (enum: BPS) | 
**amount** | **f64** | Fee in basis points (1 = 0.01%, 10000 = 100%) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


