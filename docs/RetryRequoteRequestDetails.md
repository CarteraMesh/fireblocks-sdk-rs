# RetryRequoteRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Indicates that the order should be re-quoted if the original quote is expired, trying to match the original quote. | 
**count** | **f64** | If quote is expired, how many times to re-generate new quotes to try having the order executed as in the original quote. | 
**slippage_bps** | Option<**f64**> | Slippage tolerance in basis points (bps) for quote orders - 1 is 0.01% and 10000 is 100% | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


