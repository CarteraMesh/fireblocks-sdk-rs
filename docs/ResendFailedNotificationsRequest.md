# ResendFailedNotificationsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_time** | Option<**f64**> | (optional) Start time for the resend window in milliseconds since epoch up to 24 hours before the current time - Default if missing means 24 hours before the current time in milliseconds since epoch - Maximum value is current time in milliseconds since epoch - Minimum value is 24 hours before the current time in milliseconds since epoch  | [optional]
**events** | Option<**Vec<String>**> | (optional) Event types to resend, default is all event types - Default if missing means all events will be included - Empty array means all events will be included  | [optional]
**items** | Option<[**models::WebhookEvent**](WebhookEvent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


