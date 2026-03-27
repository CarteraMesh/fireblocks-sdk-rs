# VaultAccountTagAttachmentRejectedOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The ID of the vault account | 
**tag_id** | **uuid::Uuid** | Tag ID | 
**action** | [**models::TagAttachmentOperationAction**](TagAttachmentOperationAction.md) |  | 
**reason** | **Reason** | Reason for rejection (enum: CAPACITY_EXCEEDED, ATTACHMENT_ALREADY_EXISTS, ATTACHMENT_DOES_NOT_EXIST, PENDING_REQUEST_EXISTS) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


