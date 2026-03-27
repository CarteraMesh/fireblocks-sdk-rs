# GetFilterParameter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique connection ID | [optional]
**user_id** | Option<**String**> | User ID that established the connection | [optional]
**vault_account_id** | Option<**f64**> | Unique vault account identifier that the connection was established with | [optional]
**connection_method** | Option<**ConnectionMethod**> | The method that the connection was established with.  - MOBILE: for connections that were established by scanning a WalletConnect QR code  - DESKTOP: for connection that were established by WalletConnect desktop connections  (enum: MOBILE, DESKTOP) | [optional]
**fee_level** | Option<**FeeLevel**> | The fee level for the transactions over the connection (enum: MEDIUM, HIGH) | [optional]
**app_url** | Option<**String**> | The dApp URL | [optional]
**app_name** | Option<**String**> | dApps name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


