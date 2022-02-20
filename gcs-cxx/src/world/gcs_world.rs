use std::string::String;

use cxx::{type_id, ExternType};
use gcs::world::component_data_storage::{GCSComponentDataStorage, HasEntityComponentResult};

use gcs::world::component_storage::GCSComponentStorage;
use gcs::world::gcs_world::GCSWorld;

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
        type BoolResult;

        fn is_error(&self) -> bool;
        fn get_result(&self) -> bool;
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
        #[cxx_name = "GCSWorld"]
        type CXXGCSWorld;

        pub(crate) fn create_component_info(hash: u64) -> Box<CXXComponentInfo>;
        fn register_component(
            self: &mut CXXGCSWorld,
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

        fn is_component_added_to_entity(
            &self,
            entity_id: &CXXEntityId,
            component: String,
        ) -> Box<BoolResult>;

        fn get_components_of_entity(&self, entity_id: &CXXEntityId) -> Box<StringVecResult>;

        fn create_entity(self: &mut CXXGCSWorld) -> Box<CXXEntityId>;

        #[cxx_name = "create_gcs_world"]
        pub fn create_cxx_gcs_world() -> Box<CXXGCSWorld>;
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
type BoolResult = GCSResult<bool>;

pub(crate) struct CXXGCSWorld(
    GCSWorld<
        GCSComponentStorage<CXXComponentInfo, CXXComponentDefinition>,
        GCSComponentDataStorage<CXXComponentData>,
    >,
);

impl CXXGCSWorld {
    fn register_component(
        self: &mut CXXGCSWorld,
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

    fn register_entity(self: &mut CXXGCSWorld, id: &CXXEntityId) -> Box<UnitResult> {
        let result = self.0.register_entity(id);
        Box::new(match result {
            Ok(_) => UnitResult::new_result(()),
            Err(err) => UnitResult::new_error(err.to_string()),
        })
    }

    fn set_component_data(
        self: &mut CXXGCSWorld,
        entity_id: &CXXEntityId,
        component: String,
        data: &CXXComponentData,
    ) -> Box<UnitResult> {
        let result = self
            .0
            .set_component_data(entity_id, component.as_str(), data);
        Box::new(match result {
            Ok(_) => UnitResult::new_result(()),
            Err(err) => UnitResult::new_error(err.to_string()),
        })
    }

    fn get_components_of_entity(
        self: &CXXGCSWorld,
        entity_id: &CXXEntityId,
    ) -> Box<StringVecResult> {
        let result = self.0.get_components_of_entity(entity_id);
        Box::new(match result {
            Ok(value) => StringVecResult::new_result(value),
            Err(err) => StringVecResult::new_error(err.to_string()),
        })
    }

    fn is_component_added_to_entity(
        self: &CXXGCSWorld,
        entity_id: &CXXEntityId,
        component: String,
    ) -> Box<BoolResult> {
        Box::new(
            match self
                .0
                .is_component_added_to_entity(entity_id, component.as_str())
            {
                HasEntityComponentResult::EntityNotFound => {
                    BoolResult::new_error("Entity not found".to_string())
                }
                HasEntityComponentResult::EntityDoesNotHaveComponent => {
                    BoolResult::new_result(false)
                }
                HasEntityComponentResult::EntityHasComponent => BoolResult::new_result(true),
            },
        )
    }

    fn create_entity(self: &mut CXXGCSWorld) -> Box<CXXEntityId> {
        Box::new(self.0.create_entity().clone())
    }
}

pub(crate) fn create_cxx_gcs_world() -> Box<CXXGCSWorld> {
    Box::new(CXXGCSWorld(GCSWorld::<
        GCSComponentStorage<CXXComponentInfo, CXXComponentDefinition>,
        GCSComponentDataStorage<CXXComponentData>,
    >::default()))
}

unsafe impl ExternType for CXXGCSWorld {
    type Id = type_id!("gcs::ffi::GCSWorld");
    type Kind = cxx::kind::Trivial;
}
