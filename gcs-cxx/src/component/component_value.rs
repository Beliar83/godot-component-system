use crate::component::component_value::ffi::{
    empty_variant, variant_from_bool, variant_from_f64, variant_from_i64, variant_from_string,
};
use crate::godot::variant::ffi::{
    variant_as_bool, variant_as_f64, variant_as_i64, variant_as_string, CXXVariantType, Variant,
};
use cxx::{type_id, ExternType};
use gcs::component::component_value::ComponentValue;
use gcs::variant::VariantType;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    extern "Rust" {
        #[cxx_name = "ComponentValue"]
        type CXXComponentValue;
        fn variant_from_component_value(value: &CXXComponentValue) -> &'static Variant;
        fn component_value_from_variant(value: &Variant) -> Box<CXXComponentValue>;
    }

    unsafe extern "C++" {
        include!("gcs-cxx/include/godot/variant.h");
        include!("rust/cxx.h");
        pub type Variant = crate::godot::variant::ffi::Variant;

        pub(crate) fn empty_variant() -> &'static Variant;
        pub(crate) fn variant_from_i64(value: i64) -> &'static Variant;
        pub(crate) fn variant_from_string(value: String) -> &'static Variant;
        pub(crate) fn variant_from_bool(value: bool) -> &'static Variant;
        pub(crate) fn variant_from_f64(value: f64) -> &'static Variant;

    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CXXComponentValue {
    Nil,
    Int(i64),
    String(String),
    Bool(bool),
    Real(f64),
}

impl Default for CXXComponentValue {
    fn default() -> Self {
        CXXComponentValue::Nil
    }
}

impl ComponentValue for CXXComponentValue {
    fn get_type(&self) -> VariantType {
        match self {
            CXXComponentValue::Nil => VariantType::Nil,
            CXXComponentValue::Int(_) => VariantType::Int,
            CXXComponentValue::String(_) => VariantType::String,
            CXXComponentValue::Bool(_) => VariantType::Bool,
            CXXComponentValue::Real(_) => VariantType::Real,
        }
    }

    fn set_nil(&mut self) {
        *self = CXXComponentValue::Nil;
    }

    fn get_nil(&self) -> () {}

    fn set_int(&mut self, value: i64) {
        *self = CXXComponentValue::Int(value)
    }

    fn get_int(&self) -> i64 {
        match self {
            CXXComponentValue::Nil => 0,
            CXXComponentValue::Int(value) => *value,
            CXXComponentValue::String(value) => {
                let result = str::parse::<i64>(value);
                match result {
                    Ok(value) => value,
                    Err(_) => 0,
                }
            }
            CXXComponentValue::Bool(value) => {
                if *value {
                    1
                } else {
                    0
                }
            }
            CXXComponentValue::Real(value) => *value as i64,
        }
    }

    fn set_string(&mut self, value: String) {
        *self = CXXComponentValue::String(value)
    }

    fn get_string(&self) -> String {
        match self {
            CXXComponentValue::Nil => "".to_string(),
            CXXComponentValue::Int(value) => value.to_string(),
            CXXComponentValue::String(value) => value.clone(),
            CXXComponentValue::Bool(value) => value.to_string(),
            CXXComponentValue::Real(value) => value.to_string(),
        }
    }

    fn set_bool(&mut self, value: bool) {
        *self = CXXComponentValue::Bool(value)
    }

    fn get_bool(&self) -> bool {
        match self {
            CXXComponentValue::Nil => false,
            CXXComponentValue::Int(value) => *value != 0,
            CXXComponentValue::String(value) => value.is_empty(),
            CXXComponentValue::Bool(value) => *value,
            CXXComponentValue::Real(value) => *value != 0.0,
        }
    }

    fn set_real(&mut self, value: f64) {
        *self = CXXComponentValue::Real(value)
    }

    fn get_real(&self) -> f64 {
        match self {
            CXXComponentValue::Nil => 0.0,
            CXXComponentValue::Int(value) => *value as f64,
            CXXComponentValue::String(value) => {
                let result = str::parse::<f64>(value);
                match result {
                    Ok(value) => value,
                    Err(_) => 0.0,
                }
            }
            CXXComponentValue::Bool(value) => {
                if *value {
                    1.0
                } else {
                    0.0
                }
            }
            CXXComponentValue::Real(value) => *value,
        }
    }
}

unsafe impl ExternType for CXXComponentValue {
    type Id = type_id!("gcs::ffi::ComponentValue");
    type Kind = cxx::kind::Opaque;
}

fn variant_from_component_value(value: &CXXComponentValue) -> &'static Variant {
    match value.clone() {
        CXXComponentValue::Nil => empty_variant(),
        CXXComponentValue::Int(value) => variant_from_i64(value),
        CXXComponentValue::String(value) => variant_from_string(value.clone()),
        CXXComponentValue::Bool(value) => variant_from_bool(value),
        CXXComponentValue::Real(value) => variant_from_f64(value),
    }
}

fn component_value_from_variant(value: &Variant) -> Box<CXXComponentValue> {
    let variant_type: CXXVariantType = value.get_type();

    match variant_type.0 {
        VariantType::Nil => Box::new(CXXComponentValue::Nil),
        VariantType::Bool => Box::new(CXXComponentValue::Bool(variant_as_bool(value))),
        VariantType::Int => Box::new(CXXComponentValue::Int(variant_as_i64(value))),
        VariantType::Real => Box::new(CXXComponentValue::Real(variant_as_f64(value))),
        VariantType::String => Box::new(CXXComponentValue::String(variant_as_string(value))),
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
        VariantType::Aaab => {
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
