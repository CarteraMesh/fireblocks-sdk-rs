# MomoPaymentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rail** | **Rail** | The payment rail type for mobile money transfers (enum: MOMO) | 
**addressing_system** | **AddressingSystem** | The addressing system used for mobile money transfers (enum: MOMO) | 
**account_holder_given_name** | **String** | The given name (first name) of the account holder | 
**account_holder_surname** | **String** | The surname (last name) of the account holder | 
**country** | **String** | The country for the transfer (ISO 3166-1 alpha-2 code) | 
**mobile_phone_number** | **String** | The mobile phone number associated with the mobile money account | 
**provider** | **Provider** | The mobile money service provider (enum: M_PESA, AIRTEL, MTN, TIGO) | 
**beneficiary_document_id** | Option<**String**> | The document ID of the beneficiary | [optional]
**beneficiary_relationship** | Option<**String**> | The relationship between sender and beneficiary | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


