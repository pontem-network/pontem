use core::convert::TryInto;
use sp_std::prelude::*;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::TypeTag;
use move_vm::data::EventHandler;
use crate::types;
use crate::{Event, Config};
use crate::addr::address_to_account;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::format;

pub trait DepositMoveEvent {
    /// Emit a Move event with content of passed `MoveEventArguments`
    fn deposit_move_event(e: MoveEventArguments);
}

pub struct EventWriter<F>(F);

pub struct MoveEventArguments {
    pub addr: AccountAddress,
    pub ty_tag: TypeTag,
    pub message: Vec<u8>,
    pub caller: Option<ModuleId>,
}

impl<T: Config> TryInto<Event<T>> for MoveEventArguments {
    type Error = codec::Error;

    fn try_into(self) -> Result<Event<T>, Self::Error> {
        let account = address_to_account::<T::AccountId>(&self.addr)?;
        let mut caller_error = None::<Self::Error>;
        let caller: Option<types::MoveModuleId<T::AccountId>> = self
            .caller
            .map(|caller| {
                caller
                    .try_into()
                    .map_err(|err| caller_error = Some(err))
                    .ok()
            })
            .flatten();

        if let Some(err) = caller_error {
            return Err(err);
        }

        let ty_tag_enc = format!("{}", self.ty_tag).as_bytes().to_vec();

        Ok(Event::Event(
            account,
            ty_tag_enc,
            self.message,
            caller,
        ))
    }
}

impl<F: Fn(MoveEventArguments)> EventHandler for EventWriter<F> {
    #[inline]
    fn on_event(
        &self,
        addr: AccountAddress,
        ty_tag: TypeTag,
        message: Vec<u8>,
        caller: Option<ModuleId>,
    ) {
        self.0(MoveEventArguments {
            addr,
            ty_tag,
            message,
            caller,
        })
    }
}

impl<F> EventWriter<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

/// Default EventWriter
pub type DefaultEventHandler = EventWriter<Box<dyn Fn(MoveEventArguments)>>;

/// Boxed fn ptr to something looks like `DepositMoveEvent::deposit_move_event`
pub type DepositMoveEventFnPtr = Box<dyn Fn(MoveEventArguments)>;

pub trait GetDepositMoveEventFn<T: DepositMoveEvent + 'static> {
    fn get_deposit_move_event_fn() -> DepositMoveEventFnPtr {
        Box::new(T::deposit_move_event)
    }
}

impl<T: DepositMoveEvent + 'static> GetDepositMoveEventFn<T> for T {}

pub trait CreateMoveEventHandler<T> {
    type EventHandler: EventHandler;

    fn create_move_event_handler() -> Self::EventHandler;
}

impl<T> CreateMoveEventHandler<T> for T
where
    T: DepositMoveEvent + 'static,
    T: GetDepositMoveEventFn<T>,
{
    type EventHandler = DefaultEventHandler;

    fn create_move_event_handler() -> Self::EventHandler {
        EventWriter::new(T::get_deposit_move_event_fn())
    }
}
