use crate::tests::parachain::*;
use crate::tests::parachain::mock_runtime::CurrencyId as MockCurrencyId;
use orml_xtokens::Error;
use xcm_emulator::{Junction, TestExt};
use frame_support::traits::Currency;
use cumulus_primitives_core::ParaId;
use polkadot_parachain::primitives::{AccountIdConversion};
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_runtime::AccountId32;
use primitives::currency::CurrencyId;

fn para_a_account() -> AccountId32 {
    ParaId::from(1).into_account()
}

#[test]
fn transfer_from_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
            kusama_runtime::Origin::signed(ALICE.into()),
            Box::new(Parachain(1).into().into()),
            Box::new(
                Junction::AccountId32 {
                    id: BOB.into(),
                    network: NetworkId::Any
                }
                .into()
                .into()
            ),
            Box::new((Here, dollar(CurrencyId::KSM) * 100).into()),
            0
        ));
    });

    ParaA::execute_with(|| {
        assert_eq!(
            Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)),
            99999999893333
        );
    });
}

#[test]
fn send_relay_chain_asset_to_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_a_account(), 2 * dollar(CurrencyId::KSM));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::KSM,
            dollar(CurrencyId::KSM) as _,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X1(Junction::AccountId32 {
                        network: NetworkId::Any,
                        id: BOB.into(),
                    })
                )
                .into()
            ),
            3_000_000_000,
        ));
        assert_eq!(
            ParaATokens::free_balance(CurrencyId::KSM, &ALICE),
            1999 * dollar(CurrencyId::KSM) as u64
        );
    });

    Relay::execute_with(|| {
        assert_eq!(
            RelayBalances::free_balance(&para_a_account()),
            dollar(CurrencyId::KSM)
        );
        assert_eq!(
            RelayBalances::free_balance(&BOB),
            dollar(CurrencyId::KSM) - 79999995
        );
    });
}

#[test]
fn cannot_lost_fund_on_send_failed() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaATokens::deposit(
            CurrencyId::PONT,
            &ALICE,
            CurrencyId::PONT * 1_000
        ));
        assert_noop!(
            ParaAXTokens::transfer(
                Some(ALICE).into(),
                CurrencyId::PONT,
                CurrencyId::PONT * 500,
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(100),
                            Junction::AccountId32 {
                                network: NetworkId::Kusama,
                                id: BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                CurrencyId::PONT * 30,
            ),
            Error::<crate::Runtime>::XcmExecutionFailed
        );

        assert_eq!(
            ParaATokens::free_balance(CurrencyId::PONT, &ALICE),
            CurrencyId::PONT * 1_000
        );
    });
}

#[test]
fn send_relay_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_a_account(), 3 * dollar(CurrencyId::KSM));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::KSM,
            3 * dollar(CurrencyId::KSM) as u64,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(2),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: BOB.into(),
                        }
                    )
                )
                .into()
            ),
            3_000_000,
        ));
        assert_eq!(
            ParaATokens::free_balance(CurrencyId::KSM, &ALICE),
            1997 * dollar(CurrencyId::KSM) as u64
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaBTokens::free_balance(MockCurrencyId::KSM, &BOB),
            3 * dollar(CurrencyId::KSM) as u64 - 160000
        );
    });
}

#[test]
fn send_self_parachain_asset_to_sibling() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(ALICE).into(),
            CurrencyId::PONT,
            CurrencyId::PONT * 500,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(2),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: BOB.into(),
                        }
                    )
                )
                .into()
            ),
            4_000_000,
        ));

        assert_eq!(ParaABalances::free_balance(&ALICE), 15000000000000);
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaBTokens::free_balance(MockCurrencyId::PONT, &BOB),
            CurrencyId::PONT * 500 - 4
        );
    });

    // Send back to Parachain A.
    ParaB::execute_with(|| {
        assert_ok!(ParaBXTokens::transfer(
            Some(BOB).into(),
            MockCurrencyId::PONT,
            CurrencyId::PONT * 500 - 4,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(1),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: BOB.into(),
                        }
                    )
                )
                .into()
            ),
            4_000_000,
        ));

        assert_eq!(ParaBTokens::free_balance(MockCurrencyId::PONT, &BOB), 0);
    });

    ParaA::execute_with(|| {
        assert_eq!(
            ParaABalances::free_balance(&BOB),
            CurrencyId::PONT * 500 - 8
        );
    });
}

#[test]
fn transfer_no_reserve_assets_fails() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_noop!(
            ParaAXTokens::transfer_multiasset(
                Some(ALICE).into(),
                Box::new(
                    MultiAsset {
                        id: xcm_emulator::Concrete(GeneralKey("PONT".into()).into()),
                        fun: CurrencyId::PONT.times(100).into().into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(2),
                            Junction::AccountId32 {
                                network: NetworkId::Any,
                                id: BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                CurrencyId::PONT * 50,
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
            ParaAXTokens::transfer_multiasset(
                Some(ALICE).into(),
                Box::new(
                    MultiAsset {
                        id: (Parent, Parachain(1), GeneralKey("PONT".into())).into(),
                        fun: CurrencyId::PONT.times(100).into().into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(1),
                            Junction::AccountId32 {
                                network: NetworkId::Any,
                                id: BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                CurrencyId::PONT * 50,
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
            ParaAXTokens::transfer_multiasset(
                Some(ALICE).into(),
                Box::new(
                    MultiAsset {
                        id: (Parent, Parachain(1), GeneralKey("PONT".into())).into(),
                        fun: CurrencyId::PONT.times(100).into().into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        0,
                        Junctions::X1(Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: BOB.into(),
                        })
                    )
                    .into()
                ),
                CurrencyId::PONT * 50,
            ),
            Error::<crate::Runtime>::InvalidDest
        );
    });
}
