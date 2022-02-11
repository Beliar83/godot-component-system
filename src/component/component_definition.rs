use crate::component::component_definition::ffi::ComponentFieldDefinition;
use cxx::{type_id, ExternType};

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {

    #[derive(Hash, Eq, PartialEq, Debug, Clone, Default)]
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

        pub fn create_component_field_definition() -> ComponentFieldDefinition;

    }

    extern "C++" {
        type VariantType = crate::godot::variant::VariantType;
    }
}

pub fn create_component_field_definition() -> ffi::ComponentFieldDefinition {
    ffi::ComponentFieldDefinition::default()
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
