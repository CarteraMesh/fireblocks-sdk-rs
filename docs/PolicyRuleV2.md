# PolicyRuleV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the policy rule | 
**id** | **String** | Unique identifier for the policy rule | 
**policy_engine_version** | **PolicyEngineVersion** | Policy engine version (enum: v2) | 
**r#type** | [**models::PolicyTypeV2**](PolicyTypeV2.md) |  | 
**sub_type** | Option<[**models::PolicyTypeV2**](PolicyTypeV2.md)> |  | [optional]
**initiator** | [**models::InitiatorConfigPatternV2**](InitiatorConfigPatternV2.md) |  | 
**asset** | [**models::AssetConfigV2**](AssetConfigV2.md) |  | 
**source** | [**models::AccountConfigV2**](AccountConfigV2.md) |  | 
**destination** | Option<[**models::DestinationConfigV2**](DestinationConfigV2.md)> |  | [optional]
**account** | Option<[**models::AccountConfigV2**](AccountConfigV2.md)> |  | [optional]
**verdict** | [**models::VerdictConfigV2**](VerdictConfigV2.md) |  | 
**amount_over_time** | Option<[**models::AmountOverTimeConfigV2**](AmountOverTimeConfigV2.md)> |  | [optional]
**amount** | Option<[**models::AmountRangeV2**](AmountRangeV2.md)> |  | [optional]
**external_descriptor** | Option<**String**> | External descriptor for the rule | [optional]
**method** | Option<[**models::ContractMethodPatternV2**](ContractMethodPatternV2.md)> |  | [optional]
**is_global_policy** | Option<**bool**> | Whether this is a global policy | [optional]
**program_call** | Option<[**models::ProgramCallConfigV2**](ProgramCallConfigV2.md)> |  | [optional]
**screening_metadata** | Option<[**models::ScreeningMetadataConfigV2**](ScreeningMetadataConfigV2.md)> |  | [optional]
**quote_asset** | Option<[**models::AssetConfigV2**](AssetConfigV2.md)> |  | [optional]
**base_asset** | Option<[**models::AssetConfigV2**](AssetConfigV2.md)> |  | [optional]
**quote_amount** | Option<[**models::AmountRangeV2**](AmountRangeV2.md)> |  | [optional]
**base_amount** | Option<[**models::AmountRangeV2**](AmountRangeV2.md)> |  | [optional]
**derivation_path** | Option<[**models::DerivationPathConfigV2**](DerivationPathConfigV2.md)> |  | [optional]
**index** | Option<**f64**> | Index for the policy rule | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


