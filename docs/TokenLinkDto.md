# TokenLinkDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The token link id | 
**status** | **Status** | The token status (enum: PENDING, COMPLETED) | 
**r#type** | Option<**Type**> | The type of token (enum: FUNGIBLE_TOKEN, NON_FUNGIBLE_TOKEN, TOKEN_UTILITY, TOKEN_EXTENSION) | [optional]
**ref_id** | Option<**String**> | The Fireblocks' reference id | [optional]
**display_name** | Option<**String**> | The token display name. If was not provided, would be taken from the contract template | [optional]
**token_metadata** | Option<[**models::TokenLinkDtoTokenMetadata**](TokenLinkDtoTokenMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


