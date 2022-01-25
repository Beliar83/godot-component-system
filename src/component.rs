use crate::component::ffi::{string_name_from_rust_string, GodotComponent};
use crate::godot::string_name::ffi::StringName;
use crate::godot::variant::ffi::Variant;
use crate::godot::variant::ffi::VariantType;
use cxx::{SharedPtr, UniquePtr};
use std::pin::Pin;

#[cxx::bridge]
pub mod ffi {
    pub struct ComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    extern "Rust" {
        pub fn create_component_field_definition(
            name: String,
            field_type: VariantType,
        ) -> SharedPtr<ComponentFieldDefinition>;
        pub fn print_definition_gd(component: &GodotComponent);
    }

    unsafe extern "C++" {
        include!("object.h");
        include!("string_name.h");
        include!("cxx.h");
        type StringName = crate::godot::string_name::ffi::StringName;
        pub type Variant = crate::godot::variant::ffi::Variant;
        type VariantType = crate::godot::variant::ffi::VariantType;

        pub type GodotComponent;
        pub fn get_fields(self: &GodotComponent) -> Vec<ComponentFieldDefinition>;
        pub fn set_field(self: Pin<&mut GodotComponent>, name: &StringName, value: &Variant);
        pub fn get_field(self: &GodotComponent, name: &StringName) -> UniquePtr<Variant>;
        pub fn string_name_from_rust_string(string: String) -> &'static StringName;
    }
}

impl Into<&StringName> for String {
    fn into(self) -> &'static StringName {
        string_name_from_rust_string(self)
    }
}

pub fn create_component_field_definition(
    name: String,
    field_type: VariantType,
) -> SharedPtr<ffi::ComponentFieldDefinition> {
    SharedPtr::new(ffi::ComponentFieldDefinition { name, field_type })
}

impl Component for ffi::GodotComponent {
    fn get_fields(&self) -> Vec<ffi::ComponentFieldDefinition> {
        self.get_fields()
    }

    fn set_field(&mut self, name: &StringName, value: &ffi::Variant) {
        let component: Pin<&mut GodotComponent> = unsafe { Pin::new_unchecked(self) };
        component.set_field(name, value);
    }

    fn get_field(&self, name: &StringName) -> UniquePtr<ffi::Variant> {
        self.get_field(name)
    }
}

pub trait Component {
    fn get_fields(&self) -> Vec<ffi::ComponentFieldDefinition>;
    fn set_field(&mut self, name: &StringName, value: &Variant);
    fn get_field(&self, name: &StringName) -> UniquePtr<Variant>;
}

fn print_definition_gd(component: &ffi::GodotComponent) {
    print_definition(Box::new(component));
}

fn print_definition(component: Box<&dyn Component>) {
    let definitions = component.get_fields();
    for field in definitions {
        let name = field.name.clone();
        let value = component.get_field(field.name.into());
        let value: &Variant = value.as_ref().unwrap();
        let variant_type: VariantType = value.get_type();
        match variant_type {
            VariantType::NIL => {}
            VariantType::BOOL => {
                let value: bool = value.into();
                println!("{} = {}", name, value)
            }
            VariantType::INT => {
                let value: i64 = value.into();
                println!("{} = {}", name, value)
            }
            VariantType::REAL => {
                let value: f64 = value.into();
                println!("{} = {}", name, value)
            }
            VariantType::STRING => {
                let value: String = value.into();
                println!("{} = \"{}\"", name, value)
            }
            VariantType::VECTOR2 => {}
            VariantType::RECT2 => {}
            VariantType::VECTOR3 => {}
            VariantType::TRANSFORM2D => {}
            VariantType::PLANE => {}
            VariantType::QUAT => {}
            VariantType::AABB => {}
            VariantType::BASIS => {}
            VariantType::TRANSFORM => {}
            VariantType::COLOR => {}
            VariantType::NODE_PATH => {}
            VariantType::_RID => {}
            VariantType::OBJECT => {}
            VariantType::DICTIONARY => {}
            VariantType::ARRAY => {}
            VariantType::POOL_BYTE_ARRAY => {}
            VariantType::POOL_INT_ARRAY => {}
            VariantType::POOL_REAL_ARRAY => {}
            VariantType::POOL_STRING_ARRAY => {}
            VariantType::POOL_VECTOR2_ARRAY => {}
            VariantType::POOL_VECTOR3_ARRAY => {}
            VariantType::POOL_COLOR_ARRAY => {}
            VariantType::VARIANT_MAX => {}
        }
    }
}
