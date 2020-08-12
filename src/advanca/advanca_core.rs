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

/// The subset of the `advanca_core::Trait` that a client must implement.
#[module]
pub trait AdvancaCore: System + Balances {}

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
    pub enclave: Vec<u8>
}

/// Deregister worker
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct DeregisterWorkerCall<T: AdvancaCore> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>
}

/// Event indicating a new user is registered
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UserAddedEvent<T: AdvancaCore> {
    /// The registered user
    pub user: <T as System>::AccountId,
}

/// Event indicating a new user is registered
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UserRemovedEvent<T: AdvancaCore> {
    /// The registered user
    pub user: <T as System>::AccountId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        extrinsic::{
            PairSigner,
            Signer,
        },
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
}
