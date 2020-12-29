use super::{Trait, Module};
use frame_support::dispatch::DispatchErrorWithPostInfo;
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::dispatch::PostDispatchInfo;
use frame_support::dispatch::Weight;
use frame_support::weights::Pays;
use frame_support::decl_error;
use move_vm::types::VmResult;
use move_core_types::vm_status::StatusCode;

pub fn from_status_code<T: Trait>(code: StatusCode) -> Result<(), Error<T>> {
    match code {
        StatusCode::EXECUTED => Ok(()),
        _ => Err(Error::<T>::from(code)),
    }
}

pub fn from_status_code_with_gas<T: Trait>(
    code: StatusCode,
    gas: Weight,
) -> DispatchResultWithPostInfo {
    let gas = PostDispatchInfo {
        actual_weight: Some(gas),
        pays_fee: Pays::Yes,
    };

    match code {
        StatusCode::EXECUTED => Ok(gas),
        _ => Err({
            DispatchErrorWithPostInfo {
                post_info: gas,
                error: Error::<T>::from(code).into(),
            }
        }),
    }
}

pub fn from_vm_result<T: Trait>(vm_result: VmResult) -> DispatchResultWithPostInfo {
    let gas = PostDispatchInfo {
        actual_weight: Some(vm_result.gas_used),
        pays_fee: Pays::Yes,
    };

    match vm_result.status_code {
        StatusCode::EXECUTED => Ok(gas),
        status_code => Err({
            DispatchErrorWithPostInfo {
                post_info: gas,
                error: Error::<T>::from(status_code).into(),
            }
        }),
    }
}

