// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Mocks for the currencies module.

#![cfg(test)]

use frame_support::{
    ord_parameter_types, parameter_types,
    traits::{Everything, GenesisBuild, Nothing, ConstU32},
    PalletId,
};
use orml_traits::parameter_type_with_key;
use sp_core::{H256, crypto::Ss58Codec};
use codec::{Decode, Encode, MaxEncodedLen};

use sp_runtime::{
    testing::Header,
    traits::{AccountIdConversion, IdentityLookup},
    AccountId32, Perbill,
};

use super::*;
use frame_system::EnsureSignedBy;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use scale_info::TypeInfo;

// Currencies id.
#[derive(
    Encode,
    Decode,
    Eq,
    PartialEq,
    Copy,
    Clone,
    RuntimeDebug,
    PartialOrd,
    Ord,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
    // Relaychain's currency.
    KSM,
    // Our native currency.
    PONT,
    // Test.
    TEST,
}

pub use crate as currencies;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

pub type AccountId = AccountId32;
impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = Everything;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<12>;
}

type Balance = u128;

parameter_type_with_key! {
    pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
        if *currency_id == DOT { return 2; }
        Default::default()
    };
}

parameter_types! {
    pub DustAccount: AccountId = PalletId(*b"orml/dst").into_account();
    pub const MaxLocks: u32 = 100;
}

impl orml_tokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = i64;
    type CurrencyId = CurrencyId;
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, DustAccount>;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = Nothing;
}

pub const NATIVE_CURRENCY_ID: CurrencyId = CurrencyId::PONT;
pub const X_TOKEN_ID: CurrencyId = CurrencyId::TEST;
pub const DOT: CurrencyId = CurrencyId::KSM;

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = NATIVE_CURRENCY_ID;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 2;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

pub type PalletBalances = pallet_balances::Pallet<Runtime>;

parameter_types! {
    pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const CouncilAccount: AccountId32 = AccountId32::from([1u8; 32]);
    pub const TreasuryAccount: AccountId32 = AccountId32::from([2u8; 32]);
}

impl Config for Runtime {
    type Event = Event;
    type MultiCurrency = Tokens;
    type NativeCurrency = AdaptedBasicCurrency;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type SweepOrigin = EnsureSignedBy<CouncilAccount, AccountId>;
    type OnDust = crate::TransferDust<Runtime, DustAccount>;
}

pub type NativeCurrency = Currency<Runtime, GetNativeCurrencyId>;
pub type AdaptedBasicCurrency = BasicCurrencyAdapter<Runtime, PalletBalances, i64, u64>;

pub type Block = frame_system::mocking::MockBlock<Runtime>;
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
        Currencies: currencies::{Pallet, Call, Event<T>},
    }
);

pub fn alice() -> AccountId {
    AccountId::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap()
}

pub fn bob() -> AccountId {
    AccountId::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap()
}

pub fn eva() -> AccountId {
    AccountId::from_ss58check("5ECW6Y5ukt1DH2zrbHXnXwCiUM1MjeoGHNhvdHLRRjDY8pf5").unwrap()
}

pub const ID_1: LockIdentifier = *b"1       ";

pub struct ExtBuilder {
    balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self { balances: vec![] }
    }
}

impl ExtBuilder {
    pub fn balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub fn one_hundred_for_alice_n_bob(self) -> Self {
        self.balances(vec![
            (alice(), NATIVE_CURRENCY_ID, 100),
            (bob(), NATIVE_CURRENCY_ID, 100),
            (alice(), X_TOKEN_ID, 100),
            (bob(), X_TOKEN_ID, 100),
        ])
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        pallet_balances::GenesisConfig::<Runtime> {
            balances: self
                .balances
                .clone()
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id == NATIVE_CURRENCY_ID)
                .map(|(account_id, _, initial_balance)| (account_id, initial_balance))
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        orml_tokens::GenesisConfig::<Runtime> {
            balances: self
                .balances
                .into_iter()
                .filter(|(_, currency_id, _)| *currency_id != NATIVE_CURRENCY_ID)
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
