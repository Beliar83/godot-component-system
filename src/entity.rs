use cxx::{type_id, ExternType};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn create() -> Self {
        EntityId(Uuid::new_v4())
    }

    pub fn as_string(&self) -> String {
        self.0.to_hyphenated().to_string()
    }
}

unsafe impl ExternType for EntityId {
    type Id = type_id!("gcs::ffi::EntityId");
    type Kind = cxx::kind::Trivial;
}

fn entity_id_from_u64(id: u64) -> Box<EntityId> {
    Box::new(EntityId(Uuid::from_u128(id as u128)))
}

pub fn entity_id_from_string(id: String) -> Result<Box<EntityId>, String> {
    let uuid = Uuid::parse_str(id.as_str());
    match uuid {
        Ok(uuid) => Ok(Box::new(EntityId(uuid))),
        Err(err) => Err(err.to_string()),
    }
}

pub(crate) fn create_entity() -> Box<EntityId> {
    Box::new(EntityId::create())
}