impl<T: Trait> From<StatusCode> for Error<T> {
    fn from(sp: StatusCode) -> Self {
        match sp {
            StatusCode::UNKNOWN_VALIDATION_STATUS => Self::UnknownValidationStatus,
            StatusCode::INVALID_SIGNATURE => Self::InvalidSignature,
            StatusCode::INVALID_AUTH_KEY => Self::InvalidAuthKey,
            StatusCode::SEQUENCE_NUMBER_TOO_OLD => Self::SequenceNumberTooOld,
            StatusCode::SEQUENCE_NUMBER_TOO_NEW => Self::SequenceNumberTooNew,
            StatusCode::INSUFFICIENT_BALANCE_FOR_TRANSACTION_FEE => {
                Self::InsufficientBalanceForTransactionFee
            }
            StatusCode::TRANSACTION_EXPIRED => Self::TransactionExpired,
            StatusCode::SENDING_ACCOUNT_DOES_NOT_EXIST => Self::SendingAccountDoesNotExist,
            StatusCode::REJECTED_WRITE_SET => Self::RejectedWriteSet,
            StatusCode::INVALID_WRITE_SET => Self::InvalidWriteSet,
            StatusCode::EXCEEDED_MAX_TRANSACTION_SIZE => Self::ExceededMaxTransactionSize,
            StatusCode::UNKNOWN_SCRIPT => Self::UnknownScript,
            StatusCode::UNKNOWN_MODULE => Self::UnknownModule,
            StatusCode::MAX_GAS_UNITS_EXCEEDS_MAX_GAS_UNITS_BOUND => {
                Self::MaxGasUnitsExceedsMaxGasUnitsBound
            }
            StatusCode::MAX_GAS_UNITS_BELOW_MIN_TRANSACTION_GAS_UNITS => {
                Self::MaxGasUnitsBelowMinTransactionGasUnits
            }
            StatusCode::GAS_UNIT_PRICE_BELOW_MIN_BOUND => Self::GasUnitPriceBelowMinBound,
            StatusCode::GAS_UNIT_PRICE_ABOVE_MAX_BOUND => Self::GasUnitPriceAboveMaxBound,
            StatusCode::INVALID_GAS_SPECIFIER => Self::InvalidGasSpecifier,
            StatusCode::SENDING_ACCOUNT_FROZEN => Self::SendingAccountFrozen,
            StatusCode::UNABLE_TO_DESERIALIZE_ACCOUNT => Self::UnableToDeserializeAccount,
            StatusCode::CURRENCY_INFO_DOES_NOT_EXIST => Self::CurrencyInfoDoesNotExist,
            StatusCode::INVALID_MODULE_PUBLISHER => Self::InvalidModulePublisher,
            StatusCode::NO_ACCOUNT_ROLE => Self::NoAccountRole,
            StatusCode::BAD_CHAIN_ID => Self::BadChainId,
            StatusCode::UNKNOWN_VERIFICATION_ERROR => Self::UnknownVerificationError,
            StatusCode::INDEX_OUT_OF_BOUNDS => Self::IndexOutOfBounds,
            StatusCode::INVALID_SIGNATURE_TOKEN => Self::InvalidSignatureToken,
            StatusCode::RECURSIVE_STRUCT_DEFINITION => Self::RecursiveStructDefinition,
            StatusCode::INVALID_RESOURCE_FIELD => Self::InvalidResourceField,
            StatusCode::INVALID_FALL_THROUGH => Self::InvalidFallThrough,
            StatusCode::NEGATIVE_STACK_SIZE_WITHIN_BLOCK => Self::NegativeStackSizeWithinBlock,
            StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE => Self::InvalidMainFunctionSignature,
            StatusCode::DUPLICATE_ELEMENT => Self::DuplicateElement,
            StatusCode::INVALID_MODULE_HANDLE => Self::InvalidModuleHandle,
            StatusCode::UNIMPLEMENTED_HANDLE => Self::UnimplementedHandle,
            StatusCode::LOOKUP_FAILED => Self::LookupFailed,
            StatusCode::TYPE_MISMATCH => Self::TypeMismatch,
            StatusCode::MISSING_DEPENDENCY => Self::MissingDependency,
            StatusCode::POP_RESOURCE_ERROR => Self::PopResourceError,
            StatusCode::BR_TYPE_MISMATCH_ERROR => Self::BrTypeMismatchError,
            StatusCode::ABORT_TYPE_MISMATCH_ERROR => Self::AbortTypeMismatchError,
            StatusCode::STLOC_TYPE_MISMATCH_ERROR => Self::StlocTypeMismatchError,
            StatusCode::STLOC_UNSAFE_TO_DESTROY_ERROR => Self::StlocUnsafeToDestroyError,
            StatusCode::UNSAFE_RET_LOCAL_OR_RESOURCE_STILL_BORROWED => {
                Self::UnsafeRetLocalOrResourceStillBorrowed
            }
            StatusCode::RET_TYPE_MISMATCH_ERROR => Self::RetTypeMismatchError,
            StatusCode::RET_BORROWED_MUTABLE_REFERENCE_ERROR => {
                Self::RetBorrowedMutableReferenceError
            }
            StatusCode::FREEZEREF_TYPE_MISMATCH_ERROR => Self::FreezerefTypeMismatchError,
            StatusCode::FREEZEREF_EXISTS_MUTABLE_BORROW_ERROR => {
                Self::FreezerefExistsMutableBorrowError
            }
            StatusCode::BORROWFIELD_TYPE_MISMATCH_ERROR => Self::BorrowfieldTypeMismatchError,
            StatusCode::BORROWFIELD_BAD_FIELD_ERROR => Self::BorrowfieldBadFieldError,
            StatusCode::BORROWFIELD_EXISTS_MUTABLE_BORROW_ERROR => {
                Self::BorrowfieldExistsMutableBorrowError
            }
            StatusCode::COPYLOC_UNAVAILABLE_ERROR => Self::CopylocUnavailableError,
            StatusCode::COPYLOC_RESOURCE_ERROR => Self::CopylocResourceError,
            StatusCode::COPYLOC_EXISTS_BORROW_ERROR => Self::CopylocExistsBorrowError,
            StatusCode::MOVELOC_UNAVAILABLE_ERROR => Self::MovelocUnavailableError,
            StatusCode::MOVELOC_EXISTS_BORROW_ERROR => Self::MovelocExistsBorrowError,
            StatusCode::BORROWLOC_REFERENCE_ERROR => Self::BorrowlocReferenceError,
            StatusCode::BORROWLOC_UNAVAILABLE_ERROR => Self::BorrowlocUnavailableError,
            StatusCode::BORROWLOC_EXISTS_BORROW_ERROR => Self::BorrowlocExistsBorrowError,
            StatusCode::CALL_TYPE_MISMATCH_ERROR => Self::CallTypeMismatchError,
            StatusCode::CALL_BORROWED_MUTABLE_REFERENCE_ERROR => {
                Self::CallBorrowedMutableReferenceError
            }
            StatusCode::PACK_TYPE_MISMATCH_ERROR => Self::PackTypeMismatchError,
            StatusCode::UNPACK_TYPE_MISMATCH_ERROR => Self::UnpackTypeMismatchError,
            StatusCode::READREF_TYPE_MISMATCH_ERROR => Self::ReadrefTypeMismatchError,
            StatusCode::READREF_RESOURCE_ERROR => Self::ReadrefResourceError,
            StatusCode::READREF_EXISTS_MUTABLE_BORROW_ERROR => {
                Self::ReadrefExistsMutableBorrowError
            }
            StatusCode::WRITEREF_TYPE_MISMATCH_ERROR => Self::WriterefTypeMismatchError,
            StatusCode::WRITEREF_RESOURCE_ERROR => Self::WriterefResourceError,
            StatusCode::WRITEREF_EXISTS_BORROW_ERROR => Self::WriterefExistsBorrowError,
            StatusCode::WRITEREF_NO_MUTABLE_REFERENCE_ERROR => {
                Self::WriterefNoMutableReferenceError
            }
            StatusCode::INTEGER_OP_TYPE_MISMATCH_ERROR => Self::IntegerOpTypeMismatchError,
            StatusCode::BOOLEAN_OP_TYPE_MISMATCH_ERROR => Self::BooleanOpTypeMismatchError,
            StatusCode::EQUALITY_OP_TYPE_MISMATCH_ERROR => Self::EqualityOpTypeMismatchError,
            StatusCode::EXISTS_RESOURCE_TYPE_MISMATCH_ERROR => {
                Self::ExistsResourceTypeMismatchError
            }
            StatusCode::BORROWGLOBAL_TYPE_MISMATCH_ERROR => Self::BorrowglobalTypeMismatchError,
            StatusCode::BORROWGLOBAL_NO_RESOURCE_ERROR => Self::BorrowglobalNoResourceError,
            StatusCode::MOVEFROM_TYPE_MISMATCH_ERROR => Self::MovefromTypeMismatchError,
            StatusCode::MOVEFROM_NO_RESOURCE_ERROR => Self::MovefromNoResourceError,
            StatusCode::MOVETO_TYPE_MISMATCH_ERROR => Self::MovetoTypeMismatchError,
            StatusCode::MOVETO_NO_RESOURCE_ERROR => Self::MovetoNoResourceError,
            StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER => {
                Self::ModuleAddressDoesNotMatchSender
            }
            StatusCode::NO_MODULE_HANDLES => Self::NoModuleHandles,
            StatusCode::POSITIVE_STACK_SIZE_AT_BLOCK_END => Self::PositiveStackSizeAtBlockEnd,
            StatusCode::MISSING_ACQUIRES_RESOURCE_ANNOTATION_ERROR => {
                Self::MissingAcquiresResourceAnnotationError
            }
            StatusCode::EXTRANEOUS_ACQUIRES_RESOURCE_ANNOTATION_ERROR => {
                Self::ExtraneousAcquiresResourceAnnotationError
            }
            StatusCode::DUPLICATE_ACQUIRES_RESOURCE_ANNOTATION_ERROR => {
                Self::DuplicateAcquiresResourceAnnotationError
            }
            StatusCode::INVALID_ACQUIRES_RESOURCE_ANNOTATION_ERROR => {
                Self::InvalidAcquiresResourceAnnotationError
            }
            StatusCode::GLOBAL_REFERENCE_ERROR => Self::GlobalReferenceError,
            StatusCode::CONSTRAINT_KIND_MISMATCH => Self::ConstraintKindMismatch,
            StatusCode::NUMBER_OF_TYPE_ARGUMENTS_MISMATCH => Self::NumberOfTypeArgumentsMismatch,
            StatusCode::LOOP_IN_INSTANTIATION_GRAPH => Self::LoopInInstantiationGraph,
            StatusCode::ZERO_SIZED_STRUCT => Self::ZeroSizedStruct,
            StatusCode::LINKER_ERROR => Self::LinkerError,
            StatusCode::INVALID_CONSTANT_TYPE => Self::InvalidConstantType,
            StatusCode::MALFORMED_CONSTANT_DATA => Self::MalformedConstantData,
            StatusCode::EMPTY_CODE_UNIT => Self::EmptyCodeUnit,
            StatusCode::INVALID_LOOP_SPLIT => Self::InvalidLoopSplit,
            StatusCode::INVALID_LOOP_BREAK => Self::InvalidLoopBreak,
            StatusCode::INVALID_LOOP_CONTINUE => Self::InvalidLoopContinue,
            StatusCode::UNSAFE_RET_UNUSED_RESOURCES => Self::UnsafeRetUnusedResources,
            StatusCode::TOO_MANY_LOCALS => Self::TooManyLocals,
            StatusCode::GENERIC_MEMBER_OPCODE_MISMATCH => Self::GenericMemberOpcodeMismatch,
            StatusCode::FUNCTION_RESOLUTION_FAILURE => Self::FunctionResolutionFailure,
            StatusCode::INVALID_OPERATION_IN_SCRIPT => Self::InvalidOperationInScript,
            StatusCode::DUPLICATE_MODULE_NAME => Self::DuplicateModuleName,
            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR => Self::UnknownInvariantViolationError,
            StatusCode::EMPTY_VALUE_STACK => Self::EmptyValueStack,
            StatusCode::PC_OVERFLOW => Self::PcOverflow,
            StatusCode::VERIFICATION_ERROR => Self::VerificationError,
            StatusCode::STORAGE_ERROR => Self::StorageError,
            StatusCode::INTERNAL_TYPE_ERROR => Self::InternalTypeError,
            StatusCode::EVENT_KEY_MISMATCH => Self::EventKeyMismatch,
            StatusCode::UNREACHABLE => Self::Unreachable,
            StatusCode::VM_STARTUP_FAILURE => Self::VmStartupFailure,
            StatusCode::UNEXPECTED_ERROR_FROM_KNOWN_MOVE_FUNCTION => {
                Self::UnexpectedErrorFromKnownMoveFunction
            }
            StatusCode::VERIFIER_INVARIANT_VIOLATION => Self::VerifierInvariantViolation,
            StatusCode::UNEXPECTED_VERIFIER_ERROR => Self::UnexpectedVerifierError,
            StatusCode::UNEXPECTED_DESERIALIZATION_ERROR => Self::UnexpectedDeserializationError,
            StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES => {
                Self::FailedToSerializeWriteSetChanges
            }
            StatusCode::FAILED_TO_DESERIALIZE_RESOURCE => Self::FailedToDeserializeResource,
            StatusCode::TYPE_RESOLUTION_FAILURE => Self::TypeResolutionFailure,
            StatusCode::UNKNOWN_BINARY_ERROR => Self::UnknownBinaryError,
            StatusCode::MALFORMED => Self::Malformed,
            StatusCode::BAD_MAGIC => Self::BadMagic,
            StatusCode::UNKNOWN_VERSION => Self::UnknownVersion,
            StatusCode::UNKNOWN_TABLE_TYPE => Self::UnknownTableType,
            StatusCode::UNKNOWN_SIGNATURE_TYPE => Self::UnknownSignatureType,
            StatusCode::UNKNOWN_SERIALIZED_TYPE => Self::UnknownSerializedType,
            StatusCode::UNKNOWN_OPCODE => Self::UnknownOpcode,
            StatusCode::BAD_HEADER_TABLE => Self::BadHeaderTable,
            StatusCode::UNEXPECTED_SIGNATURE_TYPE => Self::UnexpectedSignatureType,
            StatusCode::DUPLICATE_TABLE => Self::DuplicateTable,
            StatusCode::UNKNOWN_NOMINAL_RESOURCE => Self::UnknownNominalResource,
            StatusCode::UNKNOWN_KIND => Self::UnknownKind,
            StatusCode::UNKNOWN_NATIVE_STRUCT_FLAG => Self::UnknownNativeStructFlag,
            StatusCode::BAD_U64 => Self::BadU64,
            StatusCode::BAD_U128 => Self::BadU128,
            StatusCode::VALUE_SERIALIZATION_ERROR => Self::ValueSerializationError,
            StatusCode::VALUE_DESERIALIZATION_ERROR => Self::ValueDeserializationError,
            StatusCode::CODE_DESERIALIZATION_ERROR => Self::CodeDeserializationError,
            StatusCode::UNKNOWN_RUNTIME_STATUS => Self::UnknownRuntimeStatus,
            StatusCode::OUT_OF_GAS => Self::OutOfGas,
            StatusCode::RESOURCE_DOES_NOT_EXIST => Self::ResourceDoesNotExist,
            StatusCode::RESOURCE_ALREADY_EXISTS => Self::ResourceAlreadyExists,
            StatusCode::MISSING_DATA => Self::MissingData,
            StatusCode::DATA_FORMAT_ERROR => Self::DataFormatError,
            StatusCode::ABORTED => Self::Aborted,
            StatusCode::ARITHMETIC_ERROR => Self::ArithmeticError,
            StatusCode::EXECUTION_STACK_OVERFLOW => Self::ExecutionStackOverflow,
            StatusCode::CALL_STACK_OVERFLOW => Self::CallStackOverflow,
            StatusCode::VM_MAX_TYPE_DEPTH_REACHED => Self::VmMaxTypeDepthReached,
            StatusCode::VM_MAX_VALUE_DEPTH_REACHED => Self::VmMaxValueDepthReached,
            StatusCode::UNKNOWN_STATUS => Self::UnknownStatus,

            StatusCode::EXECUTED => unreachable!(),
        }
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Script senders should not be empty
        ScriptValidationError,

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
    }
}
