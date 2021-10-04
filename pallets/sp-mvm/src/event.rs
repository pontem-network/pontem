// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! Implement support of Move VM events inside Substrate.
use core::convert::TryInto;
use move_vm::io::traits::EventHandler;
use sp_std::prelude::*;
use move_core_types::language_storage::TypeTag;
use crate::{Event, Config};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::format;

pub trait DepositMoveEvent {
    /// Emit a Move event with content of passed `MoveEventArguments'.
    fn deposit_move_event(e: MoveEventArguments);
}

/// Basic event writer.
pub struct EventWriter<F>(F);

/// Move VM event struct.
pub struct MoveEventArguments {
    /// Event GUID.
    pub guid: Vec<u8>,
    /// Move VM type stored into event.
    pub ty_tag: TypeTag,
    /// Event message.
    pub message: Vec<u8>,
}

impl<T: Config> TryInto<Event<T>> for MoveEventArguments {
    type Error = parity_scale_codec::Error;

    /// Convert Move VM event into pallet one.
    fn try_into(self) -> Result<Event<T>, Self::Error> {
        let ty_tag_enc = format!("{}", self.ty_tag).as_bytes().to_vec();
        Ok(Event::Event(self.guid, ty_tag_enc, self.message))
    }
}

impl<F: Fn(MoveEventArguments)> EventHandler for EventWriter<F> {
    #[inline]
    /// Catch new events and pass them to Even Writer function.
    fn on_event(&self, guid: Vec<u8>, _seq_num: u64, ty_tag: TypeTag, message: Vec<u8>) {
        self.0(MoveEventArguments {
            guid,
            ty_tag,
            message,
        })
    }
}

impl<F> EventWriter<F> {
    /// New Event Writer.
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

/// Default EventWriter.
pub type DefaultEventHandler = EventWriter<Box<dyn Fn(MoveEventArguments)>>;

/// Boxed fn ptr to something looks like `DepositMoveEvent::deposit_move_event`.
pub type DepositMoveEventFnPtr = Box<dyn Fn(MoveEventArguments)>;

pub trait GetDepositMoveEventFn<T: DepositMoveEvent + 'static> {
    fn get_deposit_move_event_fn() -> DepositMoveEventFnPtr {
        Box::new(T::deposit_move_event)
    }
}

impl<T: DepositMoveEvent + 'static> GetDepositMoveEventFn<T> for T {}

pub trait CreateMoveEventHandler<T> {
    type EventHandler: EventHandler;

    /// Create a new event handler for Move VM.
    fn create_move_event_handler() -> Self::EventHandler;
}

impl<T> CreateMoveEventHandler<T> for T
where
    T: DepositMoveEvent + 'static,
    T: GetDepositMoveEventFn<T>,
{
    type EventHandler = DefaultEventHandler;

    /// Create a new event handler for Move VM.
    ///
    /// Event Handler will be used inside the VM to convert Move VM events to Substrate one once a new event happens.
    fn create_move_event_handler() -> Self::EventHandler {
        EventWriter::new(T::get_deposit_move_event_fn())
    }
}
