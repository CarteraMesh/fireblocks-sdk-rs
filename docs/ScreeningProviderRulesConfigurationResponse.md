# ScreeningProviderRulesConfigurationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | Option<**Direction**> |  (enum: INBOUND, OUTBOUND) | [optional]
**status** | Option<**Status**> |  (enum: COMPLETED, PENDING, REJECTED, FAILED, CANCELED, BLOCKING_TIME_EXPIRED) | [optional]
**amount_usd** | Option<**f64**> |  | [optional]
**amount** | Option<**f64**> |  | [optional]
**asset** | Option<**String**> |  | [optional]
**action** | **Action** |  (enum: ACCEPT, REJECT, ALERT, WAIT, FREEZE, CANCEL) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


