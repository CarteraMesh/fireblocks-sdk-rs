# ComplianceResultFullPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aml** | Option<[**Vec<models::ComplianceScreeningResultFullPayload>**](ComplianceScreeningResultFullPayload.md)> | The end result of the AML screening. | [optional]
**tr** | Option<[**Vec<models::ComplianceScreeningResultFullPayload>**](ComplianceScreeningResultFullPayload.md)> | The result of the Travel Rule screening. | [optional]
**aml_list** | Option<[**Vec<models::ComplianceScreeningResultFullPayload>**](ComplianceScreeningResultFullPayload.md)> | The list of all results of the AML screening. | [optional]
**status** | Option<**Status**> | Status of compliance result screening. (enum: Started, NetworkConnectionAddressResolve, ScreeningPrepare, AMLStarted, AMLCompleted, AMLFailed, AMLInBackground, TRPreconditionChecks, TRStarted, TRCompleted, TRFailed, Completed, IncomingStarted, IncomingScreeningPrepare, IncomingWaitForFirstConfirmation, AMLIncomingStarted, AMLIncomingCompleted, AMLIncomingFailed, AMLIncomingInBackground, TRIncomingStarted, TRIncomingCompleted, TRIncomingFailed, IncomingCompleted) | [optional]
**aml_registration** | Option<[**Vec<models::AmlRegistrationResultFullPayload>**](AmlRegistrationResultFullPayload.md)> | The results of the AML address registration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


