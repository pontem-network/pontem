// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! This pallet enables Move Virtual Machine developed by Facebook's Diem team in your Substrate based chain.
//! The pallet allows to execute Move Smart contracts, utilizing Move VM adopted for WASM Runtime.
//! You can find Move VM in the following repository - https://github.com/pontem-network/sp-move-vm

//! Move VM has two types of smart contracts: scripts and modules.
//! Modules can be stored in the chain storage, under user account (address) and module name, modules code that can be resused multiplay times.
//! Also, modules store user data, similar to EVM smart contracts.
//! Scripts executed only one time, during transaction execution, it can access modules, or not (depends on script goal).
//! Scripts used as entry point, because indeed scripts launch code execution in Move VM.
//! Read more about scripts and modules in Pontem documentation - https://docs.pontem.network/03.-move-vm/modules

//! All provided extrinsics functions require to configure a gas limit, similar to EVM.
//! Current pallet contains following extrinsics to iterate with Move VM:
//! execute(tx_bc: Vec<u8>, gas_limit: u64) - execute Move script with bytecode `tx_bc`.
//! publish_module(module_bc: Vec<u8>, gas_limit: u64) - publish Move module with bytecode `module_bc`.
//! publish_package(package: Vec<u8>, gas_limit: u64) - publish package (a set of Move modules) from binary `package`.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate log;

