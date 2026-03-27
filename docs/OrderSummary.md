# OrderSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**via** | [**models::AccessType**](AccessType.md) |  | 
**side** | **Side** | Side of the order (enum: BUY, SELL) | 
**base_amount** | **String** |  | 
**quote_amount** | Option<**String**> |  | [optional]
**base_asset_id** | **String** |  | 
**quote_asset_id** | **String** |  | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**destination** | [**models::AccountReference**](AccountReference.md) |  | 
**source** | Option<[**models::SettlementSourceAccount**](SettlementSourceAccount.md)> |  | [optional]
**created_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


