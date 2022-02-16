use crate::variant::VariantType;
use std::hash::Hash;

pub trait ComponentDefinition: Default + Hash + Clone {
    type FieldDefinition: ComponentFieldDefinition + Hash + Eq + PartialEq + Clone + Default;
    fn get_fields(&self) -> Vec<Self::FieldDefinition>;
    fn add_field(&mut self, field_definition: Self::FieldDefinition);
}

pub trait ComponentFieldDefinition: Default + Hash + Clone + Eq {
    fn get_type(&self) -> VariantType;
    fn get_name(&self) -> String;
}
