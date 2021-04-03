use crate::{Event, Config};
use sp_std::prelude::*;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::TypeTag;
use move_vm::data::EventHandler;

pub trait DepositMoveEvent {
    fn deposit_move_event(e: MoveEventArguments);
}

pub struct EventWriter<F>(F);

pub struct MoveEventArguments {
    pub addr: AccountAddress,
    pub ty_tag: TypeTag,
    pub message: Vec<u8>,
    pub caller: Option<ModuleId>,
}

impl<T: Config> Into<Event<T>> for MoveEventArguments {
    fn into(self) -> Event<T> {
        use codec::Encode;
        Event::Event(self.addr, self.ty_tag.encode(), self.message, self.caller)
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
