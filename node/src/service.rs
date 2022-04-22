use cumulus_client_cli::CollatorOptions;
// use cumulus_relay_chain_interface::build_relay_chain_interface;
// use cumulus_relay_chain_local::build_relay_chain_interface;
// use polkadot_collator::build_relay_chain_interface;
// use cumulus_relay_chain_inprocess_interface::build_relay_chain_interface;
use cumulus_relay_chain_inprocess_interface::build_inprocess_relay_chain;
use cumulus_relay_chain_interface::{RelayChainError, RelayChainInterface, RelayChainResult};
use crate::cli::Sealing;
use cumulus_primitives_parachain_inherent::{MockValidationDataInherentDataProvider, MockXcmConfig};
use futures::StreamExt;
use sp_core::H256;
use cumulus_client_service::{
    prepare_node_config, start_collator, start_full_node, StartCollatorParams,
    StartFullNodeParams,
};
use std::time::Duration;
use sc_consensus_manual_seal::{run_manual_seal, EngineCommand, ManualSealParams};
use cumulus_primitives_core::ParaId;
use pontem_runtime::RuntimeApi;
use sp_blockchain::HeaderBackend;
use sc_service::{Configuration, PartialComponents, Role, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use std::sync::Arc;
use substrate_prometheus_endpoint::Registry;
use sp_keystore::SyncCryptoStorePtr;
use cumulus_client_consensus_common::ParachainConsensus;
use cumulus_client_network::BlockAnnounceValidator;
use nimbus_primitives::NimbusId;
use nimbus_consensus::{
    BuildNimbusConsensusParams, NimbusConsensus, NimbusManualSealConsensusDataProvider,
};
use primitives::Block;
use sc_executor::NativeElseWasmExecutor;
use sp_runtime::Percent;

type FullBackend = TFullBackend<Block>;
type FullClient =
    TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ParachainRuntimeExecutor>>;
type MaybeSelectChain = Option<sc_consensus::LongestChain<FullBackend, Block>>;

pub type HostFunctions = frame_benchmarking::benchmarking::HostFunctions;

pub struct ParachainRuntimeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainRuntimeExecutor {
    type ExtendHostFunctions = HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        pontem_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        pontem_runtime::native_version()
    }
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this function if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
pub fn new_partial(
    config: &Configuration,
    dev_service: bool,
) -> Result<
    PartialComponents<
        FullClient,
        TFullBackend<Block>,
        MaybeSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (Option<Telemetry>, Option<TelemetryWorkerHandle>),
    >,
    sc_service::Error,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = NativeElseWasmExecutor::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            &config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let import_queue = if dev_service {
        // There is a bug in this import queue where it doesn't properly check inherents:
        // https://github.com/paritytech/substrate/issues/8164
        sc_consensus_manual_seal::import_queue(
            Box::new(client.clone()),
            &task_manager.spawn_essential_handle(),
            config.prometheus_registry(),
        )
    } else {
        nimbus_consensus::import_queue(
            client.clone(),
            client.clone(),
            move |_, _| async move {
                let time = sp_timestamp::InherentDataProvider::from_system_time();

                Ok((time,))
            },
            &task_manager.spawn_essential_handle(),
            config.prometheus_registry().clone(),
            !dev_service,
        )?
    };

    let maybe_select_chain = if dev_service {
        Some(sc_consensus::LongestChain::new(backend.clone()))
    } else {
        None
    };

    let params = PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: maybe_select_chain,
        other: (telemetry, telemetry_worker_handle),
    };

    Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    id: ParaId,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)> {
    if matches!(parachain_config.role, Role::Light) {
        return Err("Light client not supported!".into());
    }

    let parachain_config = prepare_node_config(parachain_config);

    let params = new_partial(&parachain_config, false)?;
    let (mut telemetry, telemetry_worker_handle) = params.other;
    let mut task_manager = params.task_manager;

    let (relay_chain_full_node, collator_key) = build_inprocess_relay_chain(
        polkadot_config,
        &parachain_config,
        telemetry_worker_handle,
        &mut task_manager,
    )
    .map_err(|e| match e {
        // polkadot_service::Error::Sub(x) => x,
        // RelayChainError::ServiceError(x) => x,
        s => format!("{}", s).into(),
    })?;

    let client = params.client.clone();
    let backend = params.backend.clone();

    let block_announce_validator = BlockAnnounceValidator::new(relay_chain_full_node.clone(), id);

    let is_validator = parachain_config.role.is_authority();
    let force_authoring = parachain_config.force_authoring;
    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let transaction_pool = params.transaction_pool.clone();
    let import_queue = cumulus_client_service::SharedImportQueue::new(params.import_queue);

    let collator_options = CollatorOptions {
        // relay_chain_rpc_url: polkadot_config.rpc_http.map(...),
        relay_chain_rpc_url: None,
    };

    let (network, system_rpc_tx, start_network) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue: import_queue.clone(),
            warp_sync: None,
            block_announce_validator_builder: Some(Box::new(|_| {
                Box::new(block_announce_validator)
            })),
        })?;

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };

            let io = crate::rpc::create_full(deps);
            Ok(io)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_extensions_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: params.keystore_container.sync_keystore(),
        backend: backend.clone(),
        network: network.clone(),
        system_rpc_tx,
        telemetry: telemetry.as_mut(),
    })?;

    let announce_block = {
        let network = network.clone();
        Arc::new(move |hash, data| network.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    if is_validator {
        let parachain_consensus = build_consensus(
            id,
            client.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            relay_chain_full_node.clone(),
            transaction_pool,
            params.keystore_container.sync_keystore(),
            force_authoring,
        )?;
        let spawner = task_manager.spawn_handle();
        let params = StartCollatorParams {
            para_id: id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_interface: relay_chain_full_node,
            spawner,
            parachain_consensus,
            import_queue,
            collator_key,
            relay_chain_slot_duration,
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id: id,
            relay_chain_interface: relay_chain_full_node,
            relay_chain_slot_duration,
            import_queue,
            collator_options,
        };

        start_full_node(params)?;
    }

    start_network.start_network();

    Ok((task_manager, client))
}

fn build_consensus(
    id: ParaId,
    client: Arc<FullClient>,
    prometheus_registry: Option<&Registry>,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
    relay_chain_node: Arc<dyn RelayChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::FullPool<Block, FullClient>>,
    keystore: SyncCryptoStorePtr,
    force_authoring: bool,
) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error> {
    let mut proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool,
        prometheus_registry.clone(),
        telemetry.clone(),
    );
    proposer_factory.set_soft_deadline(Percent::from_percent(100));

    let create_inherent_data_providers = move |_, (relay_parent, validation_data, author_id)| {
        let relay_chain_node = relay_chain_node.clone();
        async move {
            let parachain_inherent =
                cumulus_primitives_parachain_inherent::ParachainInherentData::create_at(
                    relay_parent,
                    &relay_chain_node,
                    &validation_data,
                    id,
                )
                .await;

            let time = sp_timestamp::InherentDataProvider::from_system_time();

            let parachain_inherent = parachain_inherent.ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from(
                    "Failed to create parachain inherent",
                )
            })?;

            let author = nimbus_primitives::InherentDataProvider::<NimbusId>(author_id);

            Ok((time, parachain_inherent, author))
        }
    };

    Ok(NimbusConsensus::build(BuildNimbusConsensusParams {
        para_id: id,
        proposer_factory,
        block_import: client.clone(),
        parachain_client: client.clone(),
        keystore,
        skip_prediction: force_authoring,
        create_inherent_data_providers,
    }))
}

