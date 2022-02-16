use cxx::{type_id, ExternType};
use gcs::entity::EntityId;
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct CXXEntityId(uuid::Uuid);

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

    fn parse_str(input: &str) -> Result<Box<Self>, String>
    where
        Self: Sized,
    {
        let uuid = Uuid::parse_str(&input);
        match uuid {
            Ok(uuid) => Ok(Box::new(CXXEntityId(uuid))),
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

pub(crate) fn entity_id_from_string(id: String) -> Result<Box<CXXEntityId>, String> {
    CXXEntityId::parse_str(id.as_str())
}
