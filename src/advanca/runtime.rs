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
//! runtime

use crate::{
    advanca::AdvancaCore,
    extrinsic::DefaultExtra,
    frame::{
        balances::{
            AccountData,
            Balances,
            Status,
        },
        sudo::Sudo,
        system::System,
    },
    runtimes::Runtime,
};
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
    type Status = Status;
}

impl AdvancaCore for AdvancaRuntime {
    type TaskId = sp_core::H256;
}

impl Sudo for AdvancaRuntime {}
