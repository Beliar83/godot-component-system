#ifndef GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H
#define GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H

#include "scene/2d/node_2d.h"
#include "scene/3d/spatial.h"
#include "rust/cxx.h"
#include "gcs-cxx/src/world/gcs_world.rs.h"
#include "component_info.h"
#include "component_definition.h"
#include "entity.h"
#include "gcs-cxx/include/godot/string.h" // NOLINT(modernize-deprecated-headers)
#include "gcs_world_base.h"

class GCSWorld2D : public Node2D, GCSWorldBase {
GDCLASS(GCSWorld2D, Node2D);
protected:
    static void _bind_methods();

public:
    Ref<ComponentInfo> register_component(const StringName& name, const Ref<ComponentDefinition>& componentDefinition) override;
    Ref<Entity> create_entity() override;
    void register_entity(Ref<Entity> entity) override;
    Ref<ComponentInfo> register_script_component(const StringName &name, Ref<Script> resource) override;
    PoolStringArray get_components_of_entity(Ref<Entity> entity) override;
};
#endif //GODOT_COMPONENT_SYSTEM_GODOT_WORLD_2D_H
