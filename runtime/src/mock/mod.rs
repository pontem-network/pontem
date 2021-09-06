use crate::*;

mod relay;

use sp_runtime::AccountId32;
use frame_support::sp_io::TestExternalities;
use frame_support::traits::GenesisBuild;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};
use cumulus_primitives_core::{ParaId, GetChannelInfo, ChannelStatus};

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([1u8; 32]);

decl_test_parachain! {
    pub struct ParaA {
        Runtime = crate::Runtime,
        new_ext = para_ext(1),
    }
}

decl_test_parachain! {
    pub struct ParaB {
        Runtime = crate::Runtime,
        new_ext = para_ext(2),
    }
}

decl_test_relay_chain! {
    pub struct Relay {
        Runtime = relay::Runtime,
        XcmConfig = relay::XcmConfig,
        new_ext = relay_ext(),
    }
}

decl_test_network! {
    pub struct TestNet {
        relay_chain = Relay,
        parachains = vec![
            (1, ParaA),
            (2, ParaB),
        ],
    }
}

pub type RelayBalances = pallet_balances::Pallet<relay::Runtime>;
pub type ParaTokens = orml_tokens::Pallet<crate::Runtime>;
pub type ParaXTokens = orml_xtokens::Pallet<crate::Runtime>;

pub fn para_ext(para_id: u32) -> TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    let parachain_info_config = parachain_info::GenesisConfig {
        parachain_id: para_id.into(),
    };
    <parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
        &parachain_info_config,
        &mut t,
    )
    .unwrap();

    orml_tokens::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, CurrencyId::Dot, 100 * 1000_000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub struct ChannelInfo;

impl GetChannelInfo for ChannelInfo {
    fn get_channel_status(_id: ParaId) -> ChannelStatus {
        ChannelStatus::Ready(10, 10)
    }
    fn get_channel_max(_id: ParaId) -> Option<usize> {
        Some(usize::max_value())
    }
}

pub fn relay_ext() -> TestExternalities {
    use relay::{Runtime, System};

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, 100 * 1000_000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
