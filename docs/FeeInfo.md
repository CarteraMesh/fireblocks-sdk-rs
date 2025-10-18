# FeeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_fee** | Option<**String**> | The fee paid to the network. | [optional]
**service_fee** | Option<**String**> | The total fee deducted by the exchange from the actual requested amount (serviceFee = amount - netAmount). | [optional]
**gas_price** | Option<**String**> | The amount of gas required by/paid to the network to process the transaction. | [optional]
**l1network_fee** | Option<**String**> | Layer 1 network fee for Layer 2 blockchain transactions | [optional]
**l2network_fee** | Option<**String**> | Layer 2 network fee (gas price component for Layer 2 transactions) | [optional]
**paid_by_relay** | Option<**bool**> | Indicates whether the relay paid the fee. | [optional]
**relay_type** | Option<**String**> | Indicates whether the relay is the same tenant (`LOCAL`) or another tenant (`THIRD_PARTY`). | [optional]
**relay_id** | Option<**String**> | The Vault account ID of the relay. | [optional]
**relay_name** | Option<**String**> | The name of the tenant hosting the third-party relay. | [optional]
**fee_usd** | Option<**String**> | The USD equivalent value of the fee | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


