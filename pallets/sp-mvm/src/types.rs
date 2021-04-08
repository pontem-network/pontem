use core::convert::{TryInto, TryFrom};
use sp_std::prelude::*;
use codec::{Encode, Decode};
use crate::addr::address_to_account;
use move_core_types::language_storage::ModuleId as InternalModuleId;
use move_core_types::language_storage::StructTag as InternalStructTag;
use move_core_types::language_storage::TypeTag as InternalTypeTag;

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub struct MoveModuleId<AccountId> {
    pub owner: AccountId,
    pub module: Vec<u8>,
}

impl<AccountId: Decode> TryFrom<InternalModuleId> for MoveModuleId<AccountId> {
    type Error = codec::Error;

    fn try_from(id: InternalModuleId) -> Result<Self, Self::Error> {
        Ok(Self {
            owner: address_to_account::<AccountId>(&id.address())?,
            module: id.name().as_bytes().to_vec(),
        })
    }
}

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub enum MoveTypeTag<AccountId: Decode> {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<Self>),
    Struct(MoveStructTag<AccountId>),
}

impl<AccountId: Decode> TryFrom<InternalTypeTag> for MoveTypeTag<AccountId> {
    type Error = codec::Error;

    fn try_from(tt: InternalTypeTag) -> Result<Self, Self::Error> {
        Ok(match tt {
            InternalTypeTag::Bool => MoveTypeTag::Bool,
            InternalTypeTag::U8 => MoveTypeTag::U8,
            InternalTypeTag::U64 => MoveTypeTag::U64,
            InternalTypeTag::U128 => MoveTypeTag::U128,
            InternalTypeTag::Address => MoveTypeTag::Address,
            InternalTypeTag::Signer => MoveTypeTag::Signer,
            InternalTypeTag::Vector(tt) => MoveTypeTag::Vector(Box::new(tt.try_into()?)),
            InternalTypeTag::Struct(st) => MoveTypeTag::Struct(st.try_into()?),
        })
    }
}
impl<AccountId: Decode> TryFrom<Box<InternalTypeTag>> for MoveTypeTag<AccountId> {
    type Error = codec::Error;

    fn try_from(tt: Box<InternalTypeTag>) -> Result<Self, Self::Error> {
        Ok(match *tt {
            InternalTypeTag::Bool => MoveTypeTag::Bool,
            InternalTypeTag::U8 => MoveTypeTag::U8,
            InternalTypeTag::U64 => MoveTypeTag::U64,
            InternalTypeTag::U128 => MoveTypeTag::U128,
            InternalTypeTag::Address => MoveTypeTag::Address,
            InternalTypeTag::Signer => MoveTypeTag::Signer,
            InternalTypeTag::Vector(tt) => MoveTypeTag::Vector(Box::new(tt.try_into()?)),
            InternalTypeTag::Struct(st) => MoveTypeTag::Struct(st.try_into()?),
        })
    }
}

#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub struct MoveStructTag<AccountId: Decode /* TryFrom<AccountAddress> */> {
    pub owner: AccountId,
    pub module: Vec<u8>, /* from Identifier, use Text in web-UI */
    pub name: Vec<u8>,   /* from Identifier, use Text in web-UI */

    // TODO: fix recursion on types (MoveTypeTag in MoveTypeTag)
    pub ty_params: Vec<()>,
}

impl<AccountId: Decode> TryFrom<InternalStructTag> for MoveStructTag<AccountId> {
    type Error = codec::Error;

    fn try_from(st: InternalStructTag) -> Result<Self, Self::Error> {
        let mut type_params = Vec::new();
        for tp in st.type_params.into_iter() {
            let _tp: MoveTypeTag<AccountId> = tp.try_into()?;
            type_params.push(());
        }

        Ok(Self {
            owner: address_to_account::<AccountId>(&st.address)?,
            module: st.module.into_string().as_bytes().to_vec(),
            name: st.name.into_string().as_bytes().to_vec(),
            ty_params: type_params,
        })
    }
}
