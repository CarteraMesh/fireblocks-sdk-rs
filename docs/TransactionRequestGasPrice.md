# TransactionRequestGasPrice

## Enum Variants

| Name | Description |
|---- | -----|
| String | **For non-EIP-1559, EVM-based transactions.** Price per gas unit. In Ethereum, this is specified in Gwei.  Notes: - Only two of the three arguments can be specified in a single transaction: &#x60;gasLimit&#x60;, &#x60;gasPrice&#x60;, and/or &#x60;networkFee&#x60;. - Fireblocks recommends using a numeric string for more precision. Although a number input exists, it is deprecated.  |
| f64 | **For non-EIP-1559, EVM-based transactions.** Price per gas unit. In Ethereum, this is specified in Gwei.  Notes: - Only two of the three arguments can be specified in a single transaction: &#x60;gasLimit&#x60;, &#x60;gasPrice&#x60;, and/or &#x60;networkFee&#x60;. - Fireblocks recommends using a numeric string for more precision. Although a number input exists, it is deprecated.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


