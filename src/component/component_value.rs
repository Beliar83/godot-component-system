use crate::component::component_value::ffi::{
    empty_variant, variant_from_bool, variant_from_f64, variant_from_i64, variant_from_string,
};
use crate::godot::variant::ffi::{
    variant_as_bool, variant_as_f64, variant_as_i64, variant_as_string, Variant, VariantType,
};
use cxx::{type_id, ExternType};

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    extern "Rust" {
        type ComponentValue;

        fn variant_from_component_value(value: &ComponentValue) -> &'static Variant;
        fn component_value_from_variant(value: &Variant) -> Box<ComponentValue>;
    }

    unsafe extern "C++" {
        include!("component.h");
        include!("cxx.h");
        pub type Variant = crate::godot::variant::ffi::Variant;

        pub(crate) fn empty_variant() -> &'static Variant;
        pub(crate) fn variant_from_i64(value: i64) -> &'static Variant;
        pub(crate) fn variant_from_string(value: String) -> &'static Variant;
        pub(crate) fn variant_from_bool(value: bool) -> &'static Variant;
        pub(crate) fn variant_from_f64(value: f64) -> &'static Variant;

    }
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

unsafe impl ExternType for ComponentValue {
    type Id = type_id!("gcs::ffi::ComponentValue");
    type Kind = cxx::kind::Opaque;
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
            VariantType::Nil => Box::new(ComponentValue::Nil),
            VariantType::Bool => Box::new(ComponentValue::Bool(variant_as_bool(value))),
            VariantType::Int => Box::new(ComponentValue::Int(variant_as_i64(value))),
            VariantType::Real => Box::new(ComponentValue::Real(variant_as_f64(value))),
            VariantType::String => Box::new(ComponentValue::String(variant_as_string(value))),
            VariantType::Vector2 => {
                unimplemented!()
            }
            VariantType::Rect2 => {
                unimplemented!()
            }
            VariantType::Vector3 => {
                unimplemented!()
            }
            VariantType::Transform2D => {
                unimplemented!()
            }
            VariantType::Plan => {
                unimplemented!()
            }
            VariantType::Quat => {
                unimplemented!()
            }
            VariantType::AABB => {
                unimplemented!()
            }
            VariantType::Basis => {
                unimplemented!()
            }
            VariantType::Transform => {
                unimplemented!()
            }
            VariantType::Color => {
                unimplemented!()
            }
            VariantType::NodePath => {
                unimplemented!()
            }
            VariantType::_RID => {
                unimplemented!()
            }
            VariantType::Object => {
                unimplemented!()
            }
            VariantType::Dictionary => {
                unimplemented!()
            }
            VariantType::Array => {
                unimplemented!()
            }
            VariantType::PoolByteArray => {
                unimplemented!()
            }
            VariantType::PoolIntArray => {
                unimplemented!()
            }
            VariantType::PoolRealArray => {
                unimplemented!()
            }
            VariantType::PoolStringArray => {
                unimplemented!()
            }
            VariantType::PoolVector2Array => {
                unimplemented!()
            }
            VariantType::PoolVector3Array => {
                unimplemented!()
            }
            VariantType::PoolColorArray => {
                unimplemented!()
            }
            VariantType::VariantMax => {
                unimplemented!()
            }
        }
    }

    #[cfg(test)]
    unimplemented!()
}
