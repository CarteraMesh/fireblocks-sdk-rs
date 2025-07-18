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
pub struct ScreeningProviderRulesConfigurationResponse {
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "amountUSD", skip_serializing_if = "Option::is_none")]
    pub amount_usd: Option<f64>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "action")]
    pub action: Action,
}

impl ScreeningProviderRulesConfigurationResponse {
    pub fn new(action: Action) -> ScreeningProviderRulesConfigurationResponse {
        ScreeningProviderRulesConfigurationResponse {
            direction: None,
            status: None,
            amount_usd: None,
            amount: None,
            asset: None,
            action,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "INBOUND")]
    Inbound,
    #[serde(rename = "OUTBOUND")]
    Outbound,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Inbound
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "BLOCKING_TIME_EXPIRED")]
    BlockingTimeExpired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Completed
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "ACCEPT")]
    Accept,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "ALERT")]
    Alert,
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "FREEZE")]
    Freeze,
    #[serde(rename = "CANCEL")]
    Cancel,
}

impl Default for Action {
    fn default() -> Action {
        Self::Accept
    }
}
