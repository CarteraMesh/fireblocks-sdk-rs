use {
    crate::models::{UnmanagedExternalWallet, UnmanagedWallet, WalletAssetExternal},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WalletContainer {
    pub id: String,
    pub name: String,
    pub assets: Vec<WalletAssetExternal>,
    pub customer_ref_id: Option<String>,
}

impl From<UnmanagedWallet> for WalletContainer {
    fn from(value: UnmanagedWallet) -> Self {
        Self {
            id: value.id,
            name: value.name,
            customer_ref_id: value.customer_ref_id,
            assets: value
                .assets
                .into_iter()
                .map(|w| WalletAssetExternal {
                    id: w.id,
                    locked_amount: w.locked_amount,
                    // locked_amount: w.locked_amount,
                    address: w.address,
                    tag: w.tag,
                    activation_time: w.activation_time,
                    status: w.status,
                })
                .collect(),
        }
    }
}

impl From<UnmanagedExternalWallet> for WalletContainer {
    fn from(value: UnmanagedExternalWallet) -> Self {
        Self {
            id: value.id,
            name: value.name,
            customer_ref_id: value.customer_ref_id,
            assets: value.assets,
        }
    }
}
