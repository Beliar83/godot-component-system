#ifndef GODOT_COMPONENT_SYSTEM_ECS_WORLD_BASE_H
#define GODOT_COMPONENT_SYSTEM_ECS_WORLD_BASE_H


#include <core/script_language.h>
#include "cxx.h"
#include "ecs_world.rs.h"
#include "component_info.h"
#include "component_definition.h"
#include "entity.h"
#include "core/object.h"

class ECSWorldBase {
private:
    ::rust::Box<::gcs::ffi::ECSWorld> world;
protected:
public:
    ECSWorldBase();
    virtual Ref<ComponentInfo> register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) = 0;
    virtual Ref<ComponentInfo> register_script_component(const StringName &name, Ref<Script> resource) = 0;
    virtual Ref<Entity> create_entity() = 0;
    virtual void register_entity(Ref<Entity> entity) = 0;
};


#endif //GODOT_COMPONENT_SYSTEM_ECS_WORLD_BASE_H
