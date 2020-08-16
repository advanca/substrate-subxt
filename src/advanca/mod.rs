// Copyright (C) 2020 ADVANCA PTE. LTD.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//! Advanca runtime definition and pallets definitions
pub use self::advanca_core::AdvancaCore;
pub use runtime::AdvancaRuntime;

pub mod advanca_core;
pub mod runtime;

#[cfg(test)]
pub use tests::{
    test_client,
    test_client_with,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Client,
        ClientBuilder,
    };
    use sp_keyring::AccountKeyring;
    use substrate_subxt_client::{
        DatabaseConfig,
        KeystoreConfig,
        Role,
        SubxtClient,
        SubxtClientConfig,
    };
    use tempdir::TempDir;

    pub type TestRuntime = AdvancaRuntime;

    pub async fn test_client_with(key: AccountKeyring) -> (Client<TestRuntime>, TempDir) {
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
