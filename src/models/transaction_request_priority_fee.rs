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

/// TransactionRequestPriorityFee : For Ethereum-based blockchains only, the fee
/// for EIP-1559 transaction pricing mechanism. Value is in Gwei.  Fireblocks
/// recommends using a numeric string for accurate precision. Although a number
/// input exists, it is deprecated. For Ethereum-based blockchains only, the fee
/// for EIP-1559 transaction pricing mechanism. Value is in Gwei.  Fireblocks
/// recommends using a numeric string for accurate precision. Although a number
/// input exists, it is deprecated.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionRequestPriorityFee {
    /// Numeric string (recommended)
    String(String),
    /// Number (deprecated)
    Number(f64),
}

impl Default for TransactionRequestPriorityFee {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
