# DestinationTransferPeerPathResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::TransferPeerPathType**](TransferPeerPathType.md) |  | 
**sub_type** | Option<**String**> | In case the type is set to `EXCHANGE_ACCOUNT` or `FIAT_ACCOUNT`, the specific exchange vendor name or fiat vendor name.In case the type is set to `INTERNAL_WALLET` or `EXTERNAL_WALLET`, the subType is set to `Internal` or `External`. | [optional]
**id** | Option<**String**> | The ID of the peer. You can retrieve the ID of each venue object using the endpoints for [listing vault accounts](https://developers.fireblocks.com/reference/getpagedvaultaccounts), [listing exchange account](https://developers.fireblocks.com/reference/getexchangeaccounts), [listing fiat accounts](https://developers.fireblocks.com/reference/getfiataccounts), [listing internal wallets](https://developers.fireblocks.com/reference/getinternalwallets), [listing external wallets](https://developers.fireblocks.com/reference/getexternalwallets), [listing network connections](https://developers.fireblocks.com/reference/getnetworkconnections). For the other types, this parameter is not needed. | [optional]
**name** | Option<**String**> | The name of the peer | [optional]
**wallet_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**trading_account** | Option<**String**> | If this transaction is an exchange internal transfer, this field will be populated with the type of that trading account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


