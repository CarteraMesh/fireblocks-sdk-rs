// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FetchAbiRequestDto {
    /// The blockchain base assetId
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
    /// The contract's onchain address
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl FetchAbiRequestDto {
    pub fn new(base_asset_id: String, contract_address: String) -> FetchAbiRequestDto {
        FetchAbiRequestDto {
            base_asset_id,
            contract_address,
        }
    }
}
