# ExtraParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_controls** | Option<[**models::NodeControls**](NodeControls.md)> |  | [optional]
**raw_message_data** | Option<[**models::ExtraParametersRawMessageData**](ExtraParametersRawMessageData.md)> |  | [optional]
**contract_call_data** | Option<**String**> | Hex encoded contract call data as a string. | [optional]
**program_call_data** | Option<**String**> | BASE64 encoded Solana unsigned serialized transaction object. | [optional]
**inputs_selection** | Option<[**models::ExtraParametersInputsSelection**](ExtraParametersInputsSelection.md)> |  | [optional]
**allow_base_asset_address** | Option<**bool**> | Transfers to unmanaged wallets only. When true, if the destination doesn't have a whitelisted address for the requested asset, we'll use the destination's whitelisted address of the base asset (e.g., use the ETH address for an ERC-20). If the requested asset's whitelisted address exists, it is used.  **Note:** This is a premium feature that should be enabled in your workspace. Please contact your Customer Success Manager or Fireblocks Support for more information.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


