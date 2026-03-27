# NotificationAttempt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sent_time** | **i64** | The time when the attempt was sent in milliseconds. | 
**duration** | **i32** | The duration of the attempt in milliseconds. | 
**response_code** | Option<**i32**> | The response code of the attempt, when missing refer to failureReason. | [optional]
**failure_reason** | Option<**FailureReason**> | The request failure reason in case responseCode is missing. (enum: TIMED_OUT, NO_RESPONSE) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


