# AccountBasedAccessProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the provider | 
**name** | **String** | Display name of the provider | 
**logo** | Option<**String**> | URL to the logo image of the provider | [optional]
**account_based** | **bool** | Indicates whether the provider access model is through accounts or directly | 
**manifest** | [**models::Manifest**](Manifest.md) |  | 
**connected** | **bool** | Whether the provider is currently connected | 
**accounts** | Option<[**Vec<models::AccountBase>**](AccountBase.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


