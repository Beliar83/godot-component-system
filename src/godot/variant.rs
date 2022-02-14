use crate::godot::variant::ffi::Variant;
use crate::godot::variant::VariantType::Nil;
use cxx::{type_id, ExternType};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum VariantType {
    Nil,
    // atomic types
    Bool,
    Int,
    Real,
    String,

    // math types
    Vector2, // 5
    Rect2,
    Vector3,
    Transform2D,
    Plan,
    Quat, // 10
    Aaab,
    Basis,
    Transform,

    // misc types
    Color,
    NodePath, // 15
    _RID,
    Object,
    Dictionary,
    Array,

    // arrays
    PoolByteArray, // 20
    PoolIntArray,
    PoolRealArray,
    PoolStringArray,
    PoolVector2Array,
    PoolVector3Array, // 25
    PoolColorArray,

    VariantMax,
}

impl Default for VariantType {
    fn default() -> Self {
        Nil
    }
}

unsafe impl ExternType for VariantType {
    type Id = type_id!("gcs::ffi::VariantType");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    unsafe extern "C++" {
        include!("../../include/godot/variant.h");
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
