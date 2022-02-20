use crate::component::component_definition::ComponentDefinition;
use crate::component::component_info::ComponentInfo;
use crate::world::errors::AddComponentError;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub trait ComponentStorage {
    type ComponentInfo: ComponentInfo;
    type ComponentDefinition: ComponentDefinition;

    fn get_component_names(&self) -> Vec<String>;
    fn has_component(&self, component_name: &str) -> bool;
    fn has_component_info(&self, component_info: &Self::ComponentInfo) -> bool;
    fn add_component(
        &mut self,
        component_name: &str,
        component_definition: Self::ComponentDefinition,
    ) -> Result<Self::ComponentInfo, AddComponentError>;
    fn get_component_info(&self, component_name: &str) -> Option<&Self::ComponentInfo>;
    fn get_component_definition(
        &self,
        component_info: &Self::ComponentInfo,
    ) -> Option<&Self::ComponentDefinition>;
}

#[derive(Default)]
pub struct GCSComponentStorage<
    TComponentInfo: ComponentInfo,
    TComponentDefinition: ComponentDefinition,
> {
    component_definitions: HashMap<TComponentInfo, TComponentDefinition>,
    component_names: HashMap<String, TComponentInfo>,
}

impl<TComponentInfo: ComponentInfo, TComponentDefinition: ComponentDefinition> ComponentStorage
    for GCSComponentStorage<TComponentInfo, TComponentDefinition>
{
    type ComponentInfo = TComponentInfo;
    type ComponentDefinition = TComponentDefinition;

    fn get_component_names(&self) -> Vec<String> {
        self.component_names.keys().cloned().collect()
    }

    fn has_component(&self, component_name: &str) -> bool {
        self.component_names
            .contains_key(&component_name.to_string())
    }

    fn has_component_info(&self, component_info: &Self::ComponentInfo) -> bool {
        self.component_definitions.contains_key(component_info)
    }

    fn add_component(
        &mut self,
        component_name: &str,
        component_definition: Self::ComponentDefinition,
    ) -> Result<Self::ComponentInfo, AddComponentError> {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.component_names.entry(component_name.to_string())
        {
            let mut hasher = DefaultHasher::default();
            component_definition.hash(&mut hasher);
            component_name.hash(&mut hasher);
            let info = ComponentInfo::create(hasher.finish());
            self.component_definitions
                .entry(info)
                .or_insert(component_definition);
            e.insert(info);
            Ok(info)
        } else {
            Err(AddComponentError::NameAlreadyAdded)
        }
    }

    fn get_component_info(&self, component_name: &str) -> Option<&Self::ComponentInfo> {
        self.component_names.get(component_name)
    }

    fn get_component_definition(
        &self,
        component_info: &Self::ComponentInfo,
    ) -> Option<&Self::ComponentDefinition> {
        self.component_definitions.get(component_info)
    }
}
