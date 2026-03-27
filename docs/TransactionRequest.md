# TransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | Option<[**models::TransactionOperation**](TransactionOperation.md)> |  | [optional]
**note** | Option<**String**> | Custom note, not sent to the blockchain, to describe the transaction at your Fireblocks workspace. | [optional]
**external_tx_id** | Option<**String**> | Allows you to add a unique ID to help prevent duplicate transactions. No specific format is required for this parameter. After you submit a transaction with an external ID, Fireblocks will automatically reject all future transactions with the same ID. - Using an external ID primarily helps in situations where, even though a submitted transaction responds with an error due to an internet outage, the transaction was still sent to and processed on the blockchain. - Use [Get a specific transaction by external transaction ID](https://developers.fireblocks.com/reference/gettransactionbyexternalid) to validate whether these transactions have been processed.  | [optional]
**asset_id** | Option<**String**> | The ID of the asset to transfer; used for `TRANSFER`, `MINT`, or `BURN` operations. See [the list of supported assets and their IDs](https://developers.fireblocks.com/reference/listassets). | [optional]
**source** | Option<[**models::SourceTransferPeerPath**](SourceTransferPeerPath.md)> |  | [optional]
**destination** | Option<[**models::DestinationTransferPeerPath**](DestinationTransferPeerPath.md)> |  | [optional]
**destinations** | Option<[**Vec<models::TransactionRequestDestination>**](TransactionRequestDestination.md)> | For UTXO based blockchains, you can send a single transaction to multiple destinations. | [optional]
**amount** | Option<[**models::TransactionRequestAmount**](TransactionRequestAmount.md)> |  | [optional]
**treat_as_gross_amount** | Option<**bool**> | \"When set to `true`, the fee will be deducted from the requested amount.\"  **Note**: This parameter can only be considered if a transaction’s asset is a base asset, such as ETH or MATIC. If the asset can’t be used for transaction fees, like USDC, this parameter is ignored and the fee is deducted from the relevant base asset wallet in the source account. | [optional]
**force_sweep** | Option<**bool**> | For Polkadot, TON, Kusama and Westend transactions only. When set to true, Fireblocks will empty the asset wallet.     **Note:** If set to true when the source account is exactly 1 DOT, the transaction will fail. Any amount more or less than 1 DOT succeeds. This is a Polkadot blockchain limitation. | [optional]
**fee_level** | Option<**FeeLevel**> | For UTXO or EVM-based blockchains only. Defines the blockchain fee level which will be payed for the transaction. Alternatively, specific fee estimation parameters exist below. (enum: LOW, MEDIUM, HIGH) | [optional]
**fee** | Option<[**models::TransactionRequestFee**](TransactionRequestFee.md)> |  | [optional]
**priority_fee** | Option<[**models::TransactionRequestPriorityFee**](TransactionRequestPriorityFee.md)> |  | [optional]
**fail_on_low_fee** | Option<**bool**> | When set to `true`, in case the current `MEDIUM` fee level is higher than the one specified in the transaction, the transaction will fail to avoid getting stuck with no confirmations. | [optional]
**max_fee** | Option<**String**> | The maximum fee (gas price or fee per byte) that should be paid for the transaction. In case the current value of the requested `feeLevel` is higher than this requested maximum fee. Represented by a numeric string for more precision. | [optional]
**gas_limit** | Option<[**models::TransactionRequestGasLimit**](TransactionRequestGasLimit.md)> |  | [optional]
**gas_price** | Option<[**models::TransactionRequestGasPrice**](TransactionRequestGasPrice.md)> |  | [optional]
**network_fee** | Option<[**models::TransactionRequestNetworkFee**](TransactionRequestNetworkFee.md)> |  | [optional]
**replace_tx_by_hash** | Option<**String**> | **For EVM-based blockchains only.**  Notes: - To replace a transaction, create another transaction using the stuck transaction's hash and specify a higher fee. - To drop a transaction, create another transaction using the stuck transaction's hash and change the `amount` field to 0.  | [optional]
**extra_parameters** | Option<[**models::ExtraParameters**](ExtraParameters.md)> |  | [optional]
**customer_ref_id** | Option<**String**> | The ID for AML providers to associate the owner of funds with transactions. | [optional]
**travel_rule_message** | Option<[**models::TravelRuleCreateTransactionRequest**](TravelRuleCreateTransactionRequest.md)> |  | [optional]
**auto_staking** | Option<**bool**> | This feature is no longer supported. | [optional]
**network_staking** | Option<[**models::TransactionRequestNetworkStaking**](TransactionRequestNetworkStaking.md)> |  | [optional]
**cpu_staking** | Option<[**models::TransactionRequestNetworkStaking**](TransactionRequestNetworkStaking.md)> |  | [optional]
**use_gasless** | Option<**bool**> | - Override the default gasless configuration by sending true\\false | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


