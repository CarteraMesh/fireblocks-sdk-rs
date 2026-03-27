# SepaPaymentInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rail** | **Rail** | The payment rail type for SEPA transfers (enum: SEPA) | 
**addressing_system** | **AddressingSystem** | The addressing system used for SEPA transfers (enum: IBAN) | 
**account_holder_given_name** | **String** | The given name (first name) of the account holder | 
**account_holder_surname** | **String** | The surname (last name) of the account holder | 
**account_holder_country** | Option<**String**> | The country where the account holder resides (ISO 3166-1 alpha-2 code) | [optional]
**account_holder_address** | Option<**String**> | The address of the account holder | [optional]
**iban** | **String** | The International Bank Account Number (IBAN) | 
**country** | **String** | The country for the transfer (ISO 3166-1 alpha-2 code) | 
**bic** | Option<**String**> | The Bank Identifier Code (BIC/SWIFT code) | [optional]
**bank_name** | Option<**String**> | The name of the bank | [optional]
**bank_branch** | Option<**String**> | The bank branch information | [optional]
**bank_address** | Option<**String**> | The address of the bank | [optional]
**purpose_code** | Option<**String**> | The purpose code for the transfer | [optional]
**tax_id** | Option<**String**> | The tax identification number | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


