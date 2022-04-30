use cxx::{type_id, ExternType};
use uuid::Uuid;

use gcs::entity::EntityId;

use godot_cxx::godot_result;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CXXEntityId(uuid::Uuid);

godot_result!(Box<CXXEntityId>, EntityIdResult);

impl EntityId for CXXEntityId {
    fn create() -> Self
    where
        Self: Sized,
    {
        CXXEntityId(Uuid::new_v4())
    }

    fn as_string(&self) -> String {
        self.0.to_hyphenated().to_string()
    }

    fn parse_str(input: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let uuid = Uuid::parse_str(&input);
        match uuid {
            Ok(uuid) => Ok(CXXEntityId(uuid)),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl CXXEntityId {
    pub(crate) fn as_string(&self) -> String {
        EntityId::as_string(self)
    }
}

unsafe impl ExternType for CXXEntityId {
    type Id = type_id!("gcs::ffi::EntityId");
    type Kind = cxx::kind::Trivial;
}

pub(crate) fn create_entity() -> Box<CXXEntityId> {
    Box::new(CXXEntityId::create())
}

pub(crate) fn entity_id_from_string(id: String) -> Box<EntityIdResult> {
    Box::new(match CXXEntityId::parse_str(id.as_str()) {
        Ok(value) => EntityIdResult::new_result(Box::new(value)),
        Err(err) => EntityIdResult::new_error(err),
    })
}
