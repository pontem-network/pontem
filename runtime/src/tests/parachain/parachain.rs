use crate::tests::parachain::*;
use crate::tests::parachain::mock_runtime::CurrencyId as MockCurrencyId;
use orml_xtokens::Error;
use xcm_emulator::{Junction, TestExt, Concrete};
use frame_support::traits::Currency;
use polkadot_parachain::primitives::{AccountIdConversion};
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_runtime::AccountId32;
use test_log::test;

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
            Box::new((Here, 100 * dollar(CurrencyId::KSM)).into()),
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
        let _ = RelayBalances::deposit_creating(&para_a_account(), 2 * dollar(CurrencyId::KSM));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(Accounts::ALICE.account()).into(),
            CurrencyId::KSM,
            dollar(CurrencyId::KSM) as _,
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
            1999 * dollar(CurrencyId::KSM) as u64
        );
    });

    Relay::execute_with(|| {
        assert_eq!(
            RelayBalances::free_balance(&para_a_account()),
            dollar(CurrencyId::KSM)
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
            CurrencyId::PONT,
            &Accounts::ALICE.account(),
            1_000 * PONT
        ));
        assert_noop!(
            ParaAXTokens::transfer(
                Some(Accounts::ALICE.account()).into(),
                CurrencyId::PONT,
                500 * PONT,
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
                30 * PONT,
            ),
            Error::<crate::Runtime>::XcmExecutionFailed
        );

        assert_eq!(
            ParaATokens::free_balance(CurrencyId::PONT, &Accounts::ALICE.account()),
            1_000 * PONT
        );
    });
}

#[test]
fn send_relay_chain_asset_to_sibling() {
    TestNet::reset();

    Relay::execute_with(|| {
        let _ = RelayBalances::deposit_creating(&para_a_account(), 4 * dollar(CurrencyId::KSM));
    });

    ParaA::execute_with(|| {
        assert_ok!(ParaAXTokens::transfer(
            Some(Accounts::ALICE.account()).into(),
            CurrencyId::KSM,
            3 * dollar(CurrencyId::KSM) as u64,
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
            1997 * dollar(CurrencyId::KSM) as u64
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
            CurrencyId::PONT,
            500 * PONT,
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
            ParaBTokens::free_balance(MockCurrencyId::PONT, &Accounts::BOB.account()),
            500 * PONT - 4
        );
    });

    // Send back to Parachain A.
    ParaB::execute_with(|| {
        assert_ok!(ParaBXTokens::transfer(
            Some(Accounts::BOB.account()).into(),
            MockCurrencyId::PONT,
            500 * PONT - 4,
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
            ParaBTokens::free_balance(MockCurrencyId::PONT, &Accounts::BOB.account()),
            0
        );
    });

    ParaA::execute_with(|| {
        assert_eq!(
            ParaABalances::free_balance(&Accounts::BOB.account()),
            500 * PONT - 8
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
                        id: Concrete(GeneralKey("PONT".into()).into()),
                        fun: (100 * PONT as u128).into(),
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
                        id: (Parent, Parachain(2000), GeneralKey("PONT".into())).into(),
                        fun: (100 * PONT as u128).into(),
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
                        id: (Parent, Parachain(2002), GeneralKey("PONT".into())).into(),
                        fun: (100 * PONT as u128).into(),
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
