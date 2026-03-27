# TokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Fireblocks NFT asset id | 
**token_id** | **String** | Token id within the contract/collection | 
**standard** | **String** | ERC721 / ERC1155 | 
**metadata_uri** | Option<**String**> | URL of the original token JSON metadata | [optional]
**cached_metadata_uri** | Option<**String**> | URL of the cached token JSON metadata | [optional]
**media** | Option<[**Vec<models::MediaEntityResponse>**](MediaEntityResponse.md)> | Media items extracted from metadata JSON | [optional]
**spam** | Option<[**models::SpamTokenResponse**](SpamTokenResponse.md)> | Token spam status | [optional]
**collection** | Option<[**models::TokenCollectionResponse**](TokenCollectionResponse.md)> | Parent collection information | [optional]
**blockchain_descriptor** | **BlockchainDescriptor** |  (enum: ETH, ETH_TEST3, ETH_TEST5, ETH_TEST6, POLYGON, POLYGON_TEST_MUMBAI, AMOY_POLYGON_TEST, XTZ, XTZ_TEST, BASECHAIN_ETH, BASECHAIN_ETH_TEST3, BASECHAIN_ETH_TEST5, ETHERLINK, ETHERLINK_TEST, MANTLE, MANTLE_TEST, GUN_GUNZILLA_TEST, ETH_SONEIUM, SONEIUM_MINATO_TEST, IOTX_IOTEX) | 
**description** | Option<**String**> | Token's metadata description | [optional]
**name** | Option<**String**> | Token's metadata name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


