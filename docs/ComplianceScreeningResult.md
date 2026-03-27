# ComplianceScreeningResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider** | Option<**Provider**> | Screening Provider (enum: CHAINALYSIS, ELLIPTIC, CHAINALYSIS_V2, ELLIPTIC_HOLISTIC, NONE) | [optional]
**payload** | Option<**serde_json::Value**> | The payload of the screening result. - The payload is a JSON object that contains the screening result. - The payload is different for each screening provider.  | [optional]
**bypass_reason** | Option<**BypassReason**> | Reason AML screening was bypassed (enum: MANUAL, UNSUPPORTED_ASSET, BYPASSED_FAILURE, UNSUPPORTED_ROUTE, PASSED_BY_POLICY, TIMED_OUT, BAD_CREDENTIALS, CONFIGURATION_ERROR, DROPPED_BY_BLOCKCHAIN, PROCESS_DISMISSED) | [optional]
**screening_status** | Option<**ScreeningStatus**> |  (enum: COMPLETED, PENDING, BYPASSED, FAILED, FROZEN) | [optional]
**timestamp** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


