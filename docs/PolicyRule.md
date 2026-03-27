# PolicyRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operator** | Option<**String**> | (deprecated - replaced by \"operators\")  | Defines users who can initiate the type of transaction to which the rule applies. options are * \"*\" - All users are allowed * Specific User id | [optional]
**operators** | Option<[**models::PolicyRuleOperators**](PolicyRuleOperators.md)> |  | [optional]
**transaction_type** | Option<**TransactionType**> | Defines the type of transaction to which the rule applies.   * TRANSFER - Default. Transfers funds from one account to another   * CONTRACT_CALL - Calls a smart contract, mainly for DeFi operations.   * PROGRAM_CALL - Calls a smart contract for web3 operations on the Solana blockchain.    * APPROVE - Allows a smart contract to withdraw from a designated wallet.   * MINT - Perform a mint operation (increase supply) on a supported token   * BURN - Perform a burn operation (reduce supply) on a supported token   * SUPPLY - Use for DeFi to lend assets   * REDEEM - Use for DeFi to get lending back   * STAKE - Allows you to allocate and lock certain assets for earning staking rewards.   * RAW - An off-chain message with no predefined format, use it to sign any message with your private key.   * TYPED_MESSAGE - An off-chain message type that follows a predefined format, used to sign specific messages that are not actual transactions.  (enum: TRANSFER, CONTRACT_CALL, PROGRAM_CALL, APPROVE, MINT, BURN, SUPPLY, REDEEM, STAKE, RAW, TYPED_MESSAGE) | [optional]
**designated_signer** | Option<**String**> | (deprecated - replaced by \"designatedSigners\") Id representing the user who signs transactions that match a specific rule | [optional]
**designated_signers** | Option<[**models::PolicyRuleDesignatedSigners**](PolicyRuleDesignatedSigners.md)> |  | [optional]
**r#type** | **Type** | Policy rule type (enum: TRANSFER) | 
**action** | **Action** | Defines what occurs when a transaction meets the rule's criteria * ALLOW - The transaction goes through and can be signed without requiring additional approvals * BLOCK - The transaction is automatically blocked * 2-TIER - Only these users or user groups can approve             If any of them reject the transaction before the required approval threshold is met, the transaction doesn't go through            The list of entities are set is \"authorizationGroups\" field  (enum: ALLOW, BLOCK, 2-TIER) | 
**asset** | **String** | Defines the type of asset being transacted, options are * \"*\" - All assets * Specific asset  | 
**src_type** | Option<[**models::PolicySrcOrDestType**](PolicySrcOrDestType.md)> | (deprecated - replaced by \"src\") source account type | [optional]
**src_sub_type** | Option<[**models::PolicySrcOrDestSubType**](PolicySrcOrDestSubType.md)> | (deprecated - replaced by \"src\") source sub account type | [optional]
**src_id** | Option<**String**> | (deprecated - replaced by \"src\") source account id | [optional]
**src** | Option<[**models::PolicyRuleSrc**](PolicyRuleSrc.md)> |  | [optional]
**dst_type** | Option<[**models::PolicySrcOrDestType**](PolicySrcOrDestType.md)> | (deprecated - replaced by \"dst\") destination account type | [optional]
**dst_sub_type** | Option<[**models::PolicySrcOrDestSubType**](PolicySrcOrDestSubType.md)> | (deprecated - replaced by \"dst\") destination sub account type | [optional]
**dst_id** | Option<**String**> | (deprecated - replaced by \"dst\") destination account id | [optional]
**dst** | Option<[**models::PolicyRuleDst**](PolicyRuleDst.md)> |  | [optional]
**dst_address_type** | Option<**DstAddressType**> | Defines whether the destination to which you are sending funds must be whitelisted, to allow one-time transfers to non-whitelisted external addresses, or both. By default, you can only transfer to an external address after it’s whitelisted.   * WHITELISTED - Can only be sent to whitelisted addresses.   * ONE_TIME - Can only be sent to non-whitelisted external addresses.   * \"*\" - can be sent to whitelisted addresses or non-whitelisted external  (enum: WHITELISTED, ONE_TIME, *) | [optional]
**amount_currency** | **AmountCurrency** | * USD - Limits the amount of any asset users can transfer based on the USD equivalent of the asset. * EUR - Limits the amount of any asset users can transfer based on the EURO equivalent of the asset. * NATIVE - Limits the amount of an asset a user can transfer when using a specific asset.  (enum: USD, EUR, NATIVE) | 
**amount_scope** | **AmountScope** | * SINGLE_TX - limit applies to a single transaction * TIMEFRAME - limit applies to all transactions within the defined time period  (enum: SINGLE_TX, TIMEFRAME) | 
**amount** | [**models::PolicyRuleAmount**](PolicyRuleAmount.md) |  | 
**period_sec** | **f64** | Time period in seconds applied by the amountScope field to accumulate transferred amounts in transactions that match the rule, until the total exceeds the value you specify under Minimum. When the specified amount is reached within that period, whether by one or many transactions, further transactions in that period either fail or require more approvals.  | 
**authorizers** | Option<**Vec<String>**> | (deprecated - replaced by \"authorizationGroups\") Allowed entities which can approves a transaction | [optional]
**authorizers_count** | Option<**f64**> | (deprecated - replaced by \"authorizationGroups\") Min amount of entities which are needed to approve a transaction | [optional]
**authorization_groups** | Option<[**models::PolicyRuleAuthorizationGroups**](PolicyRuleAuthorizationGroups.md)> |  | [optional]
**amount_aggregation** | Option<[**models::PolicyRuleAmountAggregation**](PolicyRuleAmountAggregation.md)> |  | [optional]
**raw_message_signing** | Option<[**models::PolicyRuleRawMessageSigning**](PolicyRuleRawMessageSigning.md)> |  | [optional]
**apply_for_approve** | Option<**bool**> | Applying this rule over APPROVE type transactions (can only be enabled when rule's transaction type is TRANSFER) | [optional]
**apply_for_typed_message** | Option<**bool**> | Applying this rule over TYPED_MESSAGE type transactions (can only be enabled when rule's transaction type is CONTRACT_CALL) | [optional]
**external_descriptor** | Option<**String**> | A unique id identifying the rule | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


