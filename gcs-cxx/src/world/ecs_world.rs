use std::string::String;

use cxx::{type_id, ExternType};

use gcs::world::component_storage::GCSComponentStorage;
use gcs::world::ecs_world::{create_ecs_world, ECSWorld};

use crate::component::component_data::create_component_data;
use crate::component::component_data::CXXComponentData;
use crate::component::component_definition::CXXComponentDefinition;
use crate::component::component_info::create_component_info;
use crate::component::component_info::CXXComponentInfo;
use crate::entity::create_entity;
use crate::entity::entity_id_from_string;
use crate::entity::CXXEntityId;
use crate::entity::EntityIdResult;
use crate::godot::error::GCSResult;

#[cxx::bridge(namespace = gcs::ffi)]
pub mod ffi {
    extern "Rust" {
        type UnitResult;

        fn is_error(&self) -> bool;
        fn get_error(&self) -> String;
    }

    extern "Rust" {
        type StringVecResult;

        fn is_error(&self) -> bool;
        fn get_result(&self) -> Vec<String>;
        fn get_error(&self) -> String;
    }

    extern "Rust" {
        type EntityIdResult;

        fn is_error(&self) -> bool;
        fn get_result(&self) -> Box<CXXEntityId>;
        fn get_error(&self) -> String;
    }

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
        fn entity_id_from_string(id: String) -> Box<EntityIdResult>;
    }

    extern "Rust" {
        type ComponentInfoResult;

        fn is_error(&self) -> bool;
        fn get_result(&self) -> Box<CXXComponentInfo>;
        fn get_error(&self) -> String;

    }

    extern "Rust" {
        #[cxx_name = "ECSWorld"]
        type CXXECSWorld;

        pub(crate) fn create_component_info(hash: u64) -> Box<CXXComponentInfo>;
        fn register_component(
            self: &mut CXXECSWorld,
            name: String,
            component_definition: &ComponentDefinition,
        ) -> Box<ComponentInfoResult>;

        fn register_entity(&mut self, id: &CXXEntityId) -> Box<UnitResult>;

        pub fn set_component_data(
            &mut self,
            entity_id: &CXXEntityId,
            component: String,
            data: &CXXComponentData,
        ) -> Box<UnitResult>;

        fn is_component_added_to_entity(&self, entity_id: &CXXEntityId, component: String) -> bool;

        fn get_components_of_entity(&self, entity_id: &CXXEntityId) -> Box<StringVecResult>;

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

type ComponentInfoResult = GCSResult<Box<CXXComponentInfo>>;
type UnitResult = GCSResult<()>;
type StringVecResult = GCSResult<Vec<String>>;

pub(crate) struct CXXECSWorld(
    ECSWorld<GCSComponentStorage<CXXComponentInfo, CXXComponentDefinition>, CXXComponentData>,
);

impl CXXECSWorld {
    fn register_component(
        self: &mut CXXECSWorld,
        name: String,
        component_definition: &CXXComponentDefinition,
    ) -> Box<ComponentInfoResult> {
        let result = self
            .0
            .register_component(name.as_str(), component_definition.clone());
        Box::new(match result {
            Ok(info) => ComponentInfoResult::new_result(Box::new(info)),
            Err(error) => ComponentInfoResult::new_error(error),
        })
    }

    fn register_entity(self: &mut CXXECSWorld, id: &CXXEntityId) -> Box<UnitResult> {
        let result = self.0.register_entity(id);
        Box::new(match result {
            Ok(_) => UnitResult::new_result(()),
            Err(err) => UnitResult::new_error(err.to_string()),
        })
    }

    fn set_component_data(
        self: &mut CXXECSWorld,
        entity_id: &CXXEntityId,
        component: String,
        data: &CXXComponentData,
    ) -> Box<UnitResult> {
        let result = self.0.set_component_data(entity_id, component, data);
        Box::new(match result {
            Ok(_) => UnitResult::new_result(()),
            Err(err) => UnitResult::new_error(err.to_string()),
        })
    }

    fn get_components_of_entity(
        self: &CXXECSWorld,
        entity_id: &CXXEntityId,
    ) -> Box<StringVecResult> {
        let result = self.0.get_components_of_entity(entity_id);
        Box::new(match result {
            Ok(value) => StringVecResult::new_result(value),
            Err(err) => StringVecResult::new_error(err.to_string()),
        })
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
        GCSComponentStorage<CXXComponentInfo, CXXComponentDefinition>,
        CXXComponentData,
    >()))
}

unsafe impl ExternType for CXXECSWorld {
    type Id = type_id!("gcs::ffi::ECSWorld");
    type Kind = cxx::kind::Trivial;
}
