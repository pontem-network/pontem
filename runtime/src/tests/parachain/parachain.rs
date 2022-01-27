use crate::tests::parachain::*;
use crate::tests::parachain::mock_runtime::CurrencyId as MockCurrencyId;
use orml_xtokens::Error;
use xcm_emulator::{Junction, TestExt};
use frame_support::traits::Currency;
use polkadot_parachain::primitives::{AccountIdConversion};
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_runtime::AccountId32;
use test_log::test;
use primitives::currency::{CurrencyId, NATIVE_SYM};

fn para_a_account() -> AccountId32 {
    ParaId::from(2000).into_account()
}

#[test]
fn transfer_from_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
            kusama_runtime::Origin::signed(Accounts::ALICE.account().into()),
            Box::new(X1(Parachain(2000)).into().into()),
            Box::new(
                X1(Junction::AccountId32 {
                    network: Any,
                    id: Accounts::BOB.into()
                })
                .into()
                .into()
            ),
            Box::new((Here, CurrencyId::KSM.times(100) as u128).into()),
            0,
        ));
    });

    ParaA::execute_with(|| {
        assert_eq!(
            ParaATokens::free_balance(CurrencyId::KSM, &Accounts::BOB.account()),
            99999999893333
        );
    });
}

#[test]
fn send_relay_chain_asset_to_relay_chain() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ =
            RelayBalances::deposit_creating(&para_a_account(), CurrencyId::KSM.times(2).into());
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(Accounts::ALICE.account()).into(),
            CurrencyId::KSM,
            CurrencyId::KSM.times(1),
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X1(Junction::AccountId32 {
                        network: NetworkId::Any,
                        id: Accounts::BOB.into(),
                    })
                )
                .into()
            ),
            4_000_000_000,
        ));
        assert_eq!(
            ParaATokens::free_balance(CurrencyId::KSM, &Accounts::ALICE.account()),
            CurrencyId::KSM.times(1999)
        );
    });

    Relay::execute_with(|| {
        assert_eq!(
            RelayBalances::free_balance(&para_a_account()),
            CurrencyId::KSM.times(1) as u128
        );
        assert_eq!(
            RelayBalances::free_balance(&Accounts::BOB.account()),
            999893333340
        );
    });
}

#[test]
fn cannot_lost_fund_on_send_failed() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaATokens::deposit(
            CurrencyId::NATIVE,
            &Accounts::ALICE.account(),
            CurrencyId::NATIVE * 1_000
        ));
        assert_noop!(
            ParaAXTokens::transfer(
                Some(Accounts::ALICE.account()).into(),
                CurrencyId::NATIVE,
                CurrencyId::NATIVE * 500,
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(100),
                            Junction::AccountId32 {
                                network: NetworkId::Kusama,
                                id: Accounts::BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                CurrencyId::NATIVE * 30,
            ),
            Error::<crate::Runtime>::XcmExecutionFailed
        );

        assert_eq!(
            ParaATokens::free_balance(CurrencyId::NATIVE, &Accounts::ALICE.account()),
            CurrencyId::NATIVE * 1_000
        );
    });
}

#[test]
fn send_relay_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ =
            RelayBalances::deposit_creating(&para_a_account(), CurrencyId::KSM.times(4).into());
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(Accounts::ALICE.account()).into(),
            CurrencyId::KSM,
            CurrencyId::KSM.times(3),
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(2001),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: Accounts::BOB.into(),
                        }
                    )
                )
                .into()
            ),
            4_000_000_000,
        ));
        assert_eq!(
            ParaATokens::free_balance(CurrencyId::KSM, &Accounts::ALICE.account()),
            CurrencyId::KSM.times(1997)
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaBTokens::free_balance(MockCurrencyId::KSM, &Accounts::BOB.account()),
            2999893226673
        );
    });
}

#[test]
fn send_self_parachain_asset_to_sibling() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(Accounts::ALICE.account()).into(),
            CurrencyId::NATIVE,
            CurrencyId::NATIVE * 500,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(2001),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: Accounts::BOB.into(),
                        }
                    )
                )
                .into()
            ),
            4_000_000_000,
        ));

        assert_eq!(
            ParaABalances::free_balance(&Accounts::ALICE.account()),
            15000000000000
        );
    });

    ParaB::execute_with(|| {
        assert_eq!(
            ParaBTokens::free_balance(MockCurrencyId::NATIVE, &Accounts::BOB.account()),
            CurrencyId::NATIVE * 500 - 4
        );
    });

    // Send back to Parachain A.
    ParaB::execute_with(|| {
        assert_ok!(ParaBXTokens::transfer(
            Some(Accounts::BOB.account()).into(),
            MockCurrencyId::NATIVE,
            CurrencyId::NATIVE * 500 - 4,
            Box::new(
                MultiLocation::new(
                    1,
                    Junctions::X2(
                        Junction::Parachain(2000),
                        Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: Accounts::BOB.into(),
                        }
                    )
                )
                .into()
            ),
            4_000_000_000,
        ));

        assert_eq!(
            ParaBTokens::free_balance(MockCurrencyId::NATIVE, &Accounts::BOB.account()),
            0
        );
    });

    ParaA::execute_with(|| {
        assert_eq!(
            ParaABalances::free_balance(&Accounts::BOB.account()),
            CurrencyId::NATIVE * 500 - 8
        );
    });
}

#[test]
fn transfer_no_reserve_assets_fails() {
    TestNet::reset();

    ParaA::execute_with(|| {
        assert_noop!(
            ParaAXTokens::transfer_multiasset(
                Some(Accounts::ALICE.account()).into(),
                Box::new(
                    MultiAsset {
                        id: xcm_emulator::Concrete(GeneralKey(NATIVE_SYM.to_vec()).into()),
                        fun: (CurrencyId::NATIVE.times(100) as u128).into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(2001),
                            Junction::AccountId32 {
                                network: NetworkId::Any,
                                id: Accounts::BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                4_000_000_000,
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
                Some(Accounts::ALICE.account()).into(),
                Box::new(
                    MultiAsset {
                        id: (Parent, Parachain(2000), GeneralKey(NATIVE_SYM.to_vec())).into(),
                        fun: (CurrencyId::NATIVE.times(100) as u128).into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        1,
                        Junctions::X2(
                            Junction::Parachain(2000),
                            Junction::AccountId32 {
                                network: NetworkId::Any,
                                id: Accounts::BOB.into(),
                            }
                        )
                    )
                    .into()
                ),
                4_000_000_000,
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
                Some(Accounts::ALICE.account()).into(),
                Box::new(
                    MultiAsset {
                        id: (Parent, Parachain(2000), GeneralKey(NATIVE_SYM.to_vec())).into(),
                        fun: (CurrencyId::NATIVE.times(100) as u128).into(),
                    }
                    .into()
                ),
                Box::new(
                    MultiLocation::new(
                        0,
                        Junctions::X1(Junction::AccountId32 {
                            network: NetworkId::Any,
                            id: Accounts::BOB.into(),
                        })
                    )
                    .into()
                ),
                4_000_000_000,
            ),
            Error::<crate::Runtime>::InvalidDest
        );
    });
}
