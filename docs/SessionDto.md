# SessionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Id of the connection | 
**user_id** | **String** | Id of the user that created the connection | 
**session_metadata** | [**models::SessionMetadata**](SessionMetadata.md) | Metadata of the connection (provided by the dapp) | 
**vault_account_id** | **f64** | The vault to connect | 
**fee_level** | **FeeLevel** | The default fee level (enum: MEDIUM, HIGH) | 
**chain_ids** | **Vec<String>** | The chains approved for the connection | 
**connection_type** | **ConnectionType** | The connection's type (enum: WalletConnect) | 
**connection_method** | **ConnectionMethod** | The method through which the connection was established (enum: DESKTOP, MOBILE, API) | 
**creation_date** | **String** | Timestamp of the session's creation | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


