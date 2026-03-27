# AssetTypeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique asset identifier | 
**name** | **String** | The name of the asset | 
**r#type** | **Type** | Asset type (enum: BASE_ASSET, ERC20, BEP20, COMPOUND, TRON_TRC20, NEAR_ASSET, SOL_ASSET, FIAT, ALGO_ASSET, XLM_ASSET, XDB_ASSET) | 
**contract_address** | Option<**String**> | Contract address of EVM based tokens | [optional]
**native_asset** | Option<**String**> | The native asset ID | [optional]
**decimals** | Option<**f64**> | Decimals of the asset | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


