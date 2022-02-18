use crate::component::component_value::ComponentValue;
use crate::entity::EntityId;

pub trait ComponentData: Default + Clone {
    type EntityIdType: EntityId;
    type ComponentValueType: ComponentValue;

    fn new(entity: Self::EntityIdType) -> Self;
    fn get_entity(&self) -> Self::EntityIdType;
    fn get_field(&self, field: String) -> &Self::ComponentValueType;
    fn set_field(&mut self, field: String, value: &Self::ComponentValueType);
}