#[cfg(feature = "runtime-benchmarks")]
extern crate serde_alt as serde;
#[cfg(feature = "runtime-benchmarks")]
extern crate bcs_alt as bcs;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;
pub mod addr;
pub mod balance;
pub mod event;
pub mod gas;
pub mod mvm;
pub mod result;
pub mod storage;
pub mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    // Clippy didn't love sp- macros
    #![allow(clippy::unused_unit)]

    use super::*;
    use super::storage::MoveVmStorage;
    use gas::GasWeightMapping;
    use event::*;
    use groupsign::utils::ensure_groupsign;
    use mvm::*;
    use weights::WeightInfo;

    use crate::storage::boxed::VmStorageBoxAdapter as StorageAdapter;
    use crate::balance::boxed::BalancesAdapter;

    use core::convert::TryInto;
    use core::convert::TryFrom;

    use sp_std::{vec::Vec, prelude::*, default::Default};
    use frame_system::pallet_prelude::*;
    use frame_support as support;
    use support::dispatch::fmt::Debug;
    use support::pallet_prelude::*;
    use support::traits::{UnixTime, tokens::fungibles};
    use support::PalletId;
    use support::dispatch::DispatchResultWithPostInfo;
    use sp_runtime::traits::{UniqueSaturatedInto, AccountIdConversion};
    use parity_scale_codec::{FullCodec, FullEncode};

    use move_vm::{Vm, StateAccess};
    use move_vm::mvm::Mvm;
    use move_vm::io::context::ExecutionContext;
    use move_vm::types::Gas;
    use move_vm::types::ModuleTx;
    use move_vm::types::Transaction;
    use move_vm::types::VmResult;
    use move_vm::types::ModulePackage;

    use move_core_types::account_address::AccountAddress;
    use move_core_types::language_storage::CORE_CODE_ADDRESS;

    #[cfg(not(feature = "std"))]
    extern crate alloc;
    #[cfg(not(feature = "std"))]
    use alloc::format;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + timestamp::Config + balances::Config + groupsign::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Gas to weight convertion settings.
        type GasWeightMapping: gas::GasWeightMapping;

        /// The AccountId that can perform a standard library update or deploy module under 0x address.
        type UpdateOrigin: EnsureOrigin<Self::Origin>;

        /// Describes weights for Move VM extrinsics.
        type WeightInfo: WeightInfo;

        /// The treasury's pallet id, used for deriving its sovereign account ID.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Currency id indetifier.
        type CurrencyId: FullCodec
            + Eq
            + PartialEq
            + Copy
            + MaybeSerializeDeserialize
            + scale_info::TypeInfo
            + Debug
            + TryFrom<Vec<u8>>
            + Default;

        // Multicurrency pallet.
        type Currencies: orml_traits::MultiCurrency<
                <Self as frame_system::Config>::AccountId,
                CurrencyId = Self::CurrencyId,
            > + fungibles::Inspect<
                <Self as frame_system::Config>::AccountId,
                AssetId = Self::CurrencyId,
            >;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    /// Move VM storage. Map with already encoded key-values pairs:
    /// - Key: `AccessPath` as bytes
    /// - Value: `WriteSet` as bytes
    // TODO: Experimentally try hasher [Identity][frame_support::Identity]
    //                           because key are already encoded - hashes.
    #[pallet::storage]
    pub type VMStorage<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event provided by Move VM
        /// [guid, typetag, message]
        Event(
            Vec<u8>, // Event guid
            Vec<u8>, // Event typetag, encoded as String
            Vec<u8>, // Actual event payload
        ),

        /// Event about successful move-module publishing
        /// [account]
        ModulePublished(T::AccountId),

        /// Event about successful move-package published
        /// [account]
        PackagePublished(T::AccountId),
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        OriginFor<T>: Into<Result<groupsign::Origin<T>, OriginFor<T>>>,
    {
        /// Execute Move script.
        ///
        /// User can send his Move script (compiled using 'dove tx' command) for execution by Move VM.
        /// The gas limit should be provided.
        #[pallet::weight(
            <T as Config>::WeightInfo::execute().saturating_add(
                T::GasWeightMapping::gas_to_weight(*gas_limit)
            )
        )]
        pub fn execute(
            origin: OriginFor<T>,
            tx_bc: Vec<u8>,
            gas_limit: u64,
        ) -> DispatchResultWithPostInfo {
            let groupsign_origin = ensure_groupsign(origin.clone());

            let (signers, root) = match groupsign_origin {
                // TODO: determine sudoer by groupsign signers
                Ok(groupsign_signers) => (groupsign_signers.signers, false),
                Err(_) => match T::UpdateOrigin::ensure_origin(origin.clone()) {
                    Ok(_) => (vec![], true),
                    Err(_) => (vec![ensure_signed(origin)?], false),
                },
            };

            let vm_result = Self::raw_execute_script(&signers, tx_bc, gas_limit, root, false)?;

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(vm_result)?;
            Ok(result)
        }

        /// Publish Move module.
        ///
        /// User can publish his Move module under his address.
        /// The gas limit should be provided.
        #[pallet::weight(
            <T as Config>::WeightInfo::publish_module().saturating_add(
                T::GasWeightMapping::gas_to_weight(*gas_limit)
            )
        )]
        pub fn publish_module(
            origin: OriginFor<T>,
            module_bc: Vec<u8>,
            gas_limit: u64,
        ) -> DispatchResultWithPostInfo {
            // Allows to update Standard Library if root.
            let (sender, signer) = Self::ensure_and_convert(origin)?;
            debug!("executing `publish module` with signed {:?}", sender);

            // Publish module.
            let vm_result = Self::raw_publish_module(&signer, module_bc, gas_limit, false)?;

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(vm_result)?;

            // Emit an event:
            Self::deposit_event(Event::ModulePublished(signer));

            Ok(result)
        }

        /// Publish module package (could be generated using 'dove build -b'), e.g.: several modules in one transaction.
        ///
        /// Deploy several modules in one transaction. Could be called by root in case needs to update Standard Library.
        /// Read more about Standard Library - https://docs.pontem.network/03.-move-vm/stdlib
        /// The gas limit should be provided.
        /// TODO: maybe we should replace it with publish_package, yet i'm currently not sure, as user anyway paying for transaction bytes.
        #[pallet::weight(
            <T as Config>::WeightInfo::publish_module().saturating_add(
                T::GasWeightMapping::gas_to_weight(*gas_limit)
            )
        )]
        pub fn publish_package(
            origin: OriginFor<T>,
            package: Vec<u8>,
            gas_limit: u64,
        ) -> DispatchResultWithPostInfo {
            // Allows to update Standard Library if root.
            let (sender, signer) = Self::ensure_and_convert(origin)?;
            debug!("executing `publish package` with signed {:?}", sender);

            let vm = Self::get_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;

            let package = {
                ModulePackage::try_from(&package[..])
                    .map_err(|_| Error::<T>::TransactionValidationError)?
                    .into_tx(sender)
            };

            let vm_result = vm.publish_module_package(gas, package, false);

            // produce result with spended gas:
            let result = result::from_vm_result::<T>(vm_result)?;

            // Emit an event:
            Self::deposit_event(Event::PackagePublished(signer));

            Ok(result)
        }
    }

    /// Genesis configuration.
    ///
    /// Allows to configure Move VM in the genesis block.
    /// Accepts Standard Library modules list and initialize state by calling initialize function with arguments.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _phantom: std::marker::PhantomData<T>,
        /// Move Standard library bytes.
        pub move_stdlib: Vec<u8>,
        /// Pontem Framework library bytes.
        pub pont_framework: Vec<u8>,
        /// Module name for genesis init.
        pub init_module: Vec<u8>,
        // Init function name.
        pub init_func: Vec<u8>,
        // Init function arguments.
        pub init_args: Vec<Vec<u8>>,
    }

    /// Default genesis configuration.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                _phantom: Default::default(),
                move_stdlib: vec![],
                pont_framework: vec![],
                init_module: vec![],
                init_func: vec![],
                init_args: vec![],
            }
        }
    }

    /// Initialize Move VM during genesis block.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            let mut stdlib_package = ModulePackage::try_from(&self.move_stdlib[..])
                .expect("Failed to parse move stdlib");

            let pont_framework_package = ModulePackage::try_from(&self.pont_framework[..])
                .expect("Failed to parse pont framework lib");

            stdlib_package.join(pont_framework_package);

            let genesis_config = move_vm::genesis::build_genesis_config(
                stdlib_package.into_tx(CORE_CODE_ADDRESS),
                Some(move_vm::genesis::InitFuncConfig {
                    module: self.init_module.clone(),
                    func: self.init_func.clone(),
                    args: self.init_args.clone(),
                }),
            );

            move_vm::genesis::init_storage(Pallet::<T>::move_vm_storage(), genesis_config)
                .expect("Unable to initialize storage");
        }
    }

    /// Clearing Move VM cache once block processed.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
    // TODO: make it configurable:  where <T as Config>::ClearMvmCachePolicy = ...
    {
        fn on_finalize(_: BlockNumberFor<T>) {
            if Self::is_move_vm_used() {
                if let Some(vm) = Self::get_move_vm_cell().get() {
                    vm.clear();
                    Self::set_move_vm_clean();
                    trace!("VM cache cleared on finalize block");
                }
            }
            // Otherwise we are not requesting VM.
        }
    }

    /// Get VM methods unification.
    impl<T: Config> Pallet<T> {
        fn get_vm() -> Result<&'static VmWrapperTy, Error<T>> {
            let vm = Self::try_get_or_create_move_vm()?;
            Ok(vm)
        }
    }

    /// Move VM allows us to configure Gas Price, but we use constant for gas price, as we follow general Substrate approach with weight and tips.
    const GAS_UNIT_PRICE: u64 = 1;

    impl<T: Config> Pallet<T> {
        #![allow(clippy::useless_conversion)]
        /// Returns gas limit object requires for execute/publish functions.
        fn get_move_gas_limit(gas_limit: u64) -> Result<Gas, Error<T>> {
            Gas::new(gas_limit, GAS_UNIT_PRICE).map_err(|_| Error::InvalidGasAmountMaxValue)
        }

        /// Get pallet account id.
        pub fn get_account_id() -> T::AccountId {
            T::PalletId::get().into_account()
        }

        /// Execute Move VM script with provided signers, script byte code, gas limit, and dry run configuration.
        /// In case of dry run nothing would be written to storage after execution (required mostly by RPC calls, e.g. estimate gas etc).
        /// Multiple signers supported by utilizing multisig pallet.
        pub fn raw_execute_script(
            signers: &[T::AccountId],
            tx_bc: Vec<u8>,
            gas_limit: u64,
            root_signed: bool,
            dry_run: bool,
        ) -> Result<VmResult, Error<T>>
        where
            <T as timestamp::Config>::Moment: UniqueSaturatedInto<u64>,
            // T::BlockNumber: BaseArithmetic,
            // T::BlockNumber: UniqueSaturatedInto<u64>,
            T::BlockNumber: TryInto<u64>,
        {
            // TODO: some minimum gas for processing transaction from bytes?
            let transaction = Transaction::try_from(&tx_bc[..])
                .map_err(|_| Error::<T>::TransactionValidationError)?;

            // TODO: think about in future change fn-parameters
            //       from `signers` & `root_signed` to just `origin: OriginFor`
            //       and so ensure root here instead of trust `root_signed`.
            ensure!(
                root_signed == transaction.has_root_signer(),
                Error::<T>::TransactionIsNotAllowedError
            );

            let vm = Self::get_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;

            let tx = {
                let signers = if transaction.signers_count() == 0 {
                    &[]
                } else {
                    signers
                };

                // Is it really necessary? VM will throw an error if it caused.
                if transaction.signers_count() as usize != signers.len() {
                    error!(
                        "Transaction signers num isn't eq signers: {} != {}",
                        transaction.signers_count(),
                        signers.len()
                    );
                    return Err(Error::<T>::TransactionSignersNumError.into());
                }

                let signers = signers
                    .iter()
                    .map(addr::account_to_bytes)
                    .map(AccountAddress::new)
                    .collect();

                transaction
                    .into_script(signers)
                    .map_err(|_| Error::<T>::TransactionValidationError)?
            };

            let ctx = {
                let height = frame_system::Pallet::<T>::block_number()
                    .try_into()
                    .map_err(|_| Error::<T>::NumConversionError)?
                    as u64;

                // Because if we call now().as_millis() during genesis it returns error.
                // And stdlib initializing during genesis.
                let time = match height {
                    0 => 0,
                    _ => <timestamp::Pallet<T> as UnixTime>::now().as_millis() as u64,
                };

                ExecutionContext::new(time, height)
            };

            let res = vm.execute_script(gas, ctx, tx, dry_run);
            debug!("execution result: {:?}", res);

            Ok(res)
        }

        /// Ensures origin is root or signed and returns account id with associated move-address.
        /// Returns error if si not signed or root/sudo.
        pub fn ensure_and_convert(
            origin: OriginFor<T>,
        ) -> Result<(AccountAddress, T::AccountId), Error<T>> {
            // Allows to update Standard Library if root.
            match T::UpdateOrigin::ensure_origin(origin.clone()) {
                Ok(_) => {
                    let signer = addr::address_to_account(&CORE_CODE_ADDRESS)
                        .map_err(|_| Error::<T>::AccountAddressConversionError)?;
                    Ok((CORE_CODE_ADDRESS, signer))
                }
                Err(_) => {
                    let signer =
                        ensure_signed(origin).map_err(|_| Error::<T>::InvalidSignature)?;
                    Ok((addr::account_to_account_address(&signer), signer))
                }
            }
        }

        /// Publish Move module script with provided account, module bytecode, gas limit, and dry run configuration.
        /// In case of dry run nothing would be written to storage after execution (required mostly by RPC calls, e.g. estimate gas etc).
        pub fn raw_publish_module(
            account: &T::AccountId,
            module_bc: Vec<u8>,
            gas_limit: u64,
            dry_run: bool,
        ) -> Result<VmResult, Error<T>> {
            let vm = Self::get_vm()?;
            let gas = Self::get_move_gas_limit(gas_limit)?;

            let tx = {
                let sender = addr::account_to_bytes(account);
                debug!("converted sender: {:?}", sender);
                ModuleTx::new(module_bc, AccountAddress::new(sender))
            };

            let res = vm.publish_module(gas, tx, dry_run);
            debug!("publication result: {:?}", res);

            Ok(res)
        }

        pub fn get_module_abi(module_id: &[u8]) -> Result<Option<Vec<u8>>, Vec<u8>> {
            let vm = Self::get_vm()
                .map_err::<Vec<u8>, _>(|e| format!("error while getting vm {:?}", e).into())?;
            vm.get_module_abi(module_id)
                .map_err(|e| format!("error in get_module_abi: {:?}", e).into())
        }

        pub fn get_module(module_id: &[u8]) -> Result<Option<Vec<u8>>, Vec<u8>> {
            let vm = Self::get_vm()
                .map_err::<Vec<u8>, _>(|e| format!("error while getting vm {:?}", e).into())?;
            vm.get_module(module_id)
                .map_err(|e| format!("error in get_module: {:?}", e).into())
        }

        pub fn get_resource(
            account: &T::AccountId,
            tag: &[u8],
        ) -> Result<Option<Vec<u8>>, Vec<u8>> {
            let vm = Self::get_vm()
                .map_err::<Vec<u8>, _>(|e| format!("error while getting vm {:?}", e).into())?;
            vm.get_resource(&AccountAddress::new(addr::account_to_bytes(account)), tag)
                .map_err(|e| format!("error in get_resource: {:?}", e).into())
        }
    }

    /// Get storage adapter ready for the VM.
    impl<T: Config, K, V> super::storage::MoveVmStorage<T, K, V> for Pallet<T>
    where
        K: FullEncode,
        V: FullCodec,
    {
        type VmStorage = VMStorage<T>;
    }

    impl<T: Config> event::DepositMoveEvent for Pallet<T> {
        fn deposit_move_event(e: MoveEventArguments) {
            debug!("MoveVM Event: {:?} {:?} {:?}", e.guid, e.ty_tag, e.message);

            // Emit an event:
            // TODO: dispatch up the error by TryInto. Error is almost impossible but who knows..
            Self::deposit_event(e.try_into().expect("Cannot back-convert address"));
        }
    }

    /// Implement traits allows to create Move VM.
    ///
    /// Supports both static (created at launch of chain), and dynamic one (usually we use regulated one);.
    impl<T: Config> mvm::TryCreateMoveVm<T> for Pallet<T> {
        type Vm = Mvm<StorageAdapter, event::DefaultEventHandler, BalancesAdapter>;
        type Error = Error<T>;

        /// Try to create Move VM instance.
        ///
        /// If successful it returns a VM instance, otherwise error.
        fn try_create_move_vm() -> Result<Self::Vm, Self::Error> {
            trace!("MoveVM created");
            Mvm::new(
                Self::move_vm_storage().into(),
                Self::create_move_event_handler(),
                balance::BalancesAdapter::<
                    <T as frame_system::Config>::AccountId,
                    T::Currencies,
                    T::CurrencyId,
                >::new(T::PalletId::get())
                .into(),
            )
            .map_err(|err| {
                error!("{}", err);
                Error::InvalidVMConfig
            })
        }
    }

    impl<T: Config> GetStaticMoveVmCell for Pallet<T> {
        type Vm = VmWrapper<<Self as mvm::TryCreateMoveVm<T>>::Vm>;

        #[inline(never)]
        fn get_move_vm_cell() -> &'static OnceCell<Self::Vm> {
            static VM: OnceCell<VmWrapperTy> = OnceCell::new();
            &VM
        }
    }

    impl<T: Config> TryGetStaticMoveVm for Pallet<T> {
        type Vm = <Self as GetStaticMoveVmCell>::Vm;
        type Error = Error<T>;

        fn try_get_or_create_move_vm() -> Result<&'static Self::Vm, Self::Error> {
            Self::set_move_vm_used();
            Self::get_move_vm_cell()
                .get_or_try_init(|| Self::try_create_move_vm_static().map(Into::into))
        }
    }

    /// Usage marker for the VM.
    impl<T: Config> MoveVmUsed for Pallet<T> {}

    /// Errors that occur during Move VM execution.
    /// Based on initial Move VM errors, but adopted for Substrate.
    #[pallet::error]
    pub enum Error<T> {
        /// Internal: numeric convertion error, overflow
        NumConversionError,

        /// Failed to read or decode VM configuration
        InvalidVMConfig,
        /// `max_gas_amount` value must be in the range from 0 to `u64::MAX / 1000`.
        /// Causes for invalid gas configuration.
        InvalidGasAmountMaxValue,
        /// Script senders should not be empty
        ScriptValidationError,
        /// Transaction deserialization & validation error
        TransactionValidationError,
        /// Transaction signers num isn't eq signers
        TransactionSignersNumError,
        /// AccountAddress conversion error.
        AccountAddressConversionError,
        /// Transaction is not allowed.
        TransactionIsNotAllowedError,

        /// Unknown validation status
        UnknownValidationStatus,
        /// The transaction has a bad signature
        InvalidSignature,
        /// Bad account authentication key
        InvalidAuthKey,
        /// Sequence number is too old
        SequenceNumberTooOld,
        /// Sequence number is too new
        SequenceNumberTooNew,
        /// The sequence number is too large and would overflow if the transaction were executed
        SequenceNumberTooBig,
        /// Insufficient balance to pay minimum transaction fee
        InsufficientBalanceForTransactionFee,
        /// The transaction has expired
        TransactionExpired,
        /// The sending account does not exist
        SendingAccountDoesNotExist,
        /// This write set transaction was rejected because it did not meet the requirements for one.
        RejectedWriteSet,
        /// This write set transaction cannot be applied to the current state.
        InvalidWriteSet,
        /// Length of program field in raw transaction exceeded max length
        ExceededMaxTransactionSize,
        /// This script is not in our allowlist of scripts.
        UnknownScript,
        /// Transaction is trying to publish a new module.
        UnknownModule,
        /// Max gas units submitted with transaction exceeds max gas units bound in VM
        MaxGasUnitsExceedsMaxGasUnitsBound,
        /// Max gas units submitted with transaction not enough to cover the intrinsic cost of the transaction.
        MaxGasUnitsBelowMinTransactionGasUnits,
        /// Gas unit price submitted with transaction is below minimum gas price set in the VM.
        GasUnitPriceBelowMinBound,
        /// Gas unit price submitted with the transaction is above the maximum gas price set in the VM.
        GasUnitPriceAboveMaxBound,
        /// Gas specifier submitted is either malformed (not a valid identifier), or does not refer to an accepted gas specifier
        InvalidGasSpecifier,
        /// The sending account is frozen
        SendingAccountFrozen,
        /// Unable to deserialize the account blob
        UnableToDeserializeAccount,
        /// The currency info was unable to be found
        CurrencyInfoDoesNotExist,
        /// The account sender doesn't have permissions to publish modules
        InvalidModulePublisher,
        /// The sending account has no role
        NoAccountRole,
        /// The transaction's chain_id does not match the one published on-chain
        BadChainId,
        /// Unknown verification error
        UnknownVerificationError,
        /// Index out of bounds
        IndexOutOfBounds,
        /// Invalid signature token
        InvalidSignatureToken,
        /// Recursive struct definition
        RecursiveStructDefinition,
        /// Invalid resource field
        InvalidResourceField,
        /// Invalid fall through
        InvalidFallThrough,
        /// Negative stack size within block
        NegativeStackSizeWithinBlock,
        /// Invalid main function signature
        InvalidMainFunctionSignature,
        /// Duplicate element
        DuplicateElement,
        /// Invalid module handle
        InvalidModuleHandle,
        /// Unimplemented handle
        UnimplementedHandle,
        /// Lookup failed
        LookupFailed,
        /// Type mismatch
        TypeMismatch,
        /// Missing dependency
        MissingDependency,
        /// Pop resource error
        PopResourceError,
        /// Br type mismatch
        BrTypeMismatchError,
        /// Abort type mismatch error
        AbortTypeMismatchError,

        /// Stloc type mismatch error
        StlocTypeMismatchError,
        /// Stloc unsafe to destroy error
        StlocUnsafeToDestroyError,
        /// Unsafe ret local or resource still borrowed
        UnsafeRetLocalOrResourceStillBorrowed,
        /// Ret type mismatch error
        RetTypeMismatchError,
        /// Ret borrowed mutable reference error
        RetBorrowedMutableReferenceError,
        /// Freezeref type mismatch error
        FreezerefTypeMismatchError,
        /// Freezeref exists mutable borrow error
        FreezerefExistsMutableBorrowError,
        /// Borrowfield type mismatch error
        BorrowfieldTypeMismatchError,
        /// Borrowfield bad field error
        BorrowfieldBadFieldError,
        /// Borrowfield exists mutable borrow error
        BorrowfieldExistsMutableBorrowError,
        /// Copyloc unavailable error
        CopylocUnavailableError,
        /// Copyloc resource error
        CopylocResourceError,
        /// Copyloc exists borrow error
        CopylocExistsBorrowError,
        /// Moveloc unavailable error
        MovelocUnavailableError,
        /// Moveloc exists borrow error
        MovelocExistsBorrowError,
        /// Borrowloc reference error
        BorrowlocReferenceError,
        /// Borrowloc unavailable error
        BorrowlocUnavailableError,
        /// Borrowloc exists borrow error
        BorrowlocExistsBorrowError,
        /// Call type mismatch error
        CallTypeMismatchError,
        /// Call borrowed mutable reference error
        CallBorrowedMutableReferenceError,
        /// Pack type mismatch error
        PackTypeMismatchError,
        /// Unpack type mismatch error
        UnpackTypeMismatchError,
        /// Readref type mismatch error
        ReadrefTypeMismatchError,
        /// Readref resource error
        ReadrefResourceError,
        /// Readref exists mutable borrow error
        ReadrefExistsMutableBorrowError,
        /// Writeref type mismatch error
        WriterefTypeMismatchError,
        /// Writeref resource error
        WriterefResourceError,
        /// Writeref exists borrow error
        WriterefExistsBorrowError,
        /// Writeref no mutable reference error
        WriterefNoMutableReferenceError,
        /// Integer op type mismatch error
        IntegerOpTypeMismatchError,
        /// Boolean op type mismatch error
        BooleanOpTypeMismatchError,
        /// Equality op type mismatch error
        EqualityOpTypeMismatchError,
        /// Exists resource type mismatch error
        ExistsResourceTypeMismatchError,
        /// Borrowglobal type mismatch error
        BorrowglobalTypeMismatchError,
        /// Borrowglobal no resource error
        BorrowglobalNoResourceError,
        /// Movefrom Type mismatch error
        MovefromTypeMismatchError,
        /// Movefrom no resource error
        MovefromNoResourceError,
        /// Moveto type mismatch error
        MovetoTypeMismatchError,
        /// Moveto no resource error
        MovetoNoResourceError,
        /// The self address of a module the transaction is publishing is not the sender address
        ModuleAddressDoesNotMatchSender,
        /// The module does not have any module handles. Each module or script must have at least one module handle.
        NoModuleHandles,
        /// Positive stack size at block end
        PositiveStackSizeAtBlockEnd,
        /// Missing acquires resource annotation error
        MissingAcquiresResourceAnnotationError,
        /// Extraneous acquires resource annotation error
        ExtraneousAcquiresResourceAnnotationError,
        /// Duplicate acquires resource annotation error
        DuplicateAcquiresResourceAnnotationError,
        /// Invalid acquires resource annotation error
        InvalidAcquiresResourceAnnotationError,
        /// Global reference error
        GlobalReferenceError,
        /// Constraint kind mismatch
        ConstraintKindMismatch,
        /// Number of type arguments mismatch
        NumberOfTypeArgumentsMismatch,
        /// Loop in instantiation graph
        LoopInInstantiationGraph,
        /// Zero sized struct.
        ZeroSizedStruct,
        /// Linker error
        LinkerError,
        /// Invalid constant type
        InvalidConstantType,
        /// Malformed constant data
        MalformedConstantData,
        /// Empty code unit
        EmptyCodeUnit,
        /// Invalid loop split
        InvalidLoopSplit,
        /// Invalid loop break
        InvalidLoopBreak,
        /// Invalid loop continue
        InvalidLoopContinue,
        /// Unsafe fet unused resources
        UnsafeRetUnusedResources,
        /// Too many locals
        TooManyLocals,
        /// Generic member opcode mismatch
        GenericMemberOpcodeMismatch,
        /// Function resolution failure
        FunctionResolutionFailure,
        /// Invalid operation in script
        InvalidOperationInScript,
        /// The sender is trying to publish a module named `M`, but the sender's account already contains a module with this name.
        DuplicateModuleName,
        /// Unknown invariant violation error
        UnknownInvariantViolationError,
        /// Empty value stack
        EmptyValueStack,
        /// Pc overflow
        PcOverflow,
        /// Verification error
        VerificationError,
        /// Storage error
        StorageError,
        /// Internal type error
        InternalTypeError,
        /// Event key mismatch
        EventKeyMismatch,
        /// Unreachable
        Unreachable,
        /// vm startup failure
        VmStartupFailure,
        /// Unexpected error from known move function
        UnexpectedErrorFromKnownMoveFunction,
        /// Verifier invariant violation
        VerifierInvariantViolation,
        /// Unexpected verifier error
        UnexpectedVerifierError,
        /// Unexpected deserialization error
        UnexpectedDeserializationError,
        /// Failed to serialize write set changes
        FailedToSerializeWriteSetChanges,
        /// Failed to deserialize resource
        FailedToDeserializeResource,
        /// Failed to resolve type due to linking being broken after verification
        TypeResolutionFailure,
        /// Unknown binary error
        UnknownBinaryError,
        /// Malformed
        Malformed,
        /// Bad magic
        BadMagic,
        /// Unknown version
        UnknownVersion,
        /// Unknown table type
        UnknownTableType,
        /// Unknown signature type
        UnknownSignatureType,
        /// Unknown serialized type
        UnknownSerializedType,
        /// Unknown opcode
        UnknownOpcode,
        /// BadHeader table
        BadHeaderTable,
        /// Unexpected signature type
        UnexpectedSignatureType,
        /// Duplicate table
        DuplicateTable,
        /// Unknown nominal resource
        UnknownNominalResource,
        /// Unknown kind
        UnknownKind,
        /// Unknown native struct flag
        UnknownNativeStructFlag,
        /// Bad U64
        BadU64,
        /// Bad U128
        BadU128,
        /// Value serialization error
        ValueSerializationError,
        /// Value deserialization error
        ValueDeserializationError,
        /// Code deserialization error
        CodeDeserializationError,
        /// Unknown runtime status
        UnknownRuntimeStatus,
        /// Out of gas
        OutOfGas,
        /// We tried to access a resource that does not exist under the account.
        ResourceDoesNotExist,
        /// We tried to create a resource under an account where that resource already exists.
        ResourceAlreadyExists,
        /// Missing data
        MissingData,
        /// Data format error
        DataFormatError,
        /// Aborted
        Aborted,
        /// Arithmetic error
        ArithmeticError,
        /// Execution stack overflow
        ExecutionStackOverflow,
        /// Call stack overflow
        CallStackOverflow,
        /// Vm max type depth reached
        VmMaxTypeDepthReached,
        /// Vm max value depth reached
        VmMaxValueDepthReached,
        /// Unknown status.
        UnknownStatus,

        // Documentation_missing
        BadTransactionFeeCurrency,
        // Documentation_missing
        FeatureUnderGating,
        // Documentation_missing
        FieldMissingTypeAbility,
        // Documentation_missing
        PopWithoutDropAbility,
        // Documentation_missing
        CopylocWithoutCopyAbility,
        // Documentation_missing
        ReadrefWithoutCopyAbility,
        // Documentation_missing
        WriterefWithoutDropAbility,
        // Documentation_missing
        ExistsWithoutKeyAbilityOrBadArgument,
        // Documentation_missing
        BorrowglobalWithoutKeyAbility,
        // Documentation_missing
        MovefromWithoutKeyAbility,
        // Documentation_missing
        MovetoWithoutKeyAbility,
        // Documentation_missing
        MissingAcquiresAnnotation,
        // Documentation_missing
        ExtraneousAcquiresAnnotation,
        // Documentation_missing
        DuplicateAcquiresAnnotation,
        // Documentation_missing
        InvalidAcquiresAnnotation,
        // Documentation_missing
        ConstraintNotSatisfied,
        // Documentation_missing
        UnsafeRetUnusedValuesWithoutDrop,
        // Documentation_missing
        BackwardIncompatibleModuleUpdate,
        // Documentation_missing
        CyclicModuleDependency,
        // Documentation_missing
        NumberOfArgumentsMismatch,
        // Documentation_missing
        InvalidParamTypeForDeserialization,
        // Documentation_missing
        FailedToDeserializeArgument,
        // Documentation_missing
        NumberOfSignerArgumentsMismatch,
        // Documentation_missing
        CalledScriptVisibleFromNonScriptVisible,
        // Documentation_missing
        ExecuteScriptFunctionCalledOnNonScriptVisible,
        // Documentation_missing
        InvalidFriendDeclWithSelf,
        // Documentation_missing
        InvalidFriendDeclWithModulesOutsideAccountAddress,
        // Documentation_missing
        InvalidFriendDeclWithModulesInDependencies,
        // Documentation_missing
        CyclicModuleFriendship,
        // Documentation_missing
        UnknownAbility,
        // Documentation_missing
        InvalidFlagBits,
        // Wrong secondary keys addresses count
        SecondaryKeysAddressesCountMismatch,
        // List of signers contain duplicates
        SignersContainDuplicates,
        // Invalid sequence nonce
        SequenceNonceInvalid,
        // Invalid phantom type param position
        InvalidPhantomTypeParamPosition,
        // Documentation_missing
        VecUpdateExistsMutableBorrowError,
        // Documentation_missing
        VecBorrowElementExistsMutableBorrowError,
        // Found duplicate of native function
        DuplicateNativeFunction,
    }
}

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;
#[cfg(feature = "std")]
impl<T: Config> GenesisConfig<T> {
    /// Direct implementation of `GenesisBuild::build_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
        <Self as GenesisBuild<T>>::build_storage(self)
    }

    /// Direct implementation of `GenesisBuild::assimilate_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
        <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
    }
}
