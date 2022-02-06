use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use uuid::Uuid;

use crate::component::ffi::ComponentFieldDefinition;
use crate::component::ComponentData;
use crate::ecs_world::ffi::ComponentInfo;

struct UuidCXX(uuid::Uuid);

#[cxx::bridge]
pub mod ffi {
    #[derive(Eq, PartialEq, Hash, Copy, Clone)]
    pub struct ComponentInfo {
        pub hash: u64,
    }

    extern "Rust" {
        include!("cxx.h");
        type ECSWorld;
        type ComponentFieldDefinition;
        #[cxx_name = "Uuid"]
        type UuidCXX;

        fn register_component(
            self: &mut ECSWorld,
            name: String,
            fields: Vec<ComponentFieldDefinition>,
        ) -> Result<ComponentInfo>;
    }
}

#[derive(Default)]
pub struct ECSWorld {
    component_definitions: HashMap<ComponentInfo, Vec<ComponentFieldDefinition>>,
    component_names: HashMap<String, ComponentInfo>,
    entities: Vec<Uuid>,
    components: HashMap<String, Vec<Rc<ComponentData>>>, // TODO: Rc maybe need to be Arc
    components_of_entity: HashMap<Uuid, HashMap<String, Rc<ComponentData>>>,
}

impl ECSWorld {
    pub fn register_component(
        &mut self,
        name: String,
        fields: Vec<ComponentFieldDefinition>,
    ) -> Result<ComponentInfo, String> {
        let mut hasher = DefaultHasher::default();
        fields.hash(&mut hasher);
        name.hash(&mut hasher);

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.component_names.entry(name.clone())
        {
            let info = ComponentInfo {
                hash: hasher.finish(),
            };
            self.component_definitions.entry(info).or_insert(fields);
            entry.insert(info);
            self.components.insert(name.clone(), Vec::new());
            Result::Ok(info)
        } else {
            Result::Err(format!(
                "Component with name \'{}\' already registered",
                name
            ))
        }
    }

    pub fn create_entity(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        self.entities.push(id);
        id
    }

    pub fn add_component_to_entity(
        &mut self,
        entity_id: Uuid,
        component: String,
    ) -> Result<(), String> {
        let components: &mut Vec<Rc<ComponentData>> = self.components.get_mut(&component).unwrap();

        if components.iter().any(|d| d.get_entity() == entity_id) {
            Err("Component was already added for that entity".to_string())
        } else {
            let data = Rc::new(ComponentData::new(entity_id));
            components.push(data.clone());
            let entity_components = self
                .components_of_entity
                .entry(entity_id)
                .or_insert_with(HashMap::new);

            entity_components.insert(component, data);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::ffi::ComponentFieldDefinition;
    use crate::ecs_world::ECSWorld;
    use crate::godot::variant::VariantType;

    #[test]
    fn register_component_adds_new_component_and_creates_a_hash() {
        let mut world = ECSWorld::default();
        let definition = vec![ComponentFieldDefinition::new(
            "Field".to_string(),
            VariantType::NIL,
        )];

        let component_name = "Test";
        let result = world.register_component(component_name.to_string(), definition.clone());
        assert!(result.is_ok(), "register_component should have returned Ok");

        let returned_info = result.unwrap();
        assert_ne!(0, returned_info.hash, "Hash should have a value");

        assert!(
            world.component_definitions.contains_key(&returned_info),
            "Component Information should have been stored in component_definitions"
        );
        let stored_definition = world.component_definitions.get(&returned_info).unwrap();
        assert_eq!(
            definition, *stored_definition,
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

        let definition_2 = vec![ComponentFieldDefinition::new(
            "Field".to_string(),
            VariantType::INT,
        )];

        let result = world.register_component(format!("{}_", component_name), definition_2);
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
        let definition = vec![ComponentFieldDefinition::new(
            "Field".to_string(),
            VariantType::NIL,
        )];

        let definition_2 = vec![ComponentFieldDefinition::new(
            "Field".to_string(),
            VariantType::INT,
        )];

        world
            .register_component("Test".to_string(), definition)
            .unwrap();
        let result = world.register_component("Test".to_string(), definition_2);
        assert!(result.is_err())
    }

    #[test]
    pub fn register_component_adds_a_component_with_an_existing_definition_under_a_different_name_with_a_unique_hash(
    ) {
        let mut world = ECSWorld::default();
        let definition = vec![ComponentFieldDefinition::new(
            "Field".to_string(),
            VariantType::NIL,
        )];

        let info_1 = world
            .register_component("Test".to_string(), definition.clone())
            .unwrap();
        let info_2 = world
            .register_component("Test2".to_string(), definition)
            .unwrap();

        assert_ne!(
            info_1.hash, info_2.hash,
            "Components with different names should have different hashes"
        );
    }

    #[test]
    pub fn create_entity_creates_a_new_entity() {
        let mut world = ECSWorld::default();

        let uuid_1 = world.create_entity();
        let uuid_2 = world.create_entity();

        assert_ne!(uuid_1, uuid_2, "Should have created unique entities");
    }

    #[test]
    pub fn add_component_to_entity_adds_a_new_component_to_an_entity() {
        let mut world = ECSWorld::default();
        let definition = vec![ComponentFieldDefinition::new(
            "Integer".to_string(),
            VariantType::INT,
        )];
        let component_name = "Test";
        world
            .register_component(component_name.to_string(), definition)
            .unwrap();

        let entity_id = world.create_entity();

        let result = world.add_component_to_entity(entity_id, component_name.to_string());

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
            data.get_entity(),
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

        let component_data = entity_components.get(component_name).unwrap();

        assert_eq!(
            entity_id,
            component_data.get_entity(),
            "Added component should belong to the entity that was passed"
        )
    }

    #[test]
    pub fn add_component_to_entity_does_not_allow_adding_the_same_component_twice_to_an_entity() {
        let mut world = ECSWorld::default();
        let definition = vec![ComponentFieldDefinition::new(
            "Integer".to_string(),
            VariantType::INT,
        )];
        let component_name = "Test";
        world
            .register_component(component_name.to_string(), definition)
            .unwrap();

        let entity_id = world.create_entity();

        world
            .add_component_to_entity(entity_id, component_name.to_string())
            .unwrap();

        let result = world.add_component_to_entity(entity_id, component_name.to_string());

        assert!(result.is_err(), "Result should have been Err");
    }
}
