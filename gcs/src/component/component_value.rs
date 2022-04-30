use godot_cxx_common::variant_type::VariantType;
use std::fmt::Debug;

pub trait ComponentValue: Clone + PartialEq + Debug {
    fn get_type(&self) -> VariantType;
    fn set_nil(&mut self);
    fn get_nil(&self) -> ();
    fn set_int(&mut self, value: i64);
    fn get_int(&self) -> i64;
    fn set_string(&mut self, value: String);
    fn get_string(&self) -> String;
    fn set_bool(&mut self, value: bool);
    fn get_bool(&self) -> bool;
    fn set_real(&mut self, value: f64);
    fn get_real(&self) -> f64;
}
