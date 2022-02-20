use crate::component::component_data::ComponentData;
use crate::world::errors::GetComponentDataOfEntityError;
use std::collections::HashMap;

pub enum HasEntityComponentResult {
    EntityNotFound,
    EntityDoesNotHaveComponent,
    EntityHasComponent,
}

pub trait ComponentDataStorage {
    type ComponentData: ComponentData;

    fn get_all(&self) -> Vec<&Self::ComponentData>;
    fn get_all_of_component(&self, component_name: &str) -> Option<Vec<&Self::ComponentData>>;
    fn get_all_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Option<Vec<&Self::ComponentData>>;
    fn get_components_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Option<Vec<String>>;
    fn get_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
    ) -> Result<&Self::ComponentData, GetComponentDataOfEntityError>;
    fn set_component_of_entity(
        &mut self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
        component_data: &Self::ComponentData,
    );
    fn has_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> bool;
    fn does_entity_have_component(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
    ) -> HasEntityComponentResult;
}

#[derive(Default)]
pub struct GCSComponentDataStorage<TComponentData: ComponentData> {
    components_of_entity: HashMap<TComponentData::EntityId, HashMap<String, TComponentData>>,
}

impl<TComponentData: ComponentData> ComponentDataStorage
    for GCSComponentDataStorage<TComponentData>
{
    type ComponentData = TComponentData;

    fn get_all(&self) -> Vec<&Self::ComponentData> {
        self.components_of_entity
            .iter()
            .flat_map(|c| c.1.values())
            .collect()
    }

    fn get_all_of_component(&self, component_name: &str) -> Option<Vec<&Self::ComponentData>> {
        let components: Vec<&Self::ComponentData> = self
            .components_of_entity
            .iter()
            .filter(|(_, c)| c.contains_key(component_name))
            .map(|(_, c)| c.get(component_name).unwrap())
            .collect();

        if components.is_empty() {
            None
        } else {
            Some(components)
        }
    }

    fn get_all_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Option<Vec<&Self::ComponentData>> {
        self.components_of_entity
            .get(entity)
            .map(|c| c.values())
            .map(|c| c.collect())
    }

    fn get_components_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> Option<Vec<String>> {
        self.components_of_entity
            .get(entity)
            .map(|c| c.keys().cloned())
            .map(|c| c.collect())
    }

    fn get_of_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
    ) -> Result<&Self::ComponentData, GetComponentDataOfEntityError> {
        self.components_of_entity.get(entity).map_or_else(
            || Err(GetComponentDataOfEntityError::EntityNotFound),
            |c| {
                c.get(component_name)
                    .ok_or(GetComponentDataOfEntityError::ComponentNotInEntity)
            },
        )
    }

    fn set_component_of_entity(
        &mut self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
        component_data: &Self::ComponentData,
    ) {
        self.components_of_entity
            .entry(entity.clone())
            .or_default()
            .insert(component_name.to_string(), component_data.clone());
    }

    fn has_entity(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
    ) -> bool {
        self.components_of_entity.contains_key(entity)
    }

    fn does_entity_have_component(
        &self,
        entity: &<<Self as ComponentDataStorage>::ComponentData as ComponentData>::EntityId,
        component_name: &str,
    ) -> HasEntityComponentResult {
        match self.components_of_entity.get(entity) {
            None => HasEntityComponentResult::EntityNotFound,
            Some(components) => {
                if components.contains_key(&component_name.to_string()) {
                    HasEntityComponentResult::EntityHasComponent
                } else {
                    HasEntityComponentResult::EntityDoesNotHaveComponent
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_structs::{TestComponentData, TestComponentValue, TestEntityId};
    use crate::world::component_data_storage::{
        ComponentDataStorage, GCSComponentDataStorage, HasEntityComponentResult,
    };
    use crate::world::errors::GetComponentDataOfEntityError;
    use std::collections::HashMap;

    #[test]
    fn does_entity_have_component_returns_entity_not_found_if_the_entity_does_have_the_component() {
        let storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";
        let result = storage.does_entity_have_component(&test_entity_id_1, test_component_1);

        assert!(
            matches!(result, HasEntityComponentResult::EntityNotFound),
            "Should have returned EntityHasComponent"
        );
    }

    #[test]
    fn does_entity_have_component_returns_does_not_have_component_if_the_entity_does_have_the_component(
    ) {
        let mut storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";
        let components_of_entity = HashMap::new();
        storage
            .components_of_entity
            .insert(test_entity_id_1.clone(), components_of_entity);
        let result = storage.does_entity_have_component(&test_entity_id_1, test_component_1);

        assert!(
            matches!(result, HasEntityComponentResult::EntityDoesNotHaveComponent),
            "Should have returned EntityHasComponent"
        );
    }

    #[test]
    fn does_entity_have_component_returns_has_component_if_the_entity_does_have_the_component() {
        let mut storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";
        let mut components_of_entity = HashMap::new();
        components_of_entity.insert(test_component_1.to_string(), TestComponentData::default());
        storage
            .components_of_entity
            .insert(test_entity_id_1.clone(), components_of_entity);
        let result = storage.does_entity_have_component(&test_entity_id_1, test_component_1);

        assert!(
            matches!(result, HasEntityComponentResult::EntityHasComponent),
            "Should have returned EntityHasComponent"
        );
    }

    #[test]
    fn has_entity_returns_false_if_the_entity_does_not_exist() {
        let storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        assert!(
            !storage.has_entity(&test_entity_id_1),
            "Should have returned true"
        );
    }

    #[test]
    fn has_entity_returns_true_if_the_entity_exists() {
        let mut storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        storage
            .components_of_entity
            .insert(test_entity_id_1.clone(), HashMap::new());

        assert!(
            storage.has_entity(&test_entity_id_1),
            "Should have returned true"
        );
    }

    #[test]
    fn set_component_of_entity_does_not_panic_when_passing_a_new_entity() {
        let mut storage = GCSComponentDataStorage::<TestComponentData>::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::new(),
        };

        storage.set_component_of_entity(&test_entity_id_1, test_component_1, &component_data);
    }

    #[test]
    fn set_component_of_entity_adds_new_component_to_existing_entity() {
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);
        components.insert(test_entity_id_1.clone(), entity_components);

        let mut storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let mut fields = HashMap::new();

        let test_field_1 = "Field1";

        let expected_value = TestComponentValue::Int(9);
        fields.insert(test_field_1.to_string(), expected_value.clone());

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields,
        };

        storage.set_component_of_entity(&test_entity_id_1, test_component_2, &component_data);

        let stored_data = storage
            .components_of_entity
            .get(&test_entity_id_1)
            .unwrap()
            .get(test_component_2);

        assert!(
            stored_data.is_some(),
            "Should have stored a component with that name for the entity"
        );

        let stored_data = stored_data.unwrap();
        let stored_value = stored_data.fields.get(&test_field_1.to_string()).unwrap();

        assert_eq!(
            expected_value, *stored_value,
            "Should have stored the passed value"
        );
    }

    #[test]
    fn get_of_entity_returns_entity_not_found_if_the_entity_does_not_exist() {
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);
        components.insert(test_entity_id_1, entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let _stored_data = storage.get_of_entity(&test_entity_id_2, test_component_1);

        assert!(
            matches!(GetComponentDataOfEntityError::EntityNotFound, _stored_data),
            "Should have returned EntityNotFound"
        );
    }

    #[test]
    fn get_of_entity_returns_component_not_in_entity_if_the_entity_does_not_have_that_component() {
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);
        components.insert(test_entity_id_1.clone(), entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let _stored_data = storage.get_of_entity(&test_entity_id_1, test_component_2);

        assert!(
            matches!(
                GetComponentDataOfEntityError::ComponentNotInEntity,
                _stored_data
            ),
            "Should have returned ComponentNotInEntity"
        );
    }

    #[test]
    fn get_of_entity_returns_an_existing_component_of_an_entity() {
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_component_1 = "Test1";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        components.insert(test_entity_id_1.clone(), entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let stored_data = storage.get_of_entity(&test_entity_id_1, test_component_1);

        assert!(stored_data.is_ok(), "Should have returned Ok");

        let stored_data = stored_data.unwrap();

        assert_eq!(
            component_data, *stored_data,
            "Should have returned to stored data"
        );
    }

    #[test]
    fn get_all_of_entity_returns_none_if_the_entity_does_not_exist() {
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);
        components.insert(test_entity_id_1.clone(), entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let returned_components = storage.get_all_of_entity(&test_entity_id_2);

        assert_eq!(None, returned_components, "Should have returned None");
    }

    #[test]
    fn get_all_components_of_entity_returns_components_of_that_entity() {
        let mut expected_components = Vec::new();
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";
        let test_component_3 = "Test3";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);
        components.insert(test_entity_id_1, entity_components);

        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);
        expected_components.push(test_component_1.to_string());

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);
        expected_components.push(test_component_2.to_string());

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_3.to_string(), component_data);
        expected_components.push(test_component_3.to_string());
        components.insert(test_entity_id_2.clone(), entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let returned_components = storage.get_components_of_entity(&test_entity_id_2);

        assert!(returned_components.is_some(), "Should have returned Some");

        let returned_components = returned_components.unwrap();

        assert!(
            !returned_components.is_empty(),
            "Should have returned all expected components: {:?}",
            expected_components
        );

        assert!(
            returned_components
                .iter()
                .all(|c| expected_components.contains(c)),
            "Should have returned all expected components: {:?}",
            expected_components
        );
    }

    #[test]
    fn get_all_of_entity_returns_all_data_of_that_entity() {
        let mut expected_components = Vec::new();
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";
        let test_component_3 = "Test3";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);
        components.insert(test_entity_id_1.clone(), entity_components);

        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_3.to_string(), component_data.clone());
        expected_components.push(component_data);
        components.insert(test_entity_id_2.clone(), entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let returned_components = storage.get_all_of_entity(&test_entity_id_2);

        assert!(returned_components.is_some(), "Should have returned Some");

        let returned_components = returned_components.unwrap();

        assert!(
            !returned_components.is_empty(),
            "Should have returned all expected components: {:?}",
            expected_components
        );

        assert!(
            returned_components
                .iter()
                .all(|c| expected_components.contains(*c)),
            "Should have returned all expected components: {:?}",
            expected_components
        );
    }

    #[test]
    fn get_all_of_component_returns_none_if_the_component_does_not_exists() {
        let mut expected_components = Vec::new();
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";
        let test_component_3 = "Test3";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);
        components.insert(test_entity_id_1.clone(), entity_components);

        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2,
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let _result = storage.get_all_of_component(test_component_3);

        assert!(
            matches!(Option::<Vec<&TestComponentData>>::None, _result),
            "Should have returned None"
        );
    }

    #[test]
    fn get_all_of_component_returns_all_of_a_component() {
        let mut expected_components = Vec::new();
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";
        let test_component_3 = "Test3";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);
        components.insert(test_entity_id_1, entity_components);

        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_3.to_string(), component_data);
        components.insert(test_entity_id_2, entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let returned_components = storage.get_all_of_component(test_component_1).unwrap();

        assert!(
            !returned_components.is_empty(),
            "Should have returned all expected components: {:?}",
            expected_components
        );

        assert!(
            returned_components
                .iter()
                .all(|c| expected_components.contains(*c)),
            "Should have returned all expected components: {:?}",
            expected_components
        );
    }

    #[test]
    fn get_all_returns_all_components() {
        let mut expected_components = Vec::new();
        #[allow(clippy::mutable_key_type)]
        let mut components = HashMap::default();
        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let test_entity_id_1 = TestEntityId::new(1);
        let test_entity_id_2 = TestEntityId::new(2);
        let test_component_1 = "Test1";
        let test_component_2 = "Test2";
        let test_component_3 = "Test3";

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_1.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data.clone());
        expected_components.push(component_data);
        components.insert(test_entity_id_1, entity_components);

        let mut entity_components: HashMap<String, TestComponentData> = HashMap::default();
        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_1.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_2.to_string(), component_data.clone());
        expected_components.push(component_data);

        let component_data = TestComponentData {
            entity: test_entity_id_2.clone(),
            fields: HashMap::default(),
        };
        entity_components.insert(test_component_3.to_string(), component_data.clone());
        expected_components.push(component_data);
        components.insert(test_entity_id_2, entity_components);

        let storage = GCSComponentDataStorage {
            components_of_entity: components.clone(),
        };

        let returned_components = storage.get_all();

        assert!(
            !returned_components.is_empty(),
            "Should have returned all expected components: {:?}",
            expected_components
        );

        assert!(
            returned_components
                .iter()
                .all(|c| expected_components.contains(*c)),
            "Should have returned all expected components: {:?}",
            expected_components
        );
    }
}
