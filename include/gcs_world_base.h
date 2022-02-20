#ifndef GODOT_COMPONENT_SYSTEM_GCS_WORLD_BASE_H
#define GODOT_COMPONENT_SYSTEM_GCS_WORLD_BASE_H


#include <core/script_language.h>
#include "rust/cxx.h"
#include "gcs-cxx/src/world/gcs_world.rs.h"
#include "component_info.h"
#include "component_definition.h"
#include "entity.h"
#include "core/object.h"

class GCSWorldBase {
private:
    ::rust::Box<::gcs::ffi::GCSWorld> world;
protected:
public:
    GCSWorldBase();
    virtual Ref<ComponentInfo> register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) = 0;
    virtual Ref<ComponentInfo> register_script_component(const StringName &name, Ref<Script> resource) = 0;
    virtual Ref<Entity> create_entity() = 0;
    virtual PoolStringArray get_components_of_entity(Ref<Entity> entity) = 0;
    virtual void register_entity(Ref<Entity> entity) = 0;
};


#endif //GODOT_COMPONENT_SYSTEM_GCS_WORLD_BASE_H
