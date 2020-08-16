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

use async_std::task;
use log::info;
use sp_core::{
    crypto::Pair,
    sr25519,
};
use sp_keyring::AccountKeyring;
use std::time::Duration;
use substrate_subxt::{
    advanca::{
        advanca_core::{
            RegisterUserCallExt,
            UserAddedEventExt,
        },
        AdvancaRuntime,
    },
    balances::{
        TransferCallExt,
        TransferEventExt,
    },
    ClientBuilder,
    PairSigner,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let url = "ws://127.0.0.1:9944";
    let client = ClientBuilder::<AdvancaRuntime>::new()
        .set_url(url)
        .build()
        .await?;
    let alice = PairSigner::new(AccountKeyring::Alice.pair());

    let (user_keypair, _) = sr25519::Pair::generate();
    let user_account = user_keypair.public().as_array_ref().to_owned().into();
    let user = PairSigner::new(user_keypair);

    let event = client
        .transfer_and_watch(&alice, &user_account, 10_000_000_000)
        .await
        .unwrap()
        .transfer()
        .unwrap()
        .unwrap();

    info!("event {:?}", event);

    // task::sleep(Duration::from_secs(10)).await;

    let event = client
        .transfer_and_watch(&user, &AccountKeyring::Alice.to_account_id(), 10)
        .await
        .unwrap()
        .transfer()
        .unwrap()
        .unwrap();

    info!("event {:?}", event);

    // let expected_event = TransferEvent {
    //     from: user_account,
    //     to: AccountKeyring::Alice.to_account_id(),
    //     amount: 10,
    // };

    Ok(())
}
