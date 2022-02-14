use cxx::{type_id, ExternType};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::component::component_data::create_component_data;
use crate::component::component_data::ComponentData;
use crate::component::component_definition::ComponentDefinition;
use crate::component::component_info::create_component_info;
use crate::component::component_info::ComponentInfo;
use crate::ecs_world::RegisterEntityError::AlreadyRegistered;
use crate::ecs_world::SetComponentDataError::{ComponentNotFound, DataInUse, EntityNotFound};
use crate::entity::create_entity;
use crate::entity::entity_id_from_string;
use crate::entity::EntityId;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {

    extern "Rust" {
        type ComponentInfo;
    }

    extern "Rust" {
        include!("component_value.rs.h");
        type ComponentData;
        fn get_field(self: &ComponentData, field: String) -> &ComponentValue;
        fn set_field(self: &mut ComponentData, field: String, value: &ComponentValue);
        fn create_component_data(entity: &EntityId) -> Box<ComponentData>;
    }
    extern "Rust" {
        include!("cxx.h");
        type EntityId;

        fn create_entity() -> Box<EntityId>;
        fn as_string(&self) -> String;
        fn entity_id_from_string(id: String) -> Result<Box<EntityId>>;
    }

    extern "Rust" {
        type ECSWorld;

        pub(crate) fn create_component_info(hash: u64) -> Box<ComponentInfo>;
        fn register_component(
            self: &mut ECSWorld,
            name: String,
            component_definition: &ComponentDefinition,
        ) -> Result<Box<ComponentInfo>>;

        fn register_entity(self: &mut ECSWorld, id: &EntityId) -> Result<()>;

        pub fn set_component_data(
            self: &mut ECSWorld,
            entity_id: &EntityId,
            component: String,
            data: &ComponentData,
        ) -> Result<()>;

        fn is_component_added_to_entity(
            self: &ECSWorld,
            entity_id: &EntityId,
            component: String,
        ) -> bool;

        fn get_components_of_entity(self: &ECSWorld, entity_id: &EntityId) -> Result<Vec<String>>;

        fn create_entity(self: &mut ECSWorld) -> Box<EntityId>;

        pub fn create_ecs_world() -> Box<ECSWorld>;
    }

    extern "C++" {
        include!("cxx.h");
        include!("godot/variant.h");
        include!("component_definition.rs.h");
        include!("component_value.rs.h");

        type ComponentFieldDefinition =
            crate::component::component_definition::ffi::ComponentFieldDefinition;
        type ComponentDefinition = crate::component::component_definition::ComponentDefinition;
        type ComponentValue = crate::component::component_value::ComponentValue;

        pub type Variant = crate::godot::variant::ffi::Variant;
    }
}

#[derive(PartialEq, Debug)]
pub enum GetComponentsOfEntityError {
    EntityNotFound,
}

impl Display for GetComponentsOfEntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetComponentsOfEntityError::EntityNotFound => {
                write!(f, "Entity with that id is was not found")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SetComponentDataError {
    EntityNotFound,
    ComponentNotFound,
    DataInUse,
}

impl Display for SetComponentDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityNotFound => {
                write!(f, "Entity with that id is was not found")
            }
            ComponentNotFound => {
                write!(f, "Component with that name is already registered")
            }
            DataInUse => {
                write!(f, "The data is already exclusively borrowed")
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum RegisterEntityError {
    AlreadyRegistered,
}

impl Display for RegisterEntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlreadyRegistered => {
                write!(f, "Entity with that id is already registered")
            }
        }
    }
}

#[derive(Default)]
pub struct ECSWorld {
    component_definitions: HashMap<ComponentInfo, ComponentDefinition>,
    component_names: HashMap<String, ComponentInfo>,
    entities: Vec<EntityId>,
    components: HashMap<String, Vec<Rc<RefCell<ComponentData>>>>,
    components_of_entity: HashMap<EntityId, HashMap<String, Rc<RefCell<ComponentData>>>>,
}

pub fn create_ecs_world() -> Box<ECSWorld> {
    Box::new(ECSWorld::default())
}

unsafe impl ExternType for ECSWorld {
    type Id = type_id!("gcs::ffi::ECSWorld");
    type Kind = cxx::kind::Trivial;
}

