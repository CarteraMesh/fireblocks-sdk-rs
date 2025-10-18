# OrderDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**via** | [**models::AccessType**](AccessType.md) |  | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**created_at** | **String** |  | 
**updated_at** | Option<**String**> |  | [optional]
**receipt** | Option<[**models::TransferReceipt**](TransferReceipt.md)> |  | [optional]
**general_fees** | Option<[**Vec<models::Fee>**](Fee.md)> |  | [optional]
**execution_steps** | [**Vec<models::ExecutionStep>**](ExecutionStep.md) |  | 
**execution_response_details** | [**models::ExecutionResponseDetails**](ExecutionResponseDetails.md) |  | 
**settlement** | [**models::Settlement**](Settlement.md) |  | 
**participants_identification** | Option<[**models::ParticipantsIdentification**](ParticipantsIdentification.md)> |  | [optional]
**payment_instructions** | Option<[**Vec<models::PaymentInstructions>**](PaymentInstructions.md)> | Payment instructions for the order, the client can use one of these to pay the order. | [optional]
**created_by** | **String** | The ID of the user who created the order | 
**customer_internal_reference_id** | Option<**String**> | Internal reference ID for the customer | [optional]
**note** | Option<**String**> | Optional note for the Order | [optional]
**expires_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


