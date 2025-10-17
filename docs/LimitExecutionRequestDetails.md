# LimitExecutionRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**side** | **String** | Side of the order | [default to Buy]
**base_amount** | **String** | Amount to convert | 
**base_asset_id** | **String** | Source asset identifier | 
**base_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**quote_asset_id** | **String** | Target asset identifier | 
**quote_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**r#type** | **String** | Order type for limit orders | 
**time_in_force** | [**models::TimeInForce**](TimeInForce.md) |  | 
**limit_price** | **String** | Price for limit orders | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


