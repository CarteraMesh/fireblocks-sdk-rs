# DestinationConfigV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::AccountTypeV2**](AccountTypeV2.md) |  | 
**sub_type** | Option<[**Vec<models::AccountIdentifierV2>**](AccountIdentifierV2.md)> |  | [optional]
**ids** | Option<[**Vec<models::AccountIdentifierV2>**](AccountIdentifierV2.md)> |  | [optional]
**operator** | [**models::PolicyOperatorV2**](PolicyOperatorV2.md) |  | 
**match_from** | Option<**MatchFrom**> | Whether to match from account or source (enum: ACCOUNT, SOURCE) | [optional]
**address_type** | **AddressType** | Type of destination addresses allowed (enum: ALL, *, WHITELISTED, ONE_TIME, OEC_PARTNER) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


