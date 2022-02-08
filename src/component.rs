use crate::component::ffi::{
    empty_variant, variant_from_bool, variant_from_f64, variant_from_i64, variant_from_string,
};
use crate::ecs_world::EntityId;
use crate::godot::variant::ffi::VariantType;
use crate::godot::variant::ffi::{
    variant_as_bool, variant_as_f64, variant_as_i64, variant_as_string, Variant,
};
use cxx::{type_id, v, ExternType};
use std::collections::HashMap;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub struct ComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    extern "Rust" {
        pub fn create_component_field_definition(
            name: String,
            field_type: VariantType,
        ) -> ComponentFieldDefinition;

        type ComponentData;
        type ComponentValue;

        fn get_field(self: &ComponentData, field: String) -> &ComponentValue;
        fn set_field(self: &mut ComponentData, field: String, value: &ComponentValue);

        fn variant_from_component_value(value: &ComponentValue) -> &'static Variant;
        fn component_value_from_variant(value: &Variant) -> Box<ComponentValue>;
    }

    unsafe extern "C++" {
        include!("component.h");
        include!("cxx.h");
        pub type Variant = crate::godot::variant::ffi::Variant;
        type VariantType = crate::godot::variant::ffi::VariantType;

        pub(crate) fn empty_variant() -> &'static Variant;
        pub(crate) fn variant_from_i64(value: i64) -> &'static Variant;
        pub(crate) fn variant_from_string(value: String) -> &'static Variant;
        pub(crate) fn variant_from_bool(value: bool) -> &'static Variant;
        pub(crate) fn variant_from_f64(value: f64) -> &'static Variant;

    }
}

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

unsafe impl ExternType for ComponentData {
    type Id = type_id!("gcs::ffi::ComponentData");
    type Kind = cxx::kind::Opaque;
}

#[derive(Clone, PartialEq, Debug)]
pub enum ComponentValue {
    Nil,
    Int(i64),
    String(String),
    Bool(bool),
    Real(f64),
}

impl Default for ComponentValue {
    fn default() -> Self {
        ComponentValue::Nil
    }
}

fn variant_from_component_value(value: &ComponentValue) -> &'static Variant {
    #[cfg(not(test))]
    {
        match value {
            ComponentValue::Nil => empty_variant(),
            ComponentValue::Int(value) => variant_from_i64(*value),
            ComponentValue::String(value) => variant_from_string(value.clone()),
            ComponentValue::Bool(value) => variant_from_bool(*value),
            ComponentValue::Real(value) => variant_from_f64(*value),
        }
    }
    #[cfg(test)]
    unimplemented!()
}

fn component_value_from_variant(value: &Variant) -> Box<ComponentValue> {
    #[cfg(not(test))]
    {
        let variant_type: VariantType = value.get_type();

        match variant_type {
            VariantType::NIL => Box::new(ComponentValue::Nil),
            VariantType::BOOL => Box::new(ComponentValue::Bool(variant_as_bool(value))),
            VariantType::INT => Box::new(ComponentValue::Int(variant_as_i64(value))),
            VariantType::REAL => Box::new(ComponentValue::Real(variant_as_f64(value))),
            VariantType::STRING => Box::new(ComponentValue::String(variant_as_string(value))),
            VariantType::VECTOR2 => {
                unimplemented!()
            }
            VariantType::RECT2 => {
                unimplemented!()
            }
            VariantType::VECTOR3 => {
                unimplemented!()
            }
            VariantType::TRANSFORM2D => {
                unimplemented!()
            }
            VariantType::PLANE => {
                unimplemented!()
            }
            VariantType::QUAT => {
                unimplemented!()
            }
            VariantType::AABB => {
                unimplemented!()
            }
            VariantType::BASIS => {
                unimplemented!()
            }
            VariantType::TRANSFORM => {
                unimplemented!()
            }
            VariantType::COLOR => {
                unimplemented!()
            }
            VariantType::NODE_PATH => {
                unimplemented!()
            }
            VariantType::_RID => {
                unimplemented!()
            }
            VariantType::OBJECT => {
                unimplemented!()
            }
            VariantType::DICTIONARY => {
                unimplemented!()
            }
            VariantType::ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_BYTE_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_INT_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_REAL_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_STRING_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_VECTOR2_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_VECTOR3_ARRAY => {
                unimplemented!()
            }
            VariantType::POOL_COLOR_ARRAY => {
                unimplemented!()
            }
            VariantType::VARIANT_MAX => {
                unimplemented!()
            }
        }
    }

    #[cfg(test)]
    unimplemented!()
}

impl ffi::ComponentFieldDefinition {
    pub fn new(name: String, field_type: VariantType) -> Self {
        ffi::ComponentFieldDefinition { name, field_type }
    }
}

pub fn create_component_field_definition(
    name: String,
    field_type: VariantType,
) -> ffi::ComponentFieldDefinition {
    ffi::ComponentFieldDefinition::new(name, field_type)
}
