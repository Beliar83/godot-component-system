use crate::component::component_value::ComponentValue;
use crate::entity::EntityId;
use cxx::{type_id, ExternType};
use std::collections::HashMap;

pub struct ComponentData {
    entity: EntityId,
    fields: HashMap<String, ComponentValue>,
}

impl ComponentData {
    pub fn new(entity: EntityId) -> Self {
        Self {
            entity,
            fields: HashMap::new(),
        }
    }

    pub fn get_entity(&self) -> EntityId {
        self.entity
    }

    pub fn get_field(&self, field: String) -> &ComponentValue {
        if self.fields.contains_key(&field) {
            self.fields.get(&field).unwrap()
        } else {
            &ComponentValue::Nil
        }
    }

    pub fn set_field(&mut self, field: String, value: &ComponentValue) {
        self.fields.insert(field, value.clone());
    }
}

pub(crate) fn create_component_data(entity: &EntityId) -> Box<ComponentData> {
    Box::new(ComponentData::new(*entity))
}

unsafe impl ExternType for ComponentData {
    type Id = type_id!("gcs::ffi::ComponentData");
    type Kind = cxx::kind::Opaque;
}
