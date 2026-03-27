# QuoteExecutionResponseDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**side** | **Side** | Side of the order (enum: BUY, SELL) | [default to Buy]
**base_amount** | **String** | Amount to convert | 
**base_asset_id** | **String** | Source asset identifier | 
**base_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**quote_asset_id** | **String** | Target asset identifier | 
**quote_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**r#type** | **Type** | Order type for quote orders (enum: QUOTE) | 
**quote_id** | **String** | Quote ID for quote orders | 
**quote_amount** | **String** | Quote amount for quote orders | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


