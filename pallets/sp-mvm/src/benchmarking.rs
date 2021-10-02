#![cfg(feature = "runtime-benchmarks")]
// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0 

//! Benchmarking setup for Move VM pallet.

// How to use:
// 1. Build node with feature `runtime-benchmarks`
// 2. Run `./target/release/pontem benchmark --dev -lsp_mvm=trace --pallet=sp_mvm --execution=wasm --wasm-execution=compiled --extrinsic='*' --steps=20 --repeat=10 --output=./target/sp-bench/`

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{CORE_CODE_ADDRESS, ModuleId, StructTag};
use move_vm::io::key::AccessKey;
use sp_std::prelude::*;

use crate::benchmarking::store::container;

use super::*;
#[allow(unused)]
use super::Pallet as Mvm;

// Pontem benchmarks.
// Deploying standard library modules, just modules, runs scripts with different arguments.
benchmarks! {

    // Needs to be fixed in multisig. Not yet sure how, needs more deconstruction.
    where_clause { where Result<pallet_multisig::Origin<T>, <T as frame_system::Config>::Origin>: From<<T as frame_system::Config>::Origin> }

    publish_empty_module {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let module = include_bytes!("../tests/benchmark_assets/artifacts/modules/2_Empty.mv").to_vec();
    }: publish_module(RawOrigin::Signed(caller), module, 100_000_000)
    verify {
        assert!(VMStorage::<T>::contains_key(module_access("Empty")));
    }
    publish_many_deps_module {
        let s in 0 .. 100;
        for (name, module) in stdlib() {
            VMStorage::<T>::insert(module_access_core(name), module);
        }
        let caller: T::AccountId = whitelisted_caller();
        let module = include_bytes!("../tests/benchmark_assets/artifacts/modules/52_StdImport.mv").to_vec();
    }: publish_module(RawOrigin::Signed(caller), module, 100_000_000)
    verify {
        assert!(VMStorage::<T>::contains_key(module_access("StdImport")));
    }
    publish_s_module {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let module = include_bytes!("../tests/benchmark_assets/artifacts/modules/6_S.mv").to_vec();
    }: publish_module(RawOrigin::Signed(caller), module, 100_000_000)
    verify {
        assert!(VMStorage::<T>::contains_key(module_access("S")));
    }
    publish_m_module {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let module = include_bytes!("../tests/benchmark_assets/artifacts/modules/5_M.mv").to_vec();
    }: publish_module(RawOrigin::Signed(caller), module, 100_000_000)
    verify {
        assert!(VMStorage::<T>::contains_key(module_access("M")));
    }
    publish_l_module {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let module = include_bytes!("../tests/benchmark_assets/artifacts/modules/4_L.mv").to_vec();
    }: publish_module(RawOrigin::Signed(caller), module, 100_000_000)
    verify {
        assert!(VMStorage::<T>::contains_key(module_access("L")));
    }
    execute_many_params {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/many_params.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 500_000)
    verify {
        // no-op
    }
    execute_store {
        let s in 0 .. 100;
         for (name, module) in stdlib() {
            VMStorage::<T>::insert(module_access_core(name), module);
        }
        VMStorage::<T>::insert(module_access_core("Store"), include_bytes!("../tests/benchmark_assets/artifacts/modules/1_Store.mv").to_vec());
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/store.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 500_000)
    verify {

        let tag = StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Store").unwrap(),
            name: Identifier::new("Container").unwrap(),
            type_params: vec![]
         };

        let ak = AccessKey::from((&AccountAddress::from_hex_literal("0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291").unwrap(), &tag));

        assert!(VMStorage::<T>::contains_key(ak.as_ref()));
    }
    execute_load {
        let s in 0 .. 100;
         for (name, module) in stdlib() {
            VMStorage::<T>::insert(module_access_core(name), module);
        }

        let tag = StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new("Store").unwrap(),
            name: Identifier::new("Container").unwrap(),
            type_params: vec![]
         };

        let ak = AccessKey::from((&CORE_CODE_ADDRESS, &tag));

        VMStorage::<T>::insert(ak.as_ref().to_vec(), bcs::to_bytes(&container()).unwrap());

        VMStorage::<T>::insert(module_access_core("Store"), include_bytes!("../tests/benchmark_assets/artifacts/modules/1_Store.mv").to_vec());
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/load.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 500_000)
    verify {
    }
    execute_store_event {
        let s in 0 .. 100;
         for (name, module) in stdlib() {
            VMStorage::<T>::insert(module_access_core(name), module);
        }
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/store_events.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 500_000)
    verify {
    }
    execute_vec_input {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/vector_input.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 500_000)
    verify {
    }
    execute_loop {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let tx = include_bytes!("../tests/benchmark_assets/artifacts/transactions/lp.mvt").to_vec();
    }: execute(RawOrigin::Signed(caller), tx, 100_000_000)
    verify {
    }
}

