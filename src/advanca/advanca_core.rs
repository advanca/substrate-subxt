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

//! Implements support for the advanca-core module.

use crate::frame::{
    system::{
        System,
        SystemEventsDecoder,
    },
    balances::{
        Balances,
        BalancesEventsDecoder,
    },
};
use codec::{
    Decode,
    Encode,
};
use core::marker::PhantomData;
use std::fmt::Debug;
use advanca_core::Enclave;

/// The subset of the `advanca_core::Trait` that a client must implement.
#[module]
pub trait AdvancaCore: System + Balances {}


// #[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, Clone)]
// /// The public information about an Enclave
// pub struct Enclave<AccountId> {
//     /// Enclave account on chain
//     pub account_id: AccountId,
//     /// Enclave public key for encryption
//     pub public_key: Vec<u8>,
//     /// Enclave attestation information which certifies all the other fields
//     pub attestation: Vec<u8>,
// }

/// Register user
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterUserCall<T: AdvancaCore> {
    /// The deposit that registration needs
    pub deposit: <T as Balances>::Balance,
    /// The public key of user
    pub public_key: Vec<u8>
}

/// Deregister user
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct DeregisterUserCall<T: AdvancaCore> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>
}

/// Register worker
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterWorkerCall<T: AdvancaCore> {
    /// The deposit that registration needs
    pub deposit: <T as Balances>::Balance,
    /// The public key of enclave
    pub enclave: Enclave<<T as System>::AccountId>,
}

/// Deregister worker
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct DeregisterWorkerCall<T: AdvancaCore> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>
}

/// Event: UserAdded
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UserAddedEvent<T: AdvancaCore> {
    /// User account
    pub user: <T as System>::AccountId,
}

/// Event: UserRemoved
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UserRemovedEvent<T: AdvancaCore> {
    /// User account
    pub user: <T as System>::AccountId,
}

/// Event: TaskSubmitted
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaskSubmittedEvent<T: AdvancaCore> {
    /// Task ID
    pub task_id: <T as System>::Hash,
}

/// Event: TaskUpdated
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaskUpdatedEvent<T: AdvancaCore> {
    /// Task ID
    pub task_id: <T as System>::Hash,
}

/// Event: TaskAccepted
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaskAcceptedEvent<T: AdvancaCore> {
    /// Task ID
    pub task_id: <T as System>::Hash,
}

/// Event: TaskCompleted
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaskCompletedEvent<T: AdvancaCore> {
    /// Task ID
    pub task_id: <T as System>::Hash,
}

/// Event: TaskAborted
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TaskAbortedEvent<T: AdvancaCore> {
    /// Task ID
    pub task_id: <T as System>::Hash,
}

/// Event: WorkerAdded
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct WorkerAddedEvent<T: AdvancaCore> {
    /// Worker account
    pub worker: <T as System>::AccountId,
}

/// Event: WorkerRemoved
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct WorkerRemovedEvent<T: AdvancaCore> {
    /// Worker account
    pub worker: <T as System>::AccountId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        extrinsic::{
            PairSigner,
            Signer,
        },
        Runtime,
    };
    use sp_keyring::AccountKeyring;
    use crate::advanca::tests::{
        test_client,
        TestRuntime,
    };

    #[async_std::test]
    async fn test_user_registration() {
        let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
        let (client, _) = test_client().await;

        // registration
        let event = client
            .register_user_and_watch(&alice, 10_000, "alice".as_bytes().to_vec())
            .await
            .unwrap()
            .user_added()
            .unwrap()
            .unwrap();
        let expected_event = UserAddedEvent {
            user: alice.account_id().clone()
        };
        assert_eq!(event, expected_event);

        // deregistration
        let event = client
            .deregister_user_and_watch(&alice)
            .await
            .unwrap()
            .user_removed()
            .unwrap()
            .unwrap();
        let expected_event = UserRemovedEvent {
            user: alice.account_id().clone()
        };

        assert_eq!(event, expected_event);
    }

    #[async_std::test]
    async fn test_worker_registration() {
        let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
        let bob = PairSigner::<TestRuntime, _>::new(AccountKeyring::Bob.pair());
        let (client, _) = test_client().await;

        let enclave = Enclave::<<TestRuntime as System>::AccountId> {
            account_id: bob.account_id().clone(),
            public_key: "bob".as_bytes().to_vec(),
            attestation: "attestation".as_bytes().to_vec(),
        };

        // registration
        let event = client
            .register_worker_and_watch(&alice, 10_000, enclave)
            .await
            .unwrap()
            .worker_added()
            .unwrap()
            .unwrap();
        let expected_event = WorkerAddedEvent {
            worker: alice.account_id().clone()
        };
        assert_eq!(event, expected_event);

        // deregistration
        let event = client
            .deregister_worker_and_watch(&alice)
            .await
            .unwrap()
            .worker_removed()
            .unwrap()
            .unwrap();
        let expected_event = WorkerRemovedEvent {
            worker: alice.account_id().clone()
        };

        assert_eq!(event, expected_event);
    }
}
