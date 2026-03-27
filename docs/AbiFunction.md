# AbiFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the contract function as it appears in the ABI | [optional]
**state_mutability** | Option<**StateMutability**> | The state mutability of the contract function as it appears in the ABI (enum: pure, view, nonpayable, payable) | [optional]
**r#type** | **Type** | The type if the function (enum: function, constructor) | 
**inputs** | [**Vec<models::Parameter>**](Parameter.md) | The parameters that this function/constructor posses | 
**outputs** | Option<[**Vec<models::Parameter>**](Parameter.md)> | The parameters that this 'read' function returns | [optional]
**description** | Option<**String**> | The documentation of this function (if has any) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


