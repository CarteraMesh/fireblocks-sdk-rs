# PixPaymentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rail** | **Rail** | The payment rail type for PIX transfers (enum: PIX) | 
**addressing_system** | **AddressingSystem** | The addressing system used for PIX transfers (enum: PIX) | 
**account_holder_given_name** | **String** | The given name (first name) of the account holder | 
**account_holder_surname** | **String** | The surname (last name) of the account holder | 
**country** | **String** | The country for the transfer (ISO 3166-1 alpha-2 code) | 
**pix_key** | **String** | The PIX key used for the transfer | 
**bank_name** | Option<**String**> | The name of the bank | [optional]
**bank_code** | Option<**String**> | The bank code (ISPB - Identificador do Sistema de Pagamentos Brasileiros) | [optional]
**key_type** | **KeyType** | The type of PIX key being used (enum: CPF, CNPJ, EMAIL, PHONE, RANDOM) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


