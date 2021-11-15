use crate::*;

pub mod mock_runtime;

use sp_runtime::AccountId32;
use frame_support::sp_io::TestExternalities;
use frame_support::traits::GenesisBuild;
use xcm_emulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};
use polkadot_primitives::v1::{MAX_CODE_SIZE, MAX_POV_SIZE};
use polkadot_runtime_parachains::configuration::HostConfiguration;
pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([1u8; 32]);

decl_test_parachain! {
    pub struct ParaA {
        Runtime = crate::Runtime,
        Origin = Origin,
        new_ext = para_ext(1),
    }
}

decl_test_parachain! {
    pub struct ParaB {
        Runtime = mock_runtime::Runtime,
        Origin = mock_runtime::Origin,
        new_ext = mock_para_ext(2),
    }
}

decl_test_relay_chain! {
    pub struct Relay {
        Runtime = kusama_runtime::Runtime,
        XcmConfig = kusama_runtime::XcmConfig,
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

pub type RelayBalances = pallet_balances::Pallet<kusama_runtime::Runtime>;

pub type ParaATokens = orml_tokens::Pallet<crate::Runtime>;
pub type ParaBTokens = orml_tokens::Pallet<mock_runtime::Runtime>;

pub type ParaABalances = pallet_balances::Pallet<crate::Runtime>;

pub type ParaAXTokens = orml_xtokens::Pallet<crate::Runtime>;
pub type ParaBXTokens = orml_xtokens::Pallet<mock_runtime::Runtime>;

pub fn mock_para_ext(para_id: u32) -> TestExternalities {
    use mock_runtime::Runtime;

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

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, 2000 * PONT), (BOB, 2000 * PONT)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
        &pallet_xcm::GenesisConfig {
            safe_xcm_version: Some(2),
        },
        &mut t,
    )
    .unwrap();

    orml_tokens::GenesisConfig::<Runtime> {
        balances: vec![(
            ALICE,
            mock_runtime::CurrencyId::KSM,
            2000 * dollar(CurrencyId::KSM) as u64,
        )],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

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

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, 2000 * PONT)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
        &pallet_xcm::GenesisConfig {
            safe_xcm_version: Some(2),
        },
        &mut t,
    )
    .unwrap();

    orml_tokens::GenesisConfig::<Runtime> {
        balances: vec![(
            ALICE,
            CurrencyId::KSM,
            2000 * dollar(CurrencyId::KSM) as u64,
        )],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

fn default_parachains_host_configuration() -> HostConfiguration<BlockNumber> {
    HostConfiguration {
        validation_upgrade_frequency: 1u32,
        validation_upgrade_delay: 1,
        code_retention_period: 1200,
        max_code_size: MAX_CODE_SIZE,
        max_pov_size: MAX_POV_SIZE,
        max_head_data_size: 32 * 1024,
        group_rotation_frequency: 20,
        chain_availability_period: 4,
        thread_availability_period: 4,
        max_upward_queue_count: 8,
        max_upward_queue_size: 1024 * 1024,
        max_downward_message_size: 1024,
        ump_service_total_weight: 4 * 1_000_000_000,
        max_upward_message_size: 1024 * 1024,
        max_upward_message_num_per_candidate: 5,
        hrmp_sender_deposit: 0,
        hrmp_recipient_deposit: 0,
        hrmp_channel_max_capacity: 8,
        hrmp_channel_max_total_size: 8 * 1024,
        hrmp_max_parachain_inbound_channels: 4,
        hrmp_max_parathread_inbound_channels: 4,
        hrmp_channel_max_message_size: 1024 * 1024,
        hrmp_max_parachain_outbound_channels: 4,
        hrmp_max_parathread_outbound_channels: 4,
        hrmp_max_message_num_per_candidate: 5,
        dispute_period: 6,
        no_show_slots: 2,
        n_delay_tranches: 25,
        needed_approvals: 2,
        relay_vrf_modulo_samples: 2,
        zeroth_delay_tranche_width: 0,
        ..Default::default()
    }
}

pub fn relay_ext() -> TestExternalities {
    use kusama_runtime::{Runtime, System};

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, 2000 * dollar(CurrencyId::KSM))],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
        config: default_parachains_host_configuration(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    <pallet_xcm::GenesisConfig as GenesisBuild<Runtime>>::assimilate_storage(
        &pallet_xcm::GenesisConfig {
            safe_xcm_version: Some(2),
        },
        &mut t,
    )
    .unwrap();

    let mut ext = TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
