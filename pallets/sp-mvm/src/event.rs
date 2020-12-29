use sp_std::prelude::*;
use frame_support::decl_event;
use move_vm::data::EventHandler;
use move_core_types::language_storage::TypeTag;

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        // Event documentation should end with an array that provides descriptive names for event parameters.
        /// Event provided by Move VM
        /// [guid, seq_num, message]
        MvmEvent(Vec<u8>, u64, Vec<u8>),
        // / [guid, seq_num, ty_tag, message]
        // Event(Vec<u8>, u64, TypeTag, Vec<u8>),
        /// Event about successful move-module publishing
        /// [account]
        ModulePublished(AccountId),
    }
);

pub struct EventWriter<F, E0>(F, core::marker::PhantomData<(E0,)>);

impl<F: Fn(RawEvent<E0>), E0> EventHandler for EventWriter<F, E0> {
    fn on_event(&self, guid: Vec<u8>, seq_num: u64, ty_tag: TypeTag, message: Vec<u8>) {
        debug!(
            "MVM Event: {:?} {:?} {:?} {:?}",
            guid, seq_num, ty_tag, message
        );

        // Emit an event:
        self.0(RawEvent::<E0>::MvmEvent(guid, seq_num, message))
    }
}

impl<F, E0> EventWriter<F, E0> {
    pub fn new(f: F) -> Self {
        Self(f, Default::default())
    }
}
