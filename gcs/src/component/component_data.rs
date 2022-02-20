use crate::component::component_value::ComponentValue;
use crate::entity::EntityId;

pub trait ComponentData: Default + Clone {
    type EntityId: EntityId;
    type ComponentValue: ComponentValue;

    fn new(entity: &Self::EntityId) -> Self;
    fn get_entity(&self) -> &Self::EntityId;
    fn get_field(&self, field: String) -> &Self::ComponentValue;
    fn set_field(&mut self, field: String, value: &Self::ComponentValue);
}