impl ECSWorld {
    pub fn register_component(
        &mut self,
        name: String,
        component_definition: &ComponentDefinition,
    ) -> Result<Box<ComponentInfo>, String> {
        let mut hasher = DefaultHasher::default();
        component_definition.hash(&mut hasher);
        name.hash(&mut hasher);

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.component_names.entry(name.clone())
        {
            let info = ComponentInfo {
                hash: hasher.finish(),
            };
            self.component_definitions
                .entry(info)
                .or_insert_with(|| component_definition.clone());
            entry.insert(info);
            self.components.insert(name.clone(), Vec::new());
            Result::Ok(Box::new(info))
        } else {
            Result::Err(format!(
                "Component with name \'{}\' already registered",
                name
            ))
        }
    }

    pub fn create_entity(&mut self) -> Box<EntityId> {
        let id = EntityId::create();
        self.entities.push(id);
        Box::new(*self.entities.last().unwrap())
    }

    pub fn register_entity(&mut self, id: &EntityId) -> Result<(), RegisterEntityError> {
        if self.entities.contains(id) {
            Err(AlreadyRegistered)
        } else {
            self.entities.push(*id);
            Ok(())
        }
    }

    pub fn add_component_to_entity(
        &mut self,
        entity_id: &EntityId,
        component: String,
    ) -> Result<(), String> {
        let components = self.components.get_mut(&component).unwrap();

        if components
            .iter()
            .any(|d| d.borrow().get_entity() == *entity_id)
        {
            Err("Component was already added for that entity".to_string())
        } else {
            let value = RefCell::new(ComponentData::new(*entity_id));
            let data = Rc::new(value);
            components.push(data.clone());

            let entity_components = self
                .components_of_entity
                .entry(*entity_id)
                .or_insert_with(HashMap::new);

            entity_components.insert(component, data);
            Ok(())
        }
    }

    pub fn set_component_data(
        &mut self,
        entity_id: &EntityId,
        component: String,
        data: &ComponentData,
    ) -> Result<(), SetComponentDataError> {
        if !self.entities.contains(entity_id) {
            Err(EntityNotFound)
        } else if !self.components.contains_key(&component) {
            Err(ComponentNotFound)
        } else {
            if self
                .components_of_entity
                .get(entity_id)
                .and_then(|c| c.get(&component))
                .is_none()
            {
                self.add_component_to_entity(entity_id, component.clone())
                    .unwrap();
            }

            let stored_data = self
                .components_of_entity
                .get_mut(entity_id)
                .and_then(|c| c.get_mut(&component))
                .unwrap();

            let mut stored_data = match { stored_data.try_borrow_mut() } {
                Ok(data) => data,
                Err(_) => return Err(DataInUse), // TODO: Can this be tested?
            };

            let component_information = self.component_names.get(&component).unwrap();

            let component_fields = self
                .component_definitions
                .get(component_information)
                .unwrap();

            for component_field in &component_fields.fields {
                let new_data = data.get_field(component_field.name.clone());
                stored_data.set_field(component_field.name.clone(), &new_data.clone());
            }
            Ok(())
        }
    }

    pub fn is_component_added_to_entity(&self, entity_id: &EntityId, component: String) -> bool {
        self.components_of_entity
            .get(entity_id)
            .map_or_else(|| false, |c| c.contains_key(&component))
    }

