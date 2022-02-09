use crate::component::component_definition::ffi::ComponentFieldDefinition;
use crate::godot::variant::ffi::VariantType;
use cxx::{type_id, ExternType};

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub struct ComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    extern "Rust" {
        include!("variant.rs.h");
        type ComponentDefinition;

        pub fn create_component_definition() -> Box<ComponentDefinition>;

        pub fn add_field(
            self: &mut ComponentDefinition,
            field_definition: ComponentFieldDefinition,
        );

        pub fn create_component_field_definition(
            name: String,
            field_type: VariantType,
        ) -> ComponentFieldDefinition;

    }

    extern "C++" {
        type VariantType = crate::godot::variant::VariantType;
    }
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ComponentDefinition {
    pub fields: Vec<ffi::ComponentFieldDefinition>,
}

unsafe impl ExternType for ComponentDefinition {
    type Id = type_id!("gcs::ffi::ComponentDefinition");
    type Kind = cxx::kind::Opaque;
}

impl Default for ComponentDefinition {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

impl ComponentDefinition {
    pub fn add_field(&mut self, field_definition: ComponentFieldDefinition) {
        self.fields.push(field_definition);
    }
}

pub fn create_component_definition() -> Box<ComponentDefinition> {
    Box::new(ComponentDefinition::default())
}
