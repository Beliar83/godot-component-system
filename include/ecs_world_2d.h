#ifndef GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H
#define GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H

#include "scene/2d/node_2d.h"
#include "scene/3d/spatial.h"
#include "cxx.h"
#include "ecs_world.rs.h"
#include "component_info.h"
#include "component_definition.h"
#include "entity.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

class ECSWorld2D : public Node2D {
GDCLASS(ECSWorld2D, Node2D);
private:
::rust::Box<::gcs::ffi::ECSWorld> world;

protected:
    static void _bind_methods();

public:
    ECSWorld2D();
    Ref<ComponentInfo> register_component(const StringName& name, const Ref<ComponentDefinition>& componentDefinition);
    Ref<Entity> create_entity();
    void register_entity(Ref<Entity> entity);
};
#endif //GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H
