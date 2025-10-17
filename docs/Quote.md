# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**via** | [**models::AccessType**](AccessType.md) |  | 
**id** | **String** |  | 
**r#type** | **String** | Indicates this is an indicative quote | 
**quote_asset_id** | **String** |  | 
**base_asset_id** | **String** |  | 
**base_amount** | **String** |  | 
**quote_amount** | **String** |  | 
**price_impact** | Option<**f64**> |  | [optional]
**quote_min_amount** | Option<**String**> |  | [optional]
**execution_steps** | Option<[**Vec<models::ExecutionStepDetails>**](ExecutionStepDetails.md)> |  | [optional]
**general_fees** | Option<[**Vec<models::Fee>**](Fee.md)> |  | [optional]
**side** | **String** | Side of the order | 
**expires_at** | **String** | ISO 8601 timestamp of the expiration time of the quote. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


