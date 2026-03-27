# CreateQuote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | [**Vec<models::CreateQuoteScopeInner>**](CreateQuoteScopeInner.md) |  | 
**base_asset_id** | **String** |  | 
**base_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**quote_asset_id** | **String** |  | 
**quote_asset_rail** | Option<[**models::TransferRail**](TransferRail.md)> |  | [optional]
**base_amount** | **String** | The amount to convert from | 
**slippage_bps** | Option<**f64**> | Slippage tolerance in basis points (bps) for defi quotes - 1 is 0.01% and 10000 is 100% | [optional][default to 50]
**settlement** | Option<[**models::DvpSettlement**](DVPSettlement.md)> |  | [optional]
**side** | **Side** | Side of the order (enum: BUY, SELL) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


