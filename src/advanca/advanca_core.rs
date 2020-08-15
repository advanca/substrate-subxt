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
    balances::{Balances, BalancesEventsDecoder},
    system::{System, SystemEventsDecoder},
};
use advanca_node_primitives::{Duration, Enclave, Privacy, TaskSpec, Ciphertext, User, Worker, Task, TaskStatus};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use frame_support::Parameter;
use sp_runtime::{
    traits::{
        CheckEqual, Hash,
        MaybeDisplay, MaybeMallocSizeOf, MaybeSerializeDeserialize, Member,
        SimpleBitOps,
    },
};
use std::fmt::Debug;

/// The subset of the `advanca_core::Trait` that a client must implement.
#[module]
pub trait AdvancaCore: System + Balances {
    /// The type of task's id
    //FIXME This is a bit hacky as TaskId was not declared as part of the advanca_core::Trait
    type TaskId: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + SimpleBitOps
        + Ord
        + Default
        + Copy
        + CheckEqual
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>
        + MaybeMallocSizeOf;
}

/// Registered Users
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct UsersStore<T: AdvancaCore> {
    #[store(returns = User::<<T as System>::AccountId>)]
    /// Account id
    pub account_id: <T as System>::AccountId,
}

/// Registered Workers
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct WorkersStore<T: AdvancaCore> {
    #[store(returns = Worker::<<T as System>::AccountId>)]
    /// Account id
    pub account_id: <T as System>::AccountId,
}

/// Saved Tasks
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct TasksStore<T: AdvancaCore> {
    #[store(returns = Task::<T::TaskId, <T as System>::AccountId, Duration, TaskSpec<Privacy>, TaskStatus, Ciphertext>)]
    /// Task id
    pub task_id: T::TaskId,
}

/// Unscheduled Tasks
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct UnscheduledTasksStore<T: AdvancaCore> {
    #[store(returns = Vec<T::TaskId>)]
    /// Runtime marker
    pub _runtime: PhantomData<T>,
}

/// Register user
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterUserCall<T: AdvancaCore> {
    /// The deposit that registration needs
    pub deposit: <T as Balances>::Balance,
    /// The public key of user
    pub public_key: Vec<u8>,
}

/// Deregister user
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct DeregisterUserCall<T: AdvancaCore> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
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
    pub _runtime: PhantomData<T>,
}

/// Submit task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitTaskCall<T: AdvancaCore> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Task owner's public key
    pub signed_owner_task_pubkey: Vec<u8>,
    /// Lease of task
    pub lease: Duration,
    /// Task specification
    pub task_spec: TaskSpec<Privacy>,
}

/// Submit task evidence
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitTaskEvidenceCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
    /// Evidences
    pub evidences: Vec<Vec<u8>>,
}

/// Update task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct UpdateTaskCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
    /// Task specification
    pub task_spec: TaskSpec<Privacy>,
}

/// Accept task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct AcceptTaskCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
    /// Worker's ephemeral key for particular task
    pub signed_eph_pubkey: Vec<u8>,
    /// Encrypted URL
    pub url: Ciphertext,
}

/// Abort task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct AbortTaskCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
}

/// Complete task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct CompleteTaskCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
}

/// Progress task
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct ProgressTaskCall<T: AdvancaCore> {
    /// Task id
    pub task_id: T::TaskId,
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

#[allow(dead_code)]
/// The implementation taken from advanca-core module
fn task_id<T: AdvancaCore>(account_id: &<T as System>::AccountId, account_nonce: <T as System>::Index) -> <T as System>::Hash {
    let mut x = account_id.encode();
    account_nonce.using_encoded(|a| x.append(&mut a.to_vec()));
    T::Hashing::hash(&x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{
        system::AccountStoreExt,
        balances::*,
    };
    use crate::advanca::tests::{test_client, TestRuntime};
    use crate::extrinsic::{PairSigner, Signer};
    use sp_keyring::AccountKeyring;
    use sp_core::{crypto::Pair, sr25519};

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
            user: alice.account_id().clone(),
        };
        assert_eq!(event, expected_event);

        let user = client
            .users(alice.account_id().clone(), None)
            .await
            .unwrap();

        assert_eq!(user, User::<<TestRuntime as System>::AccountId>{
            account_id: alice.account_id().clone(),
            public_key: "alice".into(),
        });

        // deregistration
        let event = client
            .deregister_user_and_watch(&alice)
            .await
            .unwrap()
            .user_removed()
            .unwrap()
            .unwrap();
        let expected_event = UserRemovedEvent {
            user: alice.account_id().clone(),
        };

        assert_eq!(event, expected_event);
    }

    #[async_std::test]
    async fn test_new_account_transferring() {
        let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());

        let (client, _) = test_client().await;

        let (user_keypair, _) = sr25519::Pair::generate();
        let user_account = user_keypair.public().as_array_ref().to_owned().into();
        let user = PairSigner::new(user_keypair);

        client.transfer_and_watch(&alice, &user_account, 10_000_000_000).await.unwrap();

        let event = client.transfer_and_watch(&user, &AccountKeyring::Alice.to_account_id(), 10)
            .await
            .unwrap()
            .transfer()
            .unwrap()
            .unwrap();

        let expected_event = TransferEvent {
            from: user_account,
            to: AccountKeyring::Alice.to_account_id(),
            amount: 10,
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
            worker: alice.account_id().clone(),
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
            worker: alice.account_id().clone(),
        };

        assert_eq!(event, expected_event);
    }

    #[async_std::test]
    async fn test_task_submission() {
        let _ = env_logger::try_init();
        let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
        let (client, _) = test_client().await;

        let signed_owner_task_pubkey = "signed_owner_task_pubkey".into();
        let duration = 0;
        let task_spec = TaskSpec::<Privacy>::default();

        // task submission
        let event = client
            .submit_task_and_watch(&alice, signed_owner_task_pubkey, duration, task_spec)
            .await
            .unwrap()
            .task_submitted()
            .unwrap()
            .unwrap();

        let alice_nonce = client.account(alice.account_id(), None).await.unwrap().nonce;
        let task_id_alice = task_id::<TestRuntime>(&alice.account_id(), alice_nonce);
        let expected_event = TaskSubmittedEvent {
            task_id: task_id_alice.clone()
        };

        assert_eq!(event, expected_event);

        // check the unscheduled tasks

        let unscheduled_tasks = client
            .unscheduled_tasks(None)
            .await
            .unwrap();

        assert_eq!(unscheduled_tasks, vec![task_id_alice]);
    }
}
