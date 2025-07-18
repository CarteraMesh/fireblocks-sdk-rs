// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    super::{Error, configuration},
    crate::{
        apis::{ContentType, ResponseContent},
        models,
    },
    async_trait::async_trait,
    reqwest,
    serde::{Deserialize, Serialize, de::Error as _},
    std::sync::Arc,
};

#[async_trait]
pub trait WhitelistedExternalWalletsApi: Send + Sync {
    /// POST /external_wallets/{walletId}/{assetId}
    ///
    /// Adds an asset to an existing external wallet. </br>Endpoint Permission:
    /// Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn add_asset_to_external_wallet(
        &self,
        params: AddAssetToExternalWalletParams,
    ) -> Result<models::ExternalWalletAsset, Error<AddAssetToExternalWalletError>>;

    /// POST /external_wallets
    ///
    /// Creates a new external wallet with the requested name.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn create_external_wallet(
        &self,
        params: CreateExternalWalletParams,
    ) -> Result<models::UnmanagedWallet, Error<CreateExternalWalletError>>;

    /// DELETE /external_wallets/{walletId}
    ///
    /// Deletes an external wallet by ID.  External Wallet is a whitelisted
    /// address of a wallet that belongs to your users/counterparties.  - You
    /// cannot see the balance of the external wallet. - You cannot initiate
    /// transactions from an external wallet as the source via Fireblocks.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver,
    /// Editor.
    async fn delete_external_wallet(
        &self,
        params: DeleteExternalWalletParams,
    ) -> Result<(), Error<DeleteExternalWalletError>>;

    /// GET /external_wallets/{walletId}
    ///
    /// Returns an external wallet by ID.  External Wallet is a whitelisted
    /// address of a wallet that belongs to your users/counterparties.  - You
    /// cannot see the balance of the external wallet. - You cannot initiate
    /// transactions from an external wallet as the source via Fireblocks.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver,
    /// Editor, Viewer.
    async fn get_external_wallet(
        &self,
        params: GetExternalWalletParams,
    ) -> Result<models::UnmanagedWallet, Error<GetExternalWalletError>>;

    /// GET /external_wallets/{walletId}/{assetId}
    ///
    /// Returns an external wallet by wallet ID and asset ID.  External Wallet
    /// is a whitelisted address of a wallet that belongs to your
    /// users/counterparties.  - You cannot see the balance of the external
    /// wallet. - You cannot initiate transactions from an external wallet as
    /// the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing
    /// Admin, Signer, Approver, Editor,   Viewer.
    async fn get_external_wallet_asset(
        &self,
        params: GetExternalWalletAssetParams,
    ) -> Result<models::ExternalWalletAsset, Error<GetExternalWalletAssetError>>;

    /// GET /external_wallets
    ///
    /// Gets a list of external wallets under the workspace.  External Wallet is
    /// a whitelisted address of a wallet that belongs to your
    /// users/counterparties.  - You cannot see the balance of the external
    /// wallet. - You cannot initiate transactions from an external wallet as
    /// the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing
    /// Admin, Signer, Approver, Editor, Viewer.
    async fn get_external_wallets(
        &self,
    ) -> Result<Vec<models::UnmanagedWallet>, Error<GetExternalWalletsError>>;

    /// DELETE /external_wallets/{walletId}/{assetId}
    ///
    /// Deletes an external wallet asset by ID. </br>Endpoint Permission: Admin,
    /// Non-Signing Admin, Signer, Approver, Editor.
    async fn remove_asset_from_external_wallet(
        &self,
        params: RemoveAssetFromExternalWalletParams,
    ) -> Result<(), Error<RemoveAssetFromExternalWalletError>>;

    /// POST /external_wallets/{walletId}/set_customer_ref_id
    ///
    /// Sets an AML/KYT customer reference ID for the specific external wallet.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn set_external_wallet_customer_ref_id(
        &self,
        params: SetExternalWalletCustomerRefIdParams,
    ) -> Result<(), Error<SetExternalWalletCustomerRefIdError>>;
}

pub struct WhitelistedExternalWalletsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl WhitelistedExternalWalletsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`add_asset_to_external_wallet`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct AddAssetToExternalWalletParams {
    /// The ID of the wallet
    pub wallet_id: String,
    /// The ID of the asset to add
    pub asset_id: String,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
    pub add_asset_to_external_wallet_request: Option<models::AddAssetToExternalWalletRequest>,
}

/// struct for passing parameters to the method [`create_external_wallet`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct CreateExternalWalletParams {
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
    pub create_wallet_request: Option<models::CreateWalletRequest>,
}

/// struct for passing parameters to the method [`delete_external_wallet`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct DeleteExternalWalletParams {
    /// The ID of the wallet to delete
    pub wallet_id: String,
}

/// struct for passing parameters to the method [`get_external_wallet`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetExternalWalletParams {
    /// The ID of the wallet to return
    pub wallet_id: String,
}

/// struct for passing parameters to the method [`get_external_wallet_asset`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetExternalWalletAssetParams {
    /// The ID of the wallet
    pub wallet_id: String,
    /// The ID of the asset to return
    pub asset_id: String,
}

/// struct for passing parameters to the method
/// [`remove_asset_from_external_wallet`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RemoveAssetFromExternalWalletParams {
    /// The ID of the wallet
    pub wallet_id: String,
    /// The ID of the asset to delete
    pub asset_id: String,
}

/// struct for passing parameters to the method
/// [`set_external_wallet_customer_ref_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct SetExternalWalletCustomerRefIdParams {
    /// The wallet ID
    pub wallet_id: String,
    pub set_customer_ref_id_request: models::SetCustomerRefIdRequest,
    /// A unique identifier for the request. If the request is sent multiple
    /// times with the same idempotency key, the server will return the same
    /// response as the first request. The idempotency key is valid for 24
    /// hours.
    pub idempotency_key: Option<String>,
}

