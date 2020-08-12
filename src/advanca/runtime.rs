//! runtime

use sp_runtime::{
    generic::Header,
    traits::{
        BlakeTwo256,
        IdentifyAccount,
        Verify,
    },
    MultiSignature,
    OpaqueExtrinsic,
};
use crate::{
    extrinsic::{
        DefaultExtra,
    },
    frame::{
        balances::{
            AccountData,
            Balances,
        },
        sudo::Sudo,
        system::System,
    },
    runtimes::Runtime,
    advanca::AdvancaCore,
};

/// Concrete type definitions compatible with the Advanca Runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdvancaRuntime;

impl Runtime for AdvancaRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for AdvancaRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for AdvancaRuntime {
    type Balance = u128;
}

impl AdvancaCore for AdvancaRuntime {
    // type Currency = <Self as Balances>::Balance;
}

impl Sudo for AdvancaRuntime {}