    pub fn get_components_of_entity(
        &self,
        entity_id: &EntityId,
    ) -> Result<Vec<String>, GetComponentsOfEntityError> {
        if self.components_of_entity.contains_key(entity_id) {
            Ok(self
                .components_of_entity
                .get(entity_id)
                .unwrap()
                .keys()
                .cloned()
                .collect())
        } else if self.entities.contains(entity_id) {
            Ok(Vec::new())
        } else {
            Err(GetComponentsOfEntityError::EntityNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::component_data::ComponentData;
    use crate::component::component_definition::ffi::ComponentFieldDefinition;
    use crate::component::component_definition::ComponentDefinition;
    use crate::component::component_value::ComponentValue;
    use crate::ecs_world::ECSWorld;
    use crate::ecs_world::SetComponentDataError::{ComponentNotFound, EntityNotFound};
    use crate::entity::{create_entity, EntityId};
    use crate::godot::variant::VariantType;

    #[test]
    fn get_components_of_entity_returns_components_of_the_passed_entity() {
        let mut world = ECSWorld::default();

        let field_definition = ComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name_1 = "Test";
        world
            .register_component(component_name_1.to_string(), &component_definition)
            .unwrap();

        let component_name_2 = "Test 2";
        world
            .register_component(component_name_2.to_string(), &component_definition)
            .unwrap();

        let entity_id = *world.create_entity();

        world
            .add_component_to_entity(&entity_id, component_name_1.to_string())
            .unwrap();
        world
            .add_component_to_entity(&entity_id, component_name_2.to_string())
            .unwrap();

        let components = world.get_components_of_entity(&entity_id);
        assert!(components.is_ok(), "Should have returned Ok");
        let components = components.unwrap();

        assert!(
            components.iter().any(|c| c == component_name_1),
            "Should have included the {} component in the list",
            component_name_1
        );
        assert!(
            components.iter().any(|c| c == component_name_2),
            "Should have included the {} component in the list",
            component_name_2
        );
    }

    #[test]
    fn get_components_returns_ok_if_entity_exists_but_never_had_components_added() {
        let mut world = ECSWorld::default();
        let entity = world.create_entity();

        let components = world.get_components_of_entity(&entity);
        assert!(components.is_ok());
    }

    #[test]
    fn get_components_returns_err_if_entity_does_not_exist() {
        let world = ECSWorld::default();

        let components = world.get_components_of_entity(&create_entity());
        assert!(components.is_err());
    }

    #[test]
    fn register_component_adds_new_component_and_creates_a_hash() {
        let mut world = ECSWorld::default();
        let mut component_definition = ComponentDefinition::default();

        let field_definition = ComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        component_definition.add_field(field_definition);

        let component_name = "Test";
        let result = world.register_component(component_name.to_string(), &component_definition);
        assert!(result.is_ok(), "register_component should have returned Ok");

        let returned_info = *result.unwrap();
        assert_ne!(0, returned_info.hash, "Hash should have a value");

        assert!(
            world.component_definitions.contains_key(&returned_info),
            "Component Information should have been stored in component_definitions"
        );
        let stored_definition = world.component_definitions.get(&returned_info).unwrap();
        assert_eq!(
            component_definition, *stored_definition,
            "Stored definition should be the same as the one added"
        );

        assert!(
            world
                .component_names
                .contains_key(&component_name.to_string()),
            "component_names should have the passed name as a key"
        );
        let stored_info = world.component_names.get(component_name).unwrap();
        assert_eq!(
            returned_info.hash, stored_info.hash,
            "Hash of stored info should be the same as the one returned"
        );

        assert!(
            world.components.contains_key(component_name),
            "A entry for the new component in components should have been added"
        );

        let field_definition_2 = ComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = ComponentDefinition::default();

        component_definition_2.add_field(field_definition_2);

        let result =
            world.register_component(format!("{}_", component_name), &component_definition_2);
        assert!(result.is_ok(), "Should have added a new component");

        let info_2 = result.unwrap();
        assert_ne!(
            returned_info.hash, info_2.hash,
            "Hash of different components should differ"
        );
    }

    #[test]
    pub fn register_component_does_not_allow_adding_of_a_component_with_a_name_that_already_exists()
    {
        let mut world = ECSWorld::default();
        let field_definition = ComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let field_definition_2 = ComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = ComponentDefinition::default();
        component_definition_2.add_field(field_definition_2);

        world
            .register_component("Test".to_string(), &component_definition)
            .unwrap();
        let result = world.register_component("Test".to_string(), &component_definition_2);
        assert!(result.is_err())
    }

    #[test]
    pub fn register_component_adds_a_component_with_an_existing_definition_under_a_different_name_with_a_unique_hash(
    ) {
        let mut world = ECSWorld::default();
        let field_definition = ComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let info_1 = *world
            .register_component("Test".to_string(), &component_definition.clone())
            .unwrap();
        let info_2 = *world
            .register_component("Test2".to_string(), &component_definition)
            .unwrap();

        assert_ne!(
            info_1.hash, info_2.hash,
            "Components with different names should have different hashes"
        );
    }

    #[test]
    pub fn create_entity_creates_a_new_entity() {
        let mut world = ECSWorld::default();

        let uuid_1 = *world.create_entity();
        let uuid_2 = *world.create_entity();

        assert_ne!(uuid_1, uuid_2, "Should have created unique entities");
    }

    #[test]
    pub fn add_component_to_entity_adds_a_new_component_to_an_entity() {
        let mut world = ECSWorld::default();
        let field_definition = ComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), &component_definition)
            .unwrap();

        let entity_id = *world.create_entity();

        let result = world.add_component_to_entity(&entity_id, component_name.to_string());

        assert!(result.is_ok(), "Result should have been Ok");
        drop(result);

        let components = world.components.get(component_name).unwrap();
        assert_eq!(
            1,
            components.len(),
            "Should have added a component to components"
        );

        let data = components.first().unwrap();
        assert_eq!(
            entity_id,
            data.borrow().get_entity(),
            "Added component should belong to the entity that was passed"
        );

        assert!(
            world.components_of_entity.contains_key(&entity_id),
            "components_of_entity should have an entry for the given entity"
        );
        let entity_components = world.components_of_entity.get(&entity_id).unwrap();

        assert!(
            entity_components.contains_key(component_name),
            "components_of_entity for the given entity should have an entry with the component_name"
        );

        let stored_component = entity_components.get(component_name).unwrap();

        assert_eq!(
            entity_id,
            stored_component.borrow().get_entity(),
            "Added component should belong to the entity that was passed"
        );
    }

    #[test]
    pub fn add_component_to_entity_does_not_allow_adding_the_same_component_twice_to_an_entity() {
        let mut world = ECSWorld::default();
        let field_definition = ComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), &component_definition)
            .unwrap();