#[async_trait]
impl WhitelistedExternalWalletsApi for WhitelistedExternalWalletsApiClient {
    /// Adds an asset to an existing external wallet. </br>Endpoint Permission:
    /// Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn add_asset_to_external_wallet(
        &self,
        params: AddAssetToExternalWalletParams,
    ) -> Result<models::ExternalWalletAsset, Error<AddAssetToExternalWalletError>> {
        let AddAssetToExternalWalletParams {
            wallet_id,
            asset_id,
            idempotency_key,
            add_asset_to_external_wallet_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}/{assetId}",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id),
            assetId = crate::apis::urlencode(asset_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&add_asset_to_external_wallet_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::ExternalWalletAsset`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::ExternalWalletAsset`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<AddAssetToExternalWalletError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Creates a new external wallet with the requested name.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn create_external_wallet(
        &self,
        params: CreateExternalWalletParams,
    ) -> Result<models::UnmanagedWallet, Error<CreateExternalWalletError>> {
        let CreateExternalWalletParams {
            idempotency_key,
            create_wallet_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/external_wallets", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&create_wallet_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::UnmanagedWallet`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::UnmanagedWallet`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<CreateExternalWalletError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes an external wallet by ID.  External Wallet is a whitelisted
    /// address of a wallet that belongs to your users/counterparties.  - You
    /// cannot see the balance of the external wallet. - You cannot initiate
    /// transactions from an external wallet as the source via Fireblocks.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver,
    /// Editor.
    async fn delete_external_wallet(
        &self,
        params: DeleteExternalWalletParams,
    ) -> Result<(), Error<DeleteExternalWalletError>> {
        let DeleteExternalWalletParams { wallet_id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteExternalWalletError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns an external wallet by ID.  External Wallet is a whitelisted
    /// address of a wallet that belongs to your users/counterparties.  - You
    /// cannot see the balance of the external wallet. - You cannot initiate
    /// transactions from an external wallet as the source via Fireblocks.
    /// </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver,
    /// Editor, Viewer.
    async fn get_external_wallet(
        &self,
        params: GetExternalWalletParams,
    ) -> Result<models::UnmanagedWallet, Error<GetExternalWalletError>> {
        let GetExternalWalletParams { wallet_id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::UnmanagedWallet`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::UnmanagedWallet`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetExternalWalletError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns an external wallet by wallet ID and asset ID.  External Wallet
    /// is a whitelisted address of a wallet that belongs to your
    /// users/counterparties.  - You cannot see the balance of the external
    /// wallet. - You cannot initiate transactions from an external wallet as
    /// the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing
    /// Admin, Signer, Approver, Editor,   Viewer.
    async fn get_external_wallet_asset(
        &self,
        params: GetExternalWalletAssetParams,
    ) -> Result<models::ExternalWalletAsset, Error<GetExternalWalletAssetError>> {
        let GetExternalWalletAssetParams {
            wallet_id,
            asset_id,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}/{assetId}",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id),
            assetId = crate::apis::urlencode(asset_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::ExternalWalletAsset`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::ExternalWalletAsset`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetExternalWalletAssetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Gets a list of external wallets under the workspace.  External Wallet is
    /// a whitelisted address of a wallet that belongs to your
    /// users/counterparties.  - You cannot see the balance of the external
    /// wallet. - You cannot initiate transactions from an external wallet as
    /// the source via Fireblocks. </br>Endpoint Permission: Admin, Non-Signing
    /// Admin, Signer, Approver, Editor, Viewer.
    async fn get_external_wallets(
        &self,
    ) -> Result<Vec<models::UnmanagedWallet>, Error<GetExternalWalletsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/external_wallets", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `Vec&lt;models::UnmanagedWallet&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `Vec&lt;models::UnmanagedWallet&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetExternalWalletsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes an external wallet asset by ID. </br>Endpoint Permission: Admin,
    /// Non-Signing Admin, Signer, Approver, Editor.
    async fn remove_asset_from_external_wallet(
        &self,
        params: RemoveAssetFromExternalWalletParams,
    ) -> Result<(), Error<RemoveAssetFromExternalWalletError>> {
        let RemoveAssetFromExternalWalletParams {
            wallet_id,
            asset_id,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}/{assetId}",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id),
            assetId = crate::apis::urlencode(asset_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<RemoveAssetFromExternalWalletError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Sets an AML/KYT customer reference ID for the specific external wallet.  External Wallet is a whitelisted address of a wallet that belongs to your users/counterparties.  - You cannot see the balance of the external wallet. - You cannot initiate transactions from an external wallet as the source via Fireblocks. Learn more about Whitelisted External Wallet Addresses [here](https://developers.fireblocks.com/docs/whitelist-addresses#external-wallets). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.
    async fn set_external_wallet_customer_ref_id(
        &self,
        params: SetExternalWalletCustomerRefIdParams,
    ) -> Result<(), Error<SetExternalWalletCustomerRefIdError>> {
        let SetExternalWalletCustomerRefIdParams {
            wallet_id,
            set_customer_ref_id_request,
            idempotency_key,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/external_wallets/{walletId}/set_customer_ref_id",
            local_var_configuration.base_path,
            walletId = crate::apis::urlencode(wallet_id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = idempotency_key {
            local_var_req_builder =
                local_var_req_builder.header("Idempotency-Key", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&set_customer_ref_id_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<SetExternalWalletCustomerRefIdError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`add_asset_to_external_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddAssetToExternalWalletError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_external_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExternalWalletError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_external_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteExternalWalletError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_external_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExternalWalletError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_external_wallet_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExternalWalletAssetError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_external_wallets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExternalWalletsError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_asset_from_external_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveAssetFromExternalWalletError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_external_wallet_customer_ref_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetExternalWalletCustomerRefIdError {
    DefaultResponse(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}