impl_benchmark_test_suite!(Mvm, crate::mock::new_test_ext(), crate::mock::Test,);

pub fn module_access_core(name: &str) -> Vec<u8> {
    ModuleId::new(CORE_CODE_ADDRESS, Identifier::new(name).unwrap()).access_vector()
}

pub fn module_access(name: &str) -> Vec<u8> {
    ModuleId::new(
        AccountAddress::from_hex_literal(
            "0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291",
        )
        .unwrap(),
        Identifier::new(name).unwrap(),
    )
    .access_vector()
}

pub fn stdlib() -> Vec<(&'static str, Vec<u8>)> {
    vec![
        (
            "Signer",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/0_Signer.mv").to_vec()
        ),
        (
            "Option",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/10_Option.mv").to_vec()
        ),
        (
            "ValidatorConfig",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/11_ValidatorConfig.mv").to_vec()
        ),
        (
            "AccountLimits",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/12_AccountLimits.mv").to_vec()
        ),
        (
            "VASP",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/13_VASP.mv").to_vec()
        ),
        (
            "FixedPoint32",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/14_FixedPoint32.mv").to_vec()
        ),
        (
            "BCS",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/15_BCS.mv").to_vec()
        ),
        (
            "Event",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/16_Event.mv").to_vec()
        ),
        (
            "DiemConfig",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/17_DiemConfig.mv").to_vec()
        ),
        (
            "RegisteredCurrencies",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/18_RegisteredCurrencies.mv").to_vec()
        ),
        (
            "NativeCurrencies",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/19_NativeCurrencies.mv").to_vec()
        ),
        (
            "U256",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/1_U256.mv").to_vec()
        ),
        (
            "Diem",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/20_Diem.mv").to_vec()
        ),
        (
            "PONT",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/21_PONT.mv").to_vec()
        ),
        (
            "DualAttestation",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/22_DualAttestation.mv").to_vec()
        ),
        (
            "DesignatedDealer",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/24_DesignatedDealer.mv").to_vec()
        ),
        (
            "AccountFreezing",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/25_AccountFreezing.mv").to_vec()
        ),
        (
            "DiemAccount",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/26_DiemAccount.mv").to_vec()
        ),
        (
            "Hash",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/27_Hash.mv").to_vec()
        ),
        (
            "Authenticator",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/28_Authenticator.mv").to_vec()
        ),
        (
            "SharedEd25519PublicKey",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/29_SharedEd25519PublicKey.mv").to_vec()
        ),
        (
            "RecoveryAddress",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/30_RecoveryAddress.mv").to_vec()
        ),
        (
            "Errors",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/2_Errors.mv").to_vec()
        ),
        (
            "AccountAdministrationScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/31_AccountAdministrationScripts.mv").to_vec()
        ),
        (
            "AccountCreationScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/32_AccountCreationScripts.mv").to_vec()
        ),
        (
            "ChainId",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/33_ChainId.mv").to_vec()
        ),
        (
            "DiemBlock",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/34_DiemBlock.mv").to_vec()
        ),
        (
            "DiemConsensusConfig",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/35_DiemConsensusConfig.mv").to_vec()
        ),
        (
            "DiemSystem",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/36_DiemSystem.mv").to_vec()
        ),
        (
            "DiemTransactionPublishingOption",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/37_DiemTransactionPublishingOption.mv").to_vec()
        ),
        (
            "DiemVMConfig",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/38_DiemVMConfig.mv").to_vec()
        ),
        (
            "DiemVersion",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/39_DiemVersion.mv").to_vec()
        ),
        (
            "TransactionFee",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/40_TransactionFee.mv").to_vec()
        ),
        (
            "CoreAddresses",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/3_CoreAddresses.mv").to_vec()
        ),
        (
            "Genesis",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/41_Genesis.mv").to_vec()
        ),
        (
            "PaymentScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/42_PaymentScripts.mv").to_vec()
        ),
        (
            "SystemAdministrationScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/43_SystemAdministrationScripts.mv").to_vec()
        ),
        (
            "TreasuryComplianceScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/44_TreasuryComplianceScripts.mv").to_vec()
        ),
        (
            "ValidatorAdministrationScripts",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/45_ValidatorAdministrationScripts.mv").to_vec()
        ),
        (
            "DiemTimestamp",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/4_DiemTimestamp.mv").to_vec()
        ),
        (
            "SlidingNonce",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/5_SlidingNonce.mv").to_vec()
        ),
        (
            "Signature",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/6_Signature.mv").to_vec()
        ),
        (
            "Vector",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/7_Vector.mv").to_vec()
        ),
        (
            "Roles",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/8_Roles.mv").to_vec()
        ),
        (
            "ValidatorOperatorConfig",
            include_bytes!("../tests/benchmark_assets/stdlib/artifacts/modules/9_ValidatorOperatorConfig.mv").to_vec()
        ),
    ]
}

