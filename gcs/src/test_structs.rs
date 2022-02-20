use crate::component::component_data::ComponentData;
use crate::component::component_definition::{ComponentDefinition, ComponentFieldDefinition};
use crate::component::component_info::ComponentInfo;
use crate::component::component_value::ComponentValue;
use crate::entity::EntityId;
use crate::variant::VariantType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    pub fields: Vec<<TestComponentDefinition as ComponentDefinition>::FieldDefinition>,
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

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct TestEntityId {
    pub id: RefCell<u8>,
}

impl Hash for TestEntityId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.borrow().hash(state);
        state.finish();
    }
}

impl TestEntityId {
    pub(crate) fn set_id(&mut self, id: u8) {
        *self.id.borrow_mut() = id;
    }

    pub(crate) fn new(id: u8) -> Self {
        Self {
            id: RefCell::new(id),
        }
    }
}

impl EntityId for TestEntityId {
    fn create() -> Self
    where
        Self: Sized,
    {
        TestEntityId {
            id: RefCell::default(),
        }
    }

    fn as_string(&self) -> String {
        "".to_string()
    }

    fn parse_str(_input: &str) -> Result<Self, String>
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

    fn get_nil(&self) {}

    fn set_int(&mut self, value: i64) {
        *self = TestComponentValue::Int(value)
    }

    fn get_int(&self) -> i64 {
        match self {
            TestComponentValue::Nil => 0,
            TestComponentValue::Int(value) => *value,
            TestComponentValue::String(value) => {
                let result = str::parse::<i64>(value);
                result.unwrap_or(0)
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
                result.unwrap_or(0.0)
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

#[derive(Default, Clone, PartialEq, Debug)]
pub struct TestComponentData {
    pub entity: TestEntityId,
    pub fields: HashMap<String, <TestComponentData as ComponentData>::ComponentValue>,
}

impl ComponentData for TestComponentData {
    type EntityId = TestEntityId;
    type ComponentValue = TestComponentValue;

    fn new(entity: &Self::EntityId) -> Self {
        TestComponentData {
            entity: entity.clone(),
            fields: HashMap::new(),
        }
    }

    fn get_entity(&self) -> &Self::EntityId {
        &self.entity
    }

    fn get_field(&self, field: String) -> &Self::ComponentValue {
        self.fields.get(&field).unwrap()
    }

    fn set_field(&mut self, field: String, value: &Self::ComponentValue) {
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
