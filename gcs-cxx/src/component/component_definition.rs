use crate::component::component_definition::ffi::CXXComponentFieldDefinition;
use cxx::{type_id, ExternType};
use gcs::component::component_definition::{ComponentDefinition, ComponentFieldDefinition};
use gcs::variant::VariantType;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    #[derive(Hash, Eq, PartialEq, Clone, Default)]
    #[cxx_name = "ComponentFieldDefinition"]
    pub struct CXXComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    extern "Rust" {
        include!("gcs-cxx/src/godot/variant.rs.h");
        #[cxx_name = "ComponentDefinition"]
        type CXXComponentDefinition;

        pub fn create_component_definition() -> Box<CXXComponentDefinition>;

        pub fn add_field(
            self: &mut CXXComponentDefinition,
            field_definition: CXXComponentFieldDefinition,
        );

        pub fn create_component_field_definition() -> CXXComponentFieldDefinition;

    }

    extern "C++" {
        type VariantType = crate::godot::variant::CXXVariantType;
    }
}

impl ComponentFieldDefinition for CXXComponentFieldDefinition {
    fn get_type(&self) -> VariantType {
        self.field_type.0
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub fn create_component_field_definition() -> ffi::CXXComponentFieldDefinition {
    ffi::CXXComponentFieldDefinition::default()
}

#[derive(Hash, Eq, PartialEq, Clone, Default)]
pub struct CXXComponentDefinition {
    pub fields: Vec<ffi::CXXComponentFieldDefinition>,
}

unsafe impl ExternType for CXXComponentDefinition {
    type Id = type_id!("gcs::ffi::ComponentDefinition");
    type Kind = cxx::kind::Opaque;
}

impl ComponentDefinition for CXXComponentDefinition {
    type FieldDefinition = CXXComponentFieldDefinition;

    fn get_fields(&self) -> Vec<Self::FieldDefinition> {
        self.fields.clone()
    }

    fn add_field(&mut self, field_definition: Self::FieldDefinition) {
        self.fields.push(field_definition);
    }
}

impl CXXComponentDefinition {
    fn add_field(&mut self, field_definition: CXXComponentFieldDefinition) {
        ComponentDefinition::add_field(self, field_definition);
    }
}

pub fn create_component_definition() -> Box<CXXComponentDefinition> {
    Box::new(CXXComponentDefinition::default())
}
