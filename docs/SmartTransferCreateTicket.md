# SmartTransferCreateTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_by_network_id** | **String** |  | 
**r#type** | **Type** | **Note:** The `DVP` value is in Early Access and should only be used if Fireblocks has enabled it in your workspace. Contact your Customer Success Manager for more information. (enum: ASYNC, DVP) | 
**expires_in** | Option<**f64**> | Number of hours after which an OPEN ticket will expire if no term is funded. | [optional]
**terms** | Option<[**Vec<models::SmartTransferCreateTicketTerm>**](SmartTransferCreateTicketTerm.md)> |  | [optional]
**external_ref_id** | Option<**String**> |  | [optional]
**note** | Option<**String**> |  | [optional]
**submit** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


