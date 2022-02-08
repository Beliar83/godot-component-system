use crate::godot::variant::ffi::VariantType;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    pub struct ComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    extern "Rust" {
        include!("variant.rs.h");
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
