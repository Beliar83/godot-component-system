use crate::component::component_data::create_component_data;
use crate::component::component_data::CXXComponentData;
use crate::component::component_definition::CXXComponentDefinition;
use crate::component::component_info::create_component_info;
use crate::component::component_info::CXXComponentInfo;
use crate::entity::create_entity;
use crate::entity::entity_id_from_string;
use crate::entity::CXXEntityId;
use cxx::{type_id, ExternType};
use gcs::ecs_world::{
    create_ecs_world, ECSWorld, GetComponentsOfEntityError, RegisterEntityError,
    SetComponentDataError,
};

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    extern "Rust" {
        #[cxx_name = "ComponentInfo"]
        type CXXComponentInfo;
    }

    extern "Rust" {
        #[cxx_name = "ComponentData"]
        type CXXComponentData;
        fn get_field(self: &CXXComponentData, field: String) -> &ComponentValue;
        fn set_field(self: &mut CXXComponentData, field: String, value: &ComponentValue);
        fn create_component_data(entity: &CXXEntityId) -> Box<CXXComponentData>;
    }
    extern "Rust" {
        #[cxx_name = "EntityId"]
        type CXXEntityId;

        fn create_entity() -> Box<CXXEntityId>;
        fn as_string(&self) -> String;
        fn entity_id_from_string(id: String) -> Result<Box<CXXEntityId>>;
    }

    extern "Rust" {
        #[cxx_name = "ECSWorld"]
        type CXXECSWorld;

        pub(crate) fn create_component_info(hash: u64) -> Box<CXXComponentInfo>;
        fn register_component(
            self: &mut CXXECSWorld,
            name: String,
            component_definition: &ComponentDefinition,
        ) -> Result<Box<CXXComponentInfo>>;

        fn register_entity(self: &mut CXXECSWorld, id: &CXXEntityId) -> Result<()>;

        pub fn set_component_data(
            self: &mut CXXECSWorld,
            entity_id: &CXXEntityId,
            component: String,
            data: &CXXComponentData,
        ) -> Result<()>;

        fn is_component_added_to_entity(
            self: &CXXECSWorld,
            entity_id: &CXXEntityId,
            component: String,
        ) -> bool;

        fn get_components_of_entity(
            self: &CXXECSWorld,
            entity_id: &CXXEntityId,
        ) -> Result<Vec<String>>;

        fn create_entity(self: &mut CXXECSWorld) -> Box<CXXEntityId>;

        #[cxx_name = "create_ecs_world"]
        pub fn create_cxx_ecs_world() -> Box<CXXECSWorld>;
    }

    extern "C++" {
        include!("rust/cxx.h");
        include!("gcs-cxx/include/godot/variant.h");
        include!("gcs-cxx/src/component/component_definition.rs.h");
        include!("gcs-cxx/src/component/component_value.rs.h");

        type ComponentDefinition = crate::component::component_definition::CXXComponentDefinition;
        type ComponentValue = crate::component::component_value::CXXComponentValue;
    }
}

pub(crate) struct CXXECSWorld(ECSWorld<CXXComponentDefinition, CXXComponentData, CXXComponentInfo>);

impl CXXECSWorld {
    fn register_component(
        self: &mut CXXECSWorld,
        name: String,
        component_definition: &CXXComponentDefinition,
    ) -> Result<Box<CXXComponentInfo>, String> {
        self.0
            .register_component(name, component_definition.clone())
            .map(|r| Box::new(r))
    }

    fn register_entity(
        self: &mut CXXECSWorld,
        id: &CXXEntityId,
    ) -> Result<(), RegisterEntityError> {
        self.0.register_entity(id)
    }

    fn set_component_data(
        self: &mut CXXECSWorld,
        entity_id: &CXXEntityId,
        component: String,
        data: &CXXComponentData,
    ) -> Result<(), SetComponentDataError> {
        self.0.set_component_data(entity_id, component, data)
    }

    fn get_components_of_entity(
        self: &CXXECSWorld,
        entity_id: &CXXEntityId,
    ) -> Result<Vec<String>, GetComponentsOfEntityError> {
        self.0.get_components_of_entity(entity_id)
    }

    fn is_component_added_to_entity(
        self: &CXXECSWorld,
        entity_id: &CXXEntityId,
        component: String,
    ) -> bool {
        self.0.is_component_added_to_entity(entity_id, component)
    }

    fn create_entity(self: &mut CXXECSWorld) -> Box<CXXEntityId> {
        self.0.create_entity()
    }
}

pub(crate) fn create_cxx_ecs_world() -> Box<CXXECSWorld> {
    Box::new(CXXECSWorld(create_ecs_world::<
        CXXComponentDefinition,
        CXXComponentData,
        CXXComponentInfo,
    >()))
}

unsafe impl ExternType for CXXECSWorld {
    type Id = type_id!("gcs::ffi::ECSWorld");
    type Kind = cxx::kind::Trivial;
}
