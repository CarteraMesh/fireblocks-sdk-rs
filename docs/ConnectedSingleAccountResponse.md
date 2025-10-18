# ConnectedSingleAccountResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the connected account. | 
**name** | **String** | Human-readable name of the connected account. | 
**provider_id** | **String** | The ID of the venue the account belongs to. | 
**status** | [**models::ConnectedAccountApprovalStatus**](ConnectedAccountApprovalStatus.md) |  | 
**manifest** | [**models::ConnectedAccountManifest**](ConnectedAccountManifest.md) |  | 
**parent_id** | Option<**String**> | The ID of the parent main account, if this is a sub account. | [optional]
**sub_accounts_ids** | Option<**Vec<String>**> | IDs of sub-accounts associated with this connected account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


