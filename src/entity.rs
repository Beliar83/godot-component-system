use cxx::{type_id, ExternType};
use uuid::Uuid;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    extern "Rust" {
        type EntityId;

        fn entity_id_from_u64_(id: u64) -> Box<EntityId>;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn create() -> Self {
        EntityId(Uuid::new_v4())
    }
}

unsafe impl ExternType for EntityId {
    type Id = type_id!("gcs::ffi::EntityId");
    type Kind = cxx::kind::Opaque;
}

fn entity_id_from_u64_(id: u64) -> Box<EntityId> {
    Box::new(EntityId(Uuid::from_u128(id as u128)))
}

fn create_entity_ud() -> Box<EntityId> {
    Box::new(EntityId::create())
}
