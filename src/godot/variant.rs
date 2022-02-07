use crate::godot::variant::ffi::Variant;
use cxx::{type_id, ExternType};

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum VariantType {
    NIL,
    // atomic types
    BOOL,
    INT,
    REAL,
    STRING,

    // math types
    VECTOR2, // 5
    RECT2,
    VECTOR3,
    TRANSFORM2D,
    PLANE,
    QUAT, // 10
    AABB,
    BASIS,
    TRANSFORM,

    // misc types
    COLOR,
    NODE_PATH, // 15
    _RID,
    OBJECT,
    DICTIONARY,
    ARRAY,

    // arrays
    POOL_BYTE_ARRAY, // 20
    POOL_INT_ARRAY,
    POOL_REAL_ARRAY,
    POOL_STRING_ARRAY,
    POOL_VECTOR2_ARRAY,
    POOL_VECTOR3_ARRAY, // 25
    POOL_COLOR_ARRAY,

    VARIANT_MAX,
}

unsafe impl ExternType for VariantType {
    type Id = type_id!("gcs::ffi::VariantType");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    unsafe extern "C++" {
        include!("../../include/variant.h");
        include!("cxx.h");
        pub type Variant;
        pub type VariantType = crate::godot::variant::VariantType;

        pub fn get_type(self: &Variant) -> VariantType;

        pub fn variant_as_i64(variant: &Variant) -> i64;
        pub fn variant_as_string(variant: &Variant) -> String;
        pub fn variant_as_bool(variant: &Variant) -> bool;
        pub fn variant_as_f64(variant: &Variant) -> f64;
    }
}

impl From<&ffi::Variant> for i64 {
    fn from(variant: &Variant) -> Self {
        ffi::variant_as_i64(variant)
    }
}

impl From<&ffi::Variant> for String {
    fn from(variant: &Variant) -> Self {
        ffi::variant_as_string(variant)
    }
}

impl From<&ffi::Variant> for bool {
    fn from(variant: &Variant) -> Self {
        ffi::variant_as_bool(variant)
    }
}

impl From<&ffi::Variant> for f64 {
    fn from(variant: &Variant) -> Self {
        ffi::variant_as_f64(variant)
    }
}
