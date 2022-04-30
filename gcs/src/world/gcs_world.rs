use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

use crate::component::component_data::ComponentData;
use crate::entity::EntityId;
use crate::world::component_data_storage::{ComponentDataStorage, HasEntityComponentResult};
use crate::world::component_storage::ComponentStorage;
use crate::world::errors::RegisterEntityError::AlreadyRegistered;
use crate::world::errors::SetComponentDataError::{ComponentNotFound, EntityNotFound};
use crate::world::errors::{
    GetComponentDataOfEntityError, RegisterEntityError, SetComponentDataError,
};

#[derive(Default)]
pub struct GCSWorld<TComponentStorage, TComponentDataStorage>
where
    TComponentStorage: ComponentStorage,
    TComponentDataStorage: ComponentDataStorage,
{
    component_storage: TComponentStorage,
    entities: Vec<
        <<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    >,
    component_data: TComponentDataStorage,
}

impl<TComponentStorage: ComponentStorage, TComponentDataStorage: ComponentDataStorage>
    GCSWorld<TComponentStorage, TComponentDataStorage>
{
    pub fn register_component(
        &mut self,
        name: &str,
        component_definition: TComponentStorage::ComponentDefinition,
    ) -> Result<TComponentStorage::ComponentInfo, String> {
        let mut hasher = DefaultHasher::default();
        component_definition.hash(&mut hasher);
        name.hash(&mut hasher);

        let result = self
            .component_storage
            .add_component(name, component_definition);

        match result {
            Ok(info) => Ok(info),
            Err(_) => Result::Err(format!(
                "Component with name \'{}\' already registered",
                name
            )),
        }
    }

    pub fn get_component_data(
        &self,
        name: &str,
    ) -> Option<Vec<&<TComponentDataStorage as ComponentDataStorage>::ComponentData>> {
        self.component_data.get_all_of_component(name)
    }

    pub fn has_component(&self, name: &str) -> bool {
        self.component_storage.has_component(name)
    }

    pub fn create_entity(
        &mut self,
    ) -> &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId
    {
        let id = EntityId::create();
        self.entities.push(id);
        self.entities.last().unwrap()
    }

    pub fn register_entity(
        &mut self,
        id: &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Result<(), RegisterEntityError> {
        if self.entities.contains(id) {
            Err(AlreadyRegistered)
        } else {
            self.entities.push(id.clone());
            Ok(())
        }
    }

    pub fn set_component_data(
        &mut self,
        entity_id: &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component: &str,
        data: &<TComponentDataStorage as ComponentDataStorage>::ComponentData,
    ) -> Result<(), SetComponentDataError> {
        if !self.entities.contains(entity_id) {
            Err(EntityNotFound)
        } else if !self.component_storage.has_component(&component) {
            Err(ComponentNotFound)
        } else {
            self.component_data
                .set_component_of_entity(entity_id, component, data);
            Ok(())
        }
    }

    pub fn is_component_added_to_entity(
        &self,
        entity_id: &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component: &str,
    ) -> HasEntityComponentResult {
        self.component_data
            .does_entity_have_component(entity_id, component)
    }

    pub fn get_components_of_entity(
        &self,
        entity_id: &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Result<Vec<String>, GetComponentDataOfEntityError> {
        if self.component_data.has_entity(entity_id) {
            Ok(self
                .component_data
                .get_components_of_entity(entity_id)
                .unwrap())
        } else if self.entities.contains(entity_id) {
            Ok(Vec::new())
        } else {
            Err(GetComponentDataOfEntityError::EntityNotFound)
        }
    }
    pub fn get_component_of_entity(
        &self,
        entity_id: &<<TComponentDataStorage as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
    ) -> Result<
        &<TComponentDataStorage as ComponentDataStorage>::ComponentData,
        GetComponentDataOfEntityError,
    > {
        if self.component_storage.has_component(component_name) {
            match self.component_data.get_of_entity(entity_id, component_name) {
                Ok(data) => Ok(data),
                Err(error) => match error {
                    GetComponentDataOfEntityError::EntityNotFound => {
                        if self.entities.contains(entity_id) {
                            Err(GetComponentDataOfEntityError::ComponentNotInEntity)
                        } else {
                            Err(GetComponentDataOfEntityError::EntityNotFound)
                        }
                    }
                    GetComponentDataOfEntityError::ComponentNotFound
                    | GetComponentDataOfEntityError::ComponentNotInEntity => Err(error),
                },
            }
        } else {
            Err(GetComponentDataOfEntityError::ComponentNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::component::component_data::ComponentData;
    use crate::component::component_definition::ComponentDefinition;
    use crate::entity::EntityId;
    use crate::test_structs::{
        TestComponentData, TestComponentDefinition, TestComponentFieldDefinition,
        TestComponentInfo, TestComponentValue, TestEntityId,
    };
    use crate::variant_type::VariantType;
    use crate::world::component_data_storage::{ComponentDataStorage, GCSComponentDataStorage};
    use crate::world::component_storage::{ComponentStorage, GCSComponentStorage};
    use crate::world::errors::GetComponentDataOfEntityError;
    use crate::world::errors::SetComponentDataError::{ComponentNotFound, EntityNotFound};
    use crate::world::gcs_world::GCSWorld;
    use std::borrow::Borrow;

    type TestComponentStorage = GCSComponentStorage<TestComponentInfo, TestComponentDefinition>;
    type TestComponentDataStorage = GCSComponentDataStorage<TestComponentData>;
    type TestGCSWorld = GCSWorld<TestComponentStorage, TestComponentDataStorage>;

    #[test]
    fn get_component_of_entity_returns_data_of_added_component() {
        let mut world = TestGCSWorld::default();

        let component_name = "Test";
        world
            .register_component(component_name, TestComponentDefinition::default())
            .unwrap();
        let entity = world.create_entity().clone();
        world
            .set_component_data(&entity, component_name, &TestComponentData::default())
            .unwrap();

        let result = world.get_component_of_entity(entity.borrow(), component_name);
        assert!(result.is_ok(), "Should have returned an Ok result");
    }

    #[test]
    fn get_component_of_entity_returns_component_not_found_when_the_component_is_not_registered() {
        let mut world = TestGCSWorld::default();
        let entity_id = world.create_entity().clone();
        let result = world.get_component_of_entity(&entity_id, "Test");
        assert!(
            matches!(
                result,
                Err(GetComponentDataOfEntityError::ComponentNotFound)
            ),
            "Should have returned error with ComponentNotFound, got {:?}",
            result
        );
    }

    #[test]
    fn get_component_of_entity_returns_entity_not_found_when_the_entity_is_not_registered() {
        let mut world = TestGCSWorld::default();
        let component_name = "Test";
        world
            .register_component(component_name, TestComponentDefinition::default())
            .unwrap();
        let result = world.get_component_of_entity(&TestEntityId::create(), component_name);
        assert!(
            matches!(result, Err(GetComponentDataOfEntityError::EntityNotFound)),
            "Should have returned error with EntityNotFound, got {:?}",
            result
        );
    }

    #[test]
    fn get_component_of_entity_returns_component_not_in_entity_when_the_entity_never_had_components(
    ) {
        let mut world = TestGCSWorld::default();
        let entity_id = world.create_entity().clone();
        let component = "Test";
        world
            .register_component(component, TestComponentDefinition::default())
            .unwrap();
        let result = world.get_component_of_entity(&entity_id, component);
        assert!(
            matches!(
                result,
                Err(GetComponentDataOfEntityError::ComponentNotInEntity)
            ),
            "Should have returned error with ComponentNotInEntity. Returned: {:?}",
            result
        );
    }

    #[test]
    fn get_component_of_entity_returns_component_not_in_entity_when_the_entity_does_not_have_the_component(
    ) {
        let mut world = TestGCSWorld::default();
        let entity_id = world.create_entity().clone();
        let component_1 = "Test1";
        let component_2 = "Test2";
        world
            .register_component(component_1, TestComponentDefinition::default())
            .unwrap();
        world
            .register_component(component_2, TestComponentDefinition::default())
            .unwrap();
        world
            .set_component_data(&entity_id, component_1, &TestComponentData::default())
            .unwrap();
        let result = world.get_component_of_entity(&entity_id, component_2);
        assert!(
            matches!(
                result,
                Err(GetComponentDataOfEntityError::ComponentNotInEntity)
            ),
            "Should have returned error with ComponentNotInEntity. Returned: {:?}",
            result
        );
    }

    #[test]
    fn get_component_data_returns_all_data_of_a_registered_component() {
        let mut world = TestGCSWorld::default();
        let component_name = "Test";
        let field_name = "TestField";

        let mut definition = TestComponentDefinition::default();
        let field_definition = TestComponentFieldDefinition {
            field_type: VariantType::Int,
            name: field_name.to_string(),
        };
        definition.add_field(field_definition);
        world
            .register_component(component_name, definition)
            .unwrap();

        let entity_id = world.create_entity().clone();
        let mut data = TestComponentData::new(&entity_id);
        data.set_field(field_name.to_string(), &TestComponentValue::Int(9));

        let field_name = "TestField";
        world
            .set_component_data(&entity_id, component_name, &data)
            .unwrap();
        let mut entity_id = TestEntityId::create();
        entity_id.set_id(1);

        world.register_entity(&entity_id).unwrap();
        let mut data = TestComponentData::new(&entity_id);
        data.set_field(field_name.to_string(), &TestComponentValue::Int(27));
        world
            .set_component_data(&entity_id, component_name, &data)
            .unwrap();
        let result = world.get_component_data(component_name);
        assert!(result.is_some(), "Should have returned Some");
        let components = result.unwrap();
        assert_eq!(2, components.len());

        for data in components {
            let _value = data.get_field(field_name.to_string());
            match *data.entity.id.borrow() {
                0 => {
                    assert!(
                        matches!(TestComponentValue::Int(9), _value),
                        "TestField Value for first entity should have been 9"
                    )
                }
                1 => {
                    assert!(
                        matches!(TestComponentValue::Int(27), _value),
                        "Value for first entity should have been 27"
                    )
                }
                _ => {}
            }
        }
    }

    #[test]
    fn get_component_data_returns_none_when_the_component_was_not_registered() {
        let world = TestGCSWorld::default();
        let component_name = "Test";
        let result = world.get_component_data(component_name);

        assert!(result.is_none(), "Should have returned None");
    }

    #[test]
    fn get_components_of_entity_returns_components_of_the_passed_entity() {
        let mut world = TestGCSWorld::default();

        let field_definition = TestComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name_1 = "Test";
        world
            .register_component(component_name_1, component_definition.clone())
            .unwrap();

        let component_name_2 = "Test 2";
        world
            .register_component(component_name_2, component_definition)
            .unwrap();

        let entity_id = world.create_entity().clone();

        world
            .set_component_data(&entity_id, component_name_1, &TestComponentData::default())
            .unwrap();
        world
            .set_component_data(&entity_id, component_name_2, &TestComponentData::default())
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
    fn get_components_of_entity_returns_ok_if_entity_exists_but_never_had_components_added() {
        let mut world = TestGCSWorld::default();
        let entity = world.create_entity().clone();

        let components = world.get_components_of_entity(&entity);
        assert!(components.is_ok());
    }

    #[test]
    fn get_components_of_entity_returns_err_if_entity_does_not_exist() {
        let world = TestGCSWorld::default();

        let components = world.get_components_of_entity(&EntityId::create());
        assert!(components.is_err());
    }

    #[test]
    fn register_component_adds_new_component_and_creates_a_hash() {
        let mut world = TestGCSWorld::default();
        let mut component_definition = TestComponentDefinition::default();

        let field_definition = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        component_definition.add_field(field_definition);

        let component_name = "Test";
        let result: Result<TestComponentInfo, String> =
            world.register_component(component_name, component_definition.clone());
        assert!(result.is_ok(), "register_component should have returned Ok");

        let returned_info = result.unwrap();
        assert_ne!(0, returned_info.hash, "Hash should have a value");

        assert!(
            world.component_storage.has_component_info(&returned_info),
            "Component Information should have been stored in component_definitions"
        );
        let stored_definition = world
            .component_storage
            .get_component_definition(&returned_info)
            .unwrap();
        assert_eq!(
            component_definition, *stored_definition,
            "Stored definition should be the same as the one added"
        );

        assert!(
            world.component_storage.has_component(component_name),
            "component_names should have the passed name as a key"
        );
        let stored_info = world
            .component_storage
            .get_component_info(component_name)
            .unwrap();
        assert_eq!(
            returned_info.hash, stored_info.hash,
            "Hash of stored info should be the same as the one returned"
        );

        let field_definition_2 = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = TestComponentDefinition::default();

        component_definition_2.add_field(field_definition_2);

        let result =
            world.register_component(&format!("{}_", component_name), component_definition_2);
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
        let mut world = TestGCSWorld::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let field_definition_2 = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = TestComponentDefinition::default();
        component_definition_2.add_field(field_definition_2);

        world
            .register_component("Test", component_definition)
            .unwrap();
        let result = world.register_component("Test", component_definition_2);
        assert!(result.is_err())
    }

    #[test]
    pub fn register_component_adds_a_component_with_an_existing_definition_under_a_different_name_with_a_unique_hash(
    ) {
        let mut world = TestGCSWorld::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let info_1 = world
            .register_component("Test", component_definition.clone())
            .unwrap();
        let info_2 = world
            .register_component("Test2", component_definition)
            .unwrap();

        assert_ne!(
            info_1.hash, info_2.hash,
            "Components with different names should have different hashes"
        );
    }

    #[test]
    pub fn create_entity_adds_entity_to_storages() {
        let mut world = TestGCSWorld::default();

        let uuid_1 = world.create_entity().clone();

        assert!(
            world.entities.contains(&uuid_1),
            "Should have added entity to entities"
        );
    }

    #[test]
    pub fn create_entity_creates_a_new_entity() {
        let mut world = TestGCSWorld::default();

        let mut uuid_1 = world.create_entity().clone();
        uuid_1.set_id(1);
        let uuid_2 = world.create_entity().clone();

        assert_ne!(uuid_1, uuid_2, "Should have created unique entities");
    }

    #[test]
    pub fn set_component_data_adds_component_data() {
        let mut world = TestGCSWorld::default();
        let field_name = "Integer";
        let field_definition = TestComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name, component_definition)
            .unwrap();

        let entity_id = world.create_entity().clone();

        world
            .set_component_data(&entity_id, component_name, &TestComponentData::default())
            .unwrap();

        let mut data = TestComponentData::new(&entity_id).clone();
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

        assert!(
            world
                .set_component_data(&entity_id, component_name, &data)
                .is_ok(),
            "set_component_data should have returned Ok"
        );

        let stored_data = world
            .get_component_of_entity(&entity_id, component_name)
            .unwrap();

        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            TestComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );

        let components = world.get_component_data(component_name).unwrap();
        let stored_data = components.first().unwrap();
        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            TestComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );
    }

    #[test]
    pub fn set_component_checks_that_entity_exists() {
        let mut world = TestGCSWorld::default();
        let entity_id = EntityId::create();
        let data = ComponentData::new(&entity_id);

        assert_eq!(
            Err(EntityNotFound),
            world.set_component_data(&entity_id, "Test", &data)
        );
    }

    #[test]
    pub fn set_component_checks_that_component_exists() {
        let mut world = TestGCSWorld::default();

        let entity_id = world.create_entity().clone();
        let data = ComponentData::new(&entity_id);
        assert_eq!(
            Err(ComponentNotFound),
            world.set_component_data(&entity_id, "Test", &data)
        );
    }

    #[test]
    pub fn set_component_adds_component_to_entity_if_it_is_not_added() {
        let mut world = TestGCSWorld::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Boolean".to_string(),
            field_type: VariantType::Bool,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let field_name = "Integer";
        let field_definition_2 = TestComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = TestComponentDefinition::default();
        component_definition_2.add_field(field_definition_2);

        let component_name = "Test_2";
        world
            .register_component("Test_1", component_definition)
            .unwrap();
        world
            .register_component(component_name, component_definition_2)
            .unwrap();

        let entity_id = world.create_entity().clone();
        world
            .set_component_data(&entity_id, "Test_1", &TestComponentData::default())
            .unwrap();

        let mut data = TestComponentData::new(&entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

        assert!(data.fields.contains_key(field_name));

        let result = world.set_component_data(&entity_id, component_name, &data);
        assert!(result.is_ok());

        let stored_data = world
            .get_component_of_entity(&entity_id, component_name)
            .unwrap();

        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            TestComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );
    }
    #[test]
    pub fn set_component_adds_initializes_entity_components_if_not_present() {
        let mut world = TestGCSWorld::default();
        let field_name = "Integer";
        let field_definition = TestComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name, component_definition)
            .unwrap();

        let entity_id = world.create_entity().clone();
        let mut data = TestComponentData::new(&entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

        world
            .set_component_data(&entity_id, component_name, &data)
            .unwrap();

        assert_eq!(
            1,
            world
                .component_data
                .get_all_of_entity(&entity_id)
                .unwrap()
                .len()
        )
    }
}
