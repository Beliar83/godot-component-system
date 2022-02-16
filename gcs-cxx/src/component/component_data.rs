use crate::component::component_value::CXXComponentValue;
use crate::entity::CXXEntityId;
use cxx::{type_id, ExternType};
use gcs::component::component_data::ComponentData;
use std::collections::HashMap;

#[derive(Default)]
pub struct CXXComponentData {
    entity: CXXEntityId,
    fields: HashMap<String, CXXComponentValue>,
}

impl ComponentData for CXXComponentData {
    type EntityIdType = CXXEntityId;
    type ComponentValueType = CXXComponentValue;

    fn new(entity: CXXEntityId) -> Self {
        Self {
            entity,
            fields: HashMap::new(),
        }
    }

    fn get_entity(&self) -> CXXEntityId {
        self.entity
    }

    fn get_field(&self, field: String) -> &Self::ComponentValueType {
        if self.fields.contains_key(&field) {
            &self.fields.get(&field).unwrap()
        } else {
            &Self::ComponentValueType::Nil
        }
    }

    fn set_field(&mut self, field: String, value: &Self::ComponentValueType) {
        self.fields.insert(field, value.clone());
    }
}

impl CXXComponentData {
    pub(crate) fn get_field(&self, field: String) -> &CXXComponentValue {
        ComponentData::get_field(self, field)
    }

    pub(crate) fn set_field(&mut self, field: String, value: &CXXComponentValue) {
        ComponentData::set_field(self, field, &value)
    }
}

pub(crate) fn create_component_data(entity: &CXXEntityId) -> Box<CXXComponentData> {
    Box::new(CXXComponentData::new(*entity))
}

unsafe impl ExternType for CXXComponentData {
    type Id = type_id!("gcs::ffi::ComponentData");
    type Kind = cxx::kind::Opaque;
}
