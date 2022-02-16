use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::component::component_data::ComponentData;
use crate::component::component_definition::{ComponentDefinition, ComponentFieldDefinition};
use crate::component::component_info::ComponentInfo;
use crate::ecs_world::RegisterEntityError::AlreadyRegistered;
use crate::ecs_world::SetComponentDataError::{ComponentNotFound, DataInUse, EntityNotFound};
use crate::entity::EntityId;

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
pub struct ECSWorld<
    TComponentDefinition: ComponentDefinition,
    TComponentData: ComponentData,
    TComponentInfo: ComponentInfo,
> {
    component_definitions: HashMap<TComponentInfo, TComponentDefinition>,
    component_names: HashMap<String, TComponentInfo>,
    entities: Vec<TComponentData::EntityIdType>,
    components: HashMap<String, Vec<Rc<RefCell<TComponentData>>>>,
    components_of_entity:
        HashMap<TComponentData::EntityIdType, HashMap<String, Rc<RefCell<TComponentData>>>>,
}

pub fn create_ecs_world<
    TComponentDefinition: ComponentDefinition,
    TComponentData: ComponentData,
    TComponentInfo: ComponentInfo,
>() -> ECSWorld<TComponentDefinition, TComponentData, TComponentInfo> {
    ECSWorld::default()
}