mod store {
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use serde::Serialize;
    use sp_std::prelude::*;

    #[derive(Serialize)]
    #[serde(crate = "serde_alt")]
    pub struct Container {
        inner_1: Inner,
        inner_2: Inner,
        inner_3: Inner,
        inner_4: Inner,
    }

    #[derive(Serialize)]
    #[serde(crate = "serde_alt")]
    pub struct Inner {
        val: bool,
        val_1: u128,
        val_2: Vec<u8>,
        val_3: u64,
    }

    pub fn container() -> Container {
        Container {
            inner_1: Inner {
                val: true,
                val_1: 1000000000,
                val_2: CORE_CODE_ADDRESS.to_vec(),
                val_3: 13,
            },
            inner_2: Inner {
                val: false,
                val_1: 13,
                val_2: CORE_CODE_ADDRESS.to_vec(),
                val_3: 0,
            },
            inner_3: Inner {
                val: false,
                val_1: 42,
                val_2: CORE_CODE_ADDRESS.to_vec(),
                val_3: 0,
            },
            inner_4: Inner {
                val: false,
                val_1: 0,
                val_2: CORE_CODE_ADDRESS.to_vec(),
                val_3: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_composite::{ExtBuilder, Test};
    use frame_support::assert_ok;

    #[test]
    fn publish_std() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_std::<Test>());
        });
    }

    #[test]
    fn publish_empty_module() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_empty_module::<Test>());
        });
    }

    #[test]
    fn publish_many_deps_module() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_many_deps_module::<Test>());
        });
    }

    #[test]
    fn publish_s_module() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_s_module::<Test>());
        });
    }

    #[test]
    fn publish_m_module() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_m_module::<Test>());
        });
    }

    #[test]
    fn publish_l_module() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_publish_l_module::<Test>());
        });
    }

    #[test]
    fn execute_many_params() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_execute_many_params::<Test>());
        });
    }

    #[test]
    fn execute_store() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_execute_store::<Test>());
        });
    }

    #[test]
    fn execute_load() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_execute_load::<Test>());
        });
    }

    #[test]
    fn execute_store_event() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_execute_store_event::<Test>());
        });
    }

    #[test]
    fn execute_vec_input() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_execute_vec_input::<Test>());
        });
    }

    #[test]
    fn execute_execute_loop() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_eexecute_loop::<Test>());
        });
    }
}
