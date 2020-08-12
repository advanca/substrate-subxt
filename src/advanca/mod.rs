//! Advanca runtime definition and pallets definitions
pub use runtime::AdvancaRuntime;
pub use self::advanca_core::AdvancaCore;

pub mod runtime;
pub mod advanca_core;


#[cfg(test)]
pub use tests::{test_client, test_client_with};

#[cfg(test)]
mod tests {
    use super::*;
    use sp_keyring::AccountKeyring;
    use substrate_subxt_client::{
        DatabaseConfig,
        KeystoreConfig,
        Role,
        SubxtClient,
        SubxtClientConfig,
    };
    use crate::{Client, ClientBuilder};
    use tempdir::TempDir;

    pub type TestRuntime = AdvancaRuntime;

    pub async fn test_client_with(
        key: AccountKeyring,
    ) -> (Client<TestRuntime>, TempDir) {
        env_logger::try_init().ok();
        let tmp = TempDir::new("subxt-").expect("failed to create tempdir");
        let config = SubxtClientConfig {
            impl_name: "substrate-subxt-full-client",
            impl_version: "0.0.1",
            author: "substrate subxt",
            copyright_start_year: 2020,
            db: DatabaseConfig::RocksDb {
                path: tmp.path().join("db"),
                cache_size: 128,
            },
            keystore: KeystoreConfig::Path {
                path: tmp.path().join("keystore"),
                password: None,
            },
            chain_spec: test_node::chain_spec::development_config().unwrap(),
            role: Role::Authority(key),
            enable_telemetry: false,
        };
        let client = ClientBuilder::new()
            .set_client(
                SubxtClient::from_config(config, test_node::service::new_full)
                    .expect("Error creating subxt client"),
            )
            .set_page_size(2)
            .build()
            .await
            .expect("Error creating client");
        (client, tmp)
    }

    pub async fn test_client() -> (Client<TestRuntime>, TempDir) {
        test_client_with(AccountKeyring::Alice).await
    }
}