impl<
        TComponentDefinition: ComponentDefinition,
        TComponentData: ComponentData,
        TComponentInfo: ComponentInfo,
    > ECSWorld<TComponentDefinition, TComponentData, TComponentInfo>
{
    pub fn register_component(
        &mut self,
        name: String,
        component_definition: TComponentDefinition,
    ) -> Result<TComponentInfo, String> {
        let mut hasher = DefaultHasher::default();
        component_definition.hash(&mut hasher);
        name.hash(&mut hasher);

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.component_names.entry(name.clone())
        {
            let info = TComponentInfo::create(hasher.finish());
            self.component_definitions
                .entry(info)
                .or_insert_with(|| component_definition.clone());
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

    pub fn create_entity(&mut self) -> Box<TComponentData::EntityIdType> {
        let id = EntityId::create();
        self.entities.push(id);
        Box::new(*self.entities.last().unwrap())
    }

    pub fn register_entity(
        &mut self,
        id: &TComponentData::EntityIdType,
    ) -> Result<(), RegisterEntityError> {
        if self.entities.contains(id) {
            Err(AlreadyRegistered)
        } else {
            self.entities.push(*id);
            Ok(())
        }
    }

    pub fn add_component_to_entity(
        &mut self,
        entity_id: &TComponentData::EntityIdType,
        component: String,
    ) -> Result<(), String> {
        let components = self.components.get_mut(&component).unwrap();

        if components
            .iter()
            .any(|d| d.borrow().get_entity() == *entity_id)
        {
            Err("Component was already added for that entity".to_string())
        } else {
            let value = RefCell::new(TComponentData::new(*entity_id));
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
        entity_id: &TComponentData::EntityIdType,
        component: String,
        data: &TComponentData,
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

            for component_field in &component_fields.get_fields() {
                let new_data = data.get_field(component_field.get_name());
                stored_data.set_field(component_field.get_name(), &new_data.clone());
            }
            Ok(())
        }
    }

    pub fn is_component_added_to_entity(
        &self,
        entity_id: &TComponentData::EntityIdType,
        component: String,
    ) -> bool {
        self.components_of_entity
            .get(entity_id)
            .map_or_else(|| false, |c| c.contains_key(&component))
    }

    pub fn get_components_of_entity(
        &self,
        entity_id: &TComponentData::EntityIdType,
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
    use crate::component::component_definition::ComponentDefinition;
    use crate::component::component_definition::ComponentFieldDefinition;
    use crate::component::component_info::ComponentInfo;
    use crate::component::component_value::ComponentValue;
    use crate::ecs_world;
    use crate::ecs_world::ECSWorld;
    use crate::ecs_world::SetComponentDataError::{ComponentNotFound, EntityNotFound};
    use crate::entity::EntityId;
    use crate::variant::VariantType;
    use std::collections::HashMap;

    #[derive(Default, Clone, Hash, Debug, PartialEq, Eq)]
    pub struct TestComponentFieldDefinition {
        pub name: String,
        pub field_type: VariantType,
    }

    impl ComponentFieldDefinition for TestComponentFieldDefinition {
        fn get_type(&self) -> VariantType {
            self.field_type
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Default, Hash, Clone, PartialEq, Debug)]
    pub struct TestComponentDefinition {
        pub fields: Vec<
            <ecs_world::tests::TestComponentDefinition as ComponentDefinition>::FieldDefinition,
        >,
    }

    impl ComponentDefinition for TestComponentDefinition {
        type FieldDefinition = TestComponentFieldDefinition;

        fn get_fields(&self) -> Vec<Self::FieldDefinition> {
            self.fields.clone()
        }

        fn add_field(&mut self, field_definition: Self::FieldDefinition) {
            self.fields.push(field_definition);
        }
    }

    #[derive(Default, Hash, Eq, PartialEq, Copy, Clone, Debug)]
    pub struct TestEntityId {
        id: u8,
    }

    impl EntityId for TestEntityId {
        fn create() -> Self
        where
            Self: Sized,
        {
            TestEntityId { id: 0 }
        }

        fn as_string(&self) -> String {
            "".to_string()
        }

        fn parse_str(_input: &str) -> Result<Box<Self>, String>
        where
            Self: Sized,
        {
            Result::Err("Not Implemented".to_string())
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum TestComponentValue {
        Nil,
        Int(i64),
        String(String),
        Bool(bool),
        Real(f64),
    }

    impl ComponentValue for TestComponentValue {
        fn get_type(&self) -> VariantType {
            match self {
                TestComponentValue::Nil => VariantType::Nil,
                TestComponentValue::Int(_) => VariantType::Int,
                TestComponentValue::String(_) => VariantType::String,
                TestComponentValue::Bool(_) => VariantType::Bool,
                TestComponentValue::Real(_) => VariantType::Real,
            }
        }

        fn set_nil(&mut self) {
            *self = TestComponentValue::Nil;
        }

        fn get_nil(&self) -> () {}

        fn set_int(&mut self, value: i64) {
            *self = TestComponentValue::Int(value)
        }

        fn get_int(&self) -> i64 {
            match self {
                TestComponentValue::Nil => 0,
                TestComponentValue::Int(value) => *value,
                TestComponentValue::String(value) => {
                    let result = str::parse::<i64>(value);
                    match result {
                        Ok(value) => value,
                        Err(_) => 0,
                    }
                }
                TestComponentValue::Bool(value) => {
                    if *value {
                        1
                    } else {
                        0
                    }
                }
                TestComponentValue::Real(value) => *value as i64,
            }
        }

        fn set_string(&mut self, value: String) {
            *self = TestComponentValue::String(value)
        }

        fn get_string(&self) -> String {
            match self {
                TestComponentValue::Nil => "".to_string(),
                TestComponentValue::Int(value) => value.to_string(),
                TestComponentValue::String(value) => value.clone(),
                TestComponentValue::Bool(value) => value.to_string(),
                TestComponentValue::Real(value) => value.to_string(),
            }
        }

        fn set_bool(&mut self, value: bool) {
            *self = TestComponentValue::Bool(value)
        }

        fn get_bool(&self) -> bool {
            match self {
                TestComponentValue::Nil => false,
                TestComponentValue::Int(value) => *value != 0,
                TestComponentValue::String(value) => value.is_empty(),
                TestComponentValue::Bool(value) => *value,
                TestComponentValue::Real(value) => *value != 0.0,
            }
        }

        fn set_real(&mut self, value: f64) {
            *self = TestComponentValue::Real(value)
        }

        fn get_real(&self) -> f64 {
            match self {
                TestComponentValue::Nil => 0.0,
                TestComponentValue::Int(value) => *value as f64,
                TestComponentValue::String(value) => {
                    let result = str::parse::<f64>(value);
                    match result {
                        Ok(value) => value,
                        Err(_) => 0.0,
                    }
                }
                TestComponentValue::Bool(value) => {
                    if *value {
                        1.0
                    } else {
                        0.0
                    }
                }
                TestComponentValue::Real(value) => *value,
            }
        }
    }

    impl Default for TestComponentValue {
        fn default() -> Self {
            TestComponentValue::Nil
        }
    }

    #[derive(Default)]
    pub struct TestComponentData {
        pub entity: TestEntityId,
        pub fields: HashMap<
            String,
            <ecs_world::tests::TestComponentData as ComponentData>::ComponentValueType,
        >,
    }

    impl ComponentData for TestComponentData {
        type EntityIdType = TestEntityId;
        type ComponentValueType = TestComponentValue;

        fn new(entity: TestEntityId) -> Self {
            TestComponentData {
                entity,
                fields: HashMap::new(),
            }
        }

        fn get_entity(&self) -> TestEntityId {
            self.entity
        }

        fn get_field(&self, field: String) -> &Self::ComponentValueType {
            self.fields.get(&field).unwrap()
        }

        fn set_field(&mut self, field: String, value: &Self::ComponentValueType) {
            self.fields.insert(field, value.clone());
        }
    }

    #[derive(Default, Hash, Eq, PartialEq, Copy, Clone)]
    pub struct TestComponentInfo {
        pub hash: u64,
    }

    impl ComponentInfo for TestComponentInfo {
        fn get_hash(&self) -> u64 {
            self.hash
        }

        fn create(hash: u64) -> Self
        where
            Self: Sized,
        {
            Self { hash }
        }
    }

    #[test]
    fn get_components_of_entity_returns_components_of_the_passed_entity() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();

        let field_definition = TestComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name_1 = "Test";
        world
            .register_component(component_name_1.to_string(), component_definition.clone())
            .unwrap();

        let component_name_2 = "Test 2";
        world
            .register_component(component_name_2.to_string(), component_definition)
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
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let entity = world.create_entity();

        let components = world.get_components_of_entity(&entity);
        assert!(components.is_ok());
    }

    #[test]
    fn get_components_returns_err_if_entity_does_not_exist() {
        let world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();

        let components = world.get_components_of_entity(&EntityId::create());
        assert!(components.is_err());
    }

    #[test]
    fn register_component_adds_new_component_and_creates_a_hash() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let mut component_definition = TestComponentDefinition::default();

        let field_definition = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        component_definition.add_field(field_definition);

        let component_name = "Test";
        let result: Result<TestComponentInfo, String> =
            world.register_component(component_name.to_string(), component_definition.clone());
        assert!(result.is_ok(), "register_component should have returned Ok");

        let returned_info = result.unwrap();
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

        let field_definition_2 = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition_2 = TestComponentDefinition::default();

        component_definition_2.add_field(field_definition_2);

        let result =
            world.register_component(format!("{}_", component_name), component_definition_2);
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
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
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
            .register_component("Test".to_string(), component_definition)
            .unwrap();
        let result = world.register_component("Test".to_string(), component_definition_2);
        assert!(result.is_err())
    }

    #[test]
    pub fn register_component_adds_a_component_with_an_existing_definition_under_a_different_name_with_a_unique_hash(
    ) {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Field".to_string(),
            field_type: VariantType::Nil,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let info_1 = world
            .register_component("Test".to_string(), component_definition.clone())
            .unwrap();
        let info_2 = world
            .register_component("Test2".to_string(), component_definition)
            .unwrap();

        assert_ne!(
            info_1.hash, info_2.hash,
            "Components with different names should have different hashes"
        );
    }

    #[test]
    pub fn create_entity_creates_a_new_entity() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();

        let mut uuid_1 = *world.create_entity();
        uuid_1.id = 1;
        let uuid_2 = *world.create_entity();

        assert_ne!(uuid_1, uuid_2, "Should have created unique entities");
    }

    #[test]
    pub fn add_component_to_entity_adds_a_new_component_to_an_entity() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), component_definition)
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
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let field_definition = TestComponentFieldDefinition {
            name: "Integer".to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), component_definition)
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
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let field_name = "Integer";
        let field_definition = TestComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), component_definition)
            .unwrap();

        let entity_id = *world.create_entity();

        world
            .add_component_to_entity(&entity_id, component_name.to_string())
            .unwrap();

        let mut data = TestComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

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
            TestComponentValue::Int(value),
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
            TestComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );
    }

    #[test]
    pub fn set_component_checks_that_entity_exists() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let entity_id = EntityId::create();
        let data = ComponentData::new(entity_id);

        assert_eq!(
            Err(EntityNotFound),
            world.set_component_data(&entity_id, "Test".to_string(), &data)
        );
    }

    #[test]
    pub fn set_component_checks_that_component_exists() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();

        let entity_id = *world.create_entity();
        let data = ComponentData::new(entity_id);
        assert_eq!(
            Err(ComponentNotFound),
            world.set_component_data(&entity_id, "Test".to_string(), &data)
        );
    }

    #[test]
    pub fn set_component_adds_component_to_entity_if_it_is_not_added() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
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
            .register_component("Test_1".to_string(), component_definition)
            .unwrap();
        world
            .register_component(component_name.to_string(), component_definition_2)
            .unwrap();

        let entity_id = *world.create_entity();
        world
            .add_component_to_entity(&entity_id, "Test_1".to_string())
            .unwrap();

        let mut data = TestComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

        assert!(data.fields.contains_key(field_name));

        let result = world.set_component_data(&entity_id, component_name.to_string(), &data);
        assert!(result.is_ok());

        let components = world.components_of_entity.get(&entity_id).unwrap();

        let stored_data = components.get(component_name).unwrap().borrow();

        let field_data = stored_data.get_field(field_name.to_string());
        assert_eq!(
            TestComponentValue::Int(value),
            *field_data,
            "Stored value should be the same as the one supplied"
        );
    }
    #[test]
    pub fn set_component_adds_initializes_entity_components_if_not_present() {
        let mut world =
            ECSWorld::<TestComponentDefinition, TestComponentData, TestComponentInfo>::default();
        let field_name = "Integer";
        let field_definition = TestComponentFieldDefinition {
            name: field_name.to_string(),
            field_type: VariantType::Int,
        };

        let mut component_definition = TestComponentDefinition::default();
        component_definition.add_field(field_definition);

        let component_name = "Test";
        world
            .register_component(component_name.to_string(), component_definition)
            .unwrap();

        let entity_id = *world.create_entity();
        let mut data = TestComponentData::new(entity_id);
        let value = 2;
        data.set_field(field_name.to_string(), &TestComponentValue::Int(value));

        world
            .set_component_data(&entity_id, component_name.to_string(), &data)
            .unwrap();

        assert_eq!(1, world.components_of_entity.get(&entity_id).unwrap().len())
    }
}
