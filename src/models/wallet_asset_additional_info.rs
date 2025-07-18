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
pub struct WalletAssetAdditionalInfo {
    #[serde(
        rename = "accountHolderGivenName",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_given_name: Option<String>,
    #[serde(
        rename = "accountHolderSurname",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_surname: Option<String>,
    #[serde(rename = "accountHolderCity", skip_serializing_if = "Option::is_none")]
    pub account_holder_city: Option<String>,
    #[serde(
        rename = "accountHolderCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_country: Option<String>,
    #[serde(
        rename = "accountHolderAddress1",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_address1: Option<String>,
    #[serde(
        rename = "accountHolderAddress2",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_address2: Option<String>,
    #[serde(
        rename = "accountHolderDistrict",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_district: Option<String>,
    #[serde(
        rename = "accountHolderPostalCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_holder_postal_code: Option<String>,
    #[serde(rename = "abaRoutingNumber", skip_serializing_if = "Option::is_none")]
    pub aba_routing_number: Option<String>,
    #[serde(rename = "abaAccountNumber", skip_serializing_if = "Option::is_none")]
    pub aba_account_number: Option<String>,
    #[serde(rename = "abaCountry", skip_serializing_if = "Option::is_none")]
    pub aba_country: Option<String>,
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "ibanCity", skip_serializing_if = "Option::is_none")]
    pub iban_city: Option<String>,
    #[serde(rename = "ibanCountry", skip_serializing_if = "Option::is_none")]
    pub iban_country: Option<String>,
    #[serde(rename = "speiClabe", skip_serializing_if = "Option::is_none")]
    pub spei_clabe: Option<String>,
    #[serde(rename = "speiName", skip_serializing_if = "Option::is_none")]
    pub spei_name: Option<String>,
}

impl WalletAssetAdditionalInfo {
    pub fn new() -> WalletAssetAdditionalInfo {
        WalletAssetAdditionalInfo {
            account_holder_given_name: None,
            account_holder_surname: None,
            account_holder_city: None,
            account_holder_country: None,
            account_holder_address1: None,
            account_holder_address2: None,
            account_holder_district: None,
            account_holder_postal_code: None,
            aba_routing_number: None,
            aba_account_number: None,
            aba_country: None,
            iban: None,
            iban_city: None,
            iban_country: None,
            spei_clabe: None,
            spei_name: None,
        }
    }
}
