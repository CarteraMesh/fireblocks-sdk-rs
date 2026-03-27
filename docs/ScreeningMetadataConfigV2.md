# ScreeningMetadataConfigV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | **Direction** | Direction of transaction (enum: INBOUND, OUTBOUND, ANY) | 
**provider** | Option<**Provider**> | Screening provider (enum: CHAINALYSIS, CHAINALYSIS_V2, ELLIPTIC, ELLIPTIC_HOLISTIC, NOTABENE) | [optional]
**risk_rating** | Option<**RiskRating**> | Risk rating threshold (enum: LOW, MEDIUM, HIGH, SEVERE, ANY) | [optional]
**risk_score** | Option<**String**> | Risk score threshold | [optional]
**exposure_type** | Option<**ExposureType**> | Exposure type (enum: DIRECT, INDIRECT, ANY) | [optional]
**category** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**Vec<String>**> |  | [optional]
**category_id** | Option<**Vec<String>**> |  | [optional]
**status** | Option<**Status**> | Transaction status (enum: COMPLETED, PENDING, REJECTED, FAILED, CANCELED, BLOCKING_TIME_EXPIRED) | [optional]
**source_address** | Option<**String**> | Source address | [optional]
**dest_address** | Option<**String**> | Destination address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


