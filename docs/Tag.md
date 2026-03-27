# Tag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The unique identifier of the tag | 
**label** | **String** | The tag label | 
**description** | Option<**String**> | Description for the tag | [optional]
**color** | Option<**String**> | The tag color in hex format | [optional]
**is_protected** | **bool** | Indication of whether the tag is a protected tag | [default to false]
**updated_at** | **f64** | The date and time the tag was last updated | 
**pending_approval_request** | Option<[**models::ApprovalRequest**](ApprovalRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