        let entity_id = *world.create_entity();

        world
            .add_component_to_entity(&entity_id, component_name.to_string())
            .unwrap();

        let result = world.add_component_to_entity(&entity_id, component_name.to_string());

        assert!(result.is_err(), "Result should have been Err");
    }

    #[test]
    pub fn set_component_data_adds_component_data() {
        let mut world = ECSWorld::default();
        let field_name = "Integer";
        let field_definition = ComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), &component_definition)
            .unwrap();

        let entity_id = *world.create_entity();

        world
            .add_component_to_entity(&entity_id, component_name.to_string())
            .unwrap();

        let mut data = ComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &ComponentValue::Int(value));

        assert!(
            world
                .set_component_data(&entity_id, component_name.to_string(), &data)
                .is_ok(),
            "set_component_data should have returned Ok"
        );

        let components = world.components_of_entity.get(&entity_id).unwrap();

        let stored_data = components.get(component_name).unwrap().borrow();

        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            ComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );

        let stored_data = world
            .components
            .get(component_name)
            .and_then(|c| c.first())
            .unwrap()
            .borrow();
        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            ComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );
    }

    #[test]
    pub fn set_component_checks_that_entity_exists() {
        let mut world = ECSWorld::default();
        let entity_id = EntityId::create();
        let data = ComponentData::new(entity_id);

        assert_eq!(
            Err(EntityNotFound),
            world.set_component_data(&entity_id, "Test".to_string(), &data)
        );
    }

    #[test]
    pub fn set_component_checks_that_component_exists() {
        let mut world = ECSWorld::default();

        let entity_id = *world.create_entity();
        let data = ComponentData::new(entity_id);
        assert_eq!(
            Err(ComponentNotFound),
            world.set_component_data(&entity_id, "Test".to_string(), &data)
        );
    }

    #[test]
    pub fn set_component_adds_component_to_entity_if_it_is_not_added() {
        let mut world = ECSWorld::default();
        let field_definition = ComponentFieldDefinition {
            name: "Boolean".to_string(),
            field_type: VariantType::Bool,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let field_name = "Integer";
        let field_definition_2 = ComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = ComponentDefinition::default();
        component_definition_2.add_field(field_definition_2);

        let component_name = "Test_2";
        world
            .register_component("Test_1".to_string(), &component_definition)
            .unwrap();
        world
            .register_component(component_name.to_string(), &component_definition_2)
            .unwrap();

        let entity_id = *world.create_entity();
        world.add_component_to_entity(&entity_id, "Test_1".to_string());

        let mut data = ComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &ComponentValue::Int(value));

        assert!(world
            .set_component_data(&entity_id, component_name.to_string(), &data)
            .is_ok());
    }
    #[test]
    pub fn set_component_adds_initializes_entity_components_if_not_present() {
        let mut world = ECSWorld::default();
        let field_name = "Integer";
        let field_definition = ComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = ComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), &component_definition)
            .unwrap();

        let entity_id = *world.create_entity();
        let mut data = ComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &ComponentValue::Int(value));

        world
            .set_component_data(&entity_id, component_name.to_string(), &data)
            .unwrap();

        assert_eq!(1, world.components_of_entity.get(&entity_id).unwrap().len())
    }
}