/// Start a normal parachain node.
pub async fn start_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    id: ParaId,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)> {
    start_node_impl(parachain_config, polkadot_config, id).await
}

pub fn new_dev(
    config: Configuration,
    author_id: nimbus_primitives::NimbusId,
    sealing: Sealing,
) -> Result<TaskManager, sc_service::Error> {
    use futures::Stream;
    let sc_service::PartialComponents {
        client,
        mut task_manager,
        import_queue,
        select_chain: maybe_select_chain,
        transaction_pool,
        other: (maybe_telemetry, _maybe_telemetry_handle),
        keystore_container,
        backend,
        ..
    } = new_partial(&config, true)?;

    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            warp_sync: None,
            block_announce_validator_builder: None,
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let prometheus_registry = config.prometheus_registry().cloned();
    let collator = config.role.is_authority();

    if collator {
        let env = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            maybe_telemetry.map(|telemetry| telemetry.handle()),
        );

        let commands_stream: Box<dyn Stream<Item = EngineCommand<H256>> + Send + Sync + Unpin> =
            match sealing {
                Sealing::Instant => Box::new(
                    transaction_pool
                        .pool()
                        .validated_pool()
                        .import_notification_stream()
                        .map(|_| EngineCommand::SealNewBlock {
                            create_empty: false,
                            finalize: false,
                            parent_hash: None,
                            sender: None,
                        }),
                ),
                Sealing::Interval(millis) => Box::new(StreamExt::map(
                    async_io::Timer::interval(std::time::Duration::from_millis(millis)),
                    |_| EngineCommand::SealNewBlock {
                        create_empty: true,
                        finalize: false,
                        parent_hash: None,
                        sender: None,
                    },
                )),
            };

        let select_chain = maybe_select_chain.expect(
            "`new_partial` builds a `LongestChainRule` when building dev service.\
				We specified the dev service when calling `new_partial`.\
				Therefore, a `LongestChainRule` is present. qed.",
        );

        let client_set_aside_for_cidp = client.clone();

        // Create channels for mocked XCM messages.
        let (_downward_xcm_sender, downward_xcm_receiver) = flume::bounded::<Vec<u8>>(100);
        let (_hrmp_xcm_sender, hrmp_xcm_receiver) = flume::bounded::<(ParaId, Vec<u8>)>(100);

        task_manager.spawn_essential_handle().spawn_blocking(
            "authorship_task",
            Some("block-authoring"),
            run_manual_seal(ManualSealParams {
                block_import: client.clone(),
                env,
                client: client.clone(),
                pool: transaction_pool.clone(),
                commands_stream,
                select_chain,
                consensus_data_provider: Some(Box::new(NimbusManualSealConsensusDataProvider {
                    keystore: keystore_container.sync_keystore(),
                    client: client.clone(),
                })),
                create_inherent_data_providers: move |block: H256, ()| {
                    let current_para_block = client_set_aside_for_cidp
                        .number(block)
                        .expect("Header lookup should succeed")
                        .expect("Header passed in as parent should be present in backend.");
                    let author_id = author_id.clone();

                    let client_for_xcm = client_set_aside_for_cidp.clone();
                    let downward_xcm_receiver = downward_xcm_receiver.clone();
                    let hrmp_xcm_receiver = hrmp_xcm_receiver.clone();

                    async move {
                        let time = sp_timestamp::InherentDataProvider::from_system_time();

                        let mocked_parachain = MockValidationDataInherentDataProvider {
                            current_para_block,
                            relay_offset: 1000,
                            relay_blocks_per_para_block: 2,
                            xcm_config: MockXcmConfig::new(
                                &*client_for_xcm,
                                block,
                                Default::default(),
                                Default::default(),
                            ),
                            raw_downward_messages: downward_xcm_receiver.drain().collect(),
                            raw_horizontal_messages: hrmp_xcm_receiver.drain().collect(),
                        };

                        let author =
                            nimbus_primitives::InherentDataProvider::<NimbusId>(author_id);

                        Ok((time, mocked_parachain, author))
                    }
                },
            }),
        );
    }

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };

            let io = crate::rpc::create_full(deps);
            Ok(io)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_extensions_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config,
        keystore: keystore_container.sync_keystore(),
        backend,
        network: network.clone(),
        system_rpc_tx,
        telemetry: None,
    })?;

    log::info!("Development Service Ready");

    network_starter.start_network();
    Ok(task_manager)
}
