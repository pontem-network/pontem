use super::*;
use frame_support::sp_std::marker::PhantomData;
use vm::errors::VMError;

pub enum Error<T: Trait> {
    #[doc(hidden)]
    /// XXX: Used only to store phantom data
    __Ignore(PhantomData<T>, frame_support::Never),

    /// Error names should be descriptive.
    NoneValue,
    /// Errors should have helpful documentation associated with them.
    StorageOverflow,
    MoveScriptTxValidationError,
    MoveScriptTxExecutionError,
}

impl<T: Trait> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        // f.write_str(self.as_str())
        f.write_str("self.as_str()")
    }
}

// impl<T: Trait> Error<T> {
//     fn as_u8(&self) -> u8 {
//         match self {
//             Error::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
//                 &["internal error: entered unreachable code: "],
//                 &match (&"`__Ignore` can never be constructed",) {
//                     (arg0,) => [::core::fmt::ArgumentV1::new(
//                         arg0,
//                         ::core::fmt::Display::fmt,
//                     )],
//                 },
//             )),
//             Error::NoneValue => 0,
//             Error::StorageOverflow => 0 + 1,
//             Error::MoveScriptTxValidationError => 0 + 1 + 1,
//             Error::MoveScriptTxExecutionError => 0 + 1 + 1 + 1,
//         }
//     }
//     fn as_str(&self) -> &'static str {
//         match self {
//             Self::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
//                 &["internal error: entered unreachable code: "],
//                 &match (&"`__Ignore` can never be constructed",) {
//                     (arg0,) => [::core::fmt::ArgumentV1::new(
//                         arg0,
//                         ::core::fmt::Display::fmt,
//                     )],
//                 },
//             )),
//             Error::NoneValue => "NoneValue",
//             Error::StorageOverflow => "StorageOverflow",
//             Error::MoveScriptTxValidationError => "MoveScriptTxValidationError",
//             Error::MoveScriptTxExecutionError => "MoveScriptTxExecutionError",
//         }
//     }
// }
impl<T: Trait> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        "err.as_str()"
    }
}
impl<T: Trait> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::PalletInfo as ::frame_support::traits::PalletInfo>::index::<Module<T>>()
            .expect("Every active module has an index in the runtime; qed")
            as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: 42, //err.as_u8(),
            message: Some("err.as_str()"),
        }
    }
}
impl<T: Trait> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NoneValue"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Error names should be descriptive.",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("StorageOverflow"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Errors should have helpful documentation associated with them.",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "MoveScriptTxValidationError",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "MoveScriptTxExecutionError",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: Trait> From<VMError> for Error<T> {
    fn from(_: VMError) -> Self {
        // ::std::rt::begin_panic("not yet implemented")
        unimplemented!("not yet implemented");
    }
}
