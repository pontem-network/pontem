use crate::mock::*;
use crate::*;
use orml_xtokens::Error;
use xcm_emulator::TestExt;
use frame_support::traits::Currency;
use cumulus_primitives_core::ParaId;
use polkadot_parachain::primitives::{Sibling, AccountIdConversion};
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_runtime::AccountId32;
use xcm::v0::{Junction, NetworkId};

fn para_a_account() -> AccountId32 {
    ParaId::from(1).into_account()
}

fn sibling_b_account() -> AccountId32 {
    use sp_runtime::traits::AccountIdConversion;
    Sibling::from(2).into_account()
}

#[test]
fn send_relay_chain_asset_to_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_a_account(), 2 * dollar(CurrencyId::Ksm));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::Ksm,
            dollar(CurrencyId::Ksm) as _,
            (
                Parent,
                Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: BOB.into(),
                },
            )
                .into(),
            3_000_000_000,
        ));
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Ksm, &ALICE),
            1999 * dollar(CurrencyId::Ksm) as u64
        );
    });

    Relay::execute_with(|| {
        assert_eq!(
            RelayBalances::free_balance(&para_a_account()),
            dollar(CurrencyId::Ksm)
        );
        assert_eq!(
            RelayBalances::free_balance(&BOB),
            dollar(CurrencyId::Ksm) - 79999995
        );
    });
}

#[test]
fn cannot_lost_fund_on_send_failed() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaTokens::deposit(CurrencyId::Pont, &ALICE, 1_000 * PONT));
        assert_noop!(
            ParaXTokens::transfer(
                Some(ALICE).into(),
                CurrencyId::Pont,
                500 * PONT,
                (
                    Parent,
                    Parachain(100),
                    Junction::AccountId32 {
                        network: NetworkId::Kusama,
                        id: BOB.into(),
                    },
                )
                    .into(),
                30 * PONT,
            ),
            Error::<crate::Runtime>::XcmExecutionFailed
        );

        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &ALICE),
            1_000 * PONT
        );
    });
}

#[test]
fn send_relay_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_a_account(), 3 * dollar(CurrencyId::Ksm));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::Ksm,
            3 * dollar(CurrencyId::Ksm) as u64,
            (
                Parent,
                Parachain(2),
                Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: BOB.into(),
                },
            )
                .into(),
            3_000_000,
        ));
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Ksm, &ALICE),
            1997 * dollar(CurrencyId::Ksm) as u64
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Ksm, &BOB),
            3 * dollar(CurrencyId::Ksm) as u64 - 160000
        );
    });
}

#[test]
fn send_sibling_asset_to_sibling() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaTokens::deposit(CurrencyId::Pont, &ALICE, 1_000 * PONT));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::Pont,
            500 * PONT,
            (
                Parent,
                Parachain(2),
                Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: BOB.into(),
                },
            )
                .into(),
            3_000_000,
        ));

        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &ALICE),
            500 * PONT
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &BOB),
            500 * PONT - 3
        );
    });
}

#[test]
fn send_self_parachain_asset_to_sibling() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaTokens::deposit(CurrencyId::Pont, &ALICE, 1_000 * PONT));

        assert_ok!(ParaXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::Pont,
            500 * PONT,
            (
                Parent,
                Parachain(2),
                Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: BOB.into(),
                },
            )
                .into(),
            3_000_000,
        ));

        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &ALICE),
            500 * PONT
        );
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &sibling_b_account()),
            500 * PONT
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaTokens::free_balance(CurrencyId::Pont, &BOB),
            500 * PONT - 3
        );
    });
}

#[test]
fn transfer_no_reserve_assets_fails() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_noop!(
            ParaXTokens::transfer_multiasset(
                Some(ALICE).into(),
                MultiAsset::ConcreteFungible {
                    id: GeneralKey("PONT".into()).into(),
                    amount: 100 * PONT as u128,
                },
                (
                    Parent,
                    Parachain(2),
                    Junction::AccountId32 {
                        network: NetworkId::Any,
                        id: BOB.into()
                    }
                )
                    .into(),
                50 * PONT,
            ),
            Error::<crate::Runtime>::AssetHasNoReserve
        );
    });
}

#[test]
fn transfer_to_self_chain_fails() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_noop!(
            ParaXTokens::transfer_multiasset(
                Some(ALICE).into(),
                MultiAsset::ConcreteFungible {
                    id: (Parent, Parachain(1), GeneralKey("PONT".into())).into(),
                    amount: 100 * PONT as u128,
                },
                (
                    Parent,
                    Parachain(1),
                    Junction::AccountId32 {
                        network: NetworkId::Any,
                        id: BOB.into()
                    }
                )
                    .into(),
                50 * PONT,
            ),
            Error::<crate::Runtime>::NotCrossChainTransfer
        );
    });
}

#[test]
fn transfer_to_invalid_dest_fails() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_noop!(
            ParaXTokens::transfer_multiasset(
                Some(ALICE).into(),
                MultiAsset::ConcreteFungible {
                    id: (Parent, Parachain(1), GeneralKey("PONT".into())).into(),
                    amount: 100 * PONT as u128,
                },
                (Junction::AccountId32 {
                    network: NetworkId::Any,
                    id: BOB.into()
                })
                .into(),
                50 * PONT,
            ),
            Error::<crate::Runtime>::InvalidDest
        );
    });
}
