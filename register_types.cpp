#include "register_types.h"
#include "core/class_db.h"
#include "component.h"
#include "ecs_world_2d.h"
#include "component_field_definition.h"
#include "component_definition.h"
#include "component_info.h"

void register_godot_component_system_types() {
    ClassDB::register_class<GodotComponent>();
    ClassDB::register_class<ECSWorld2D>();
    ClassDB::register_class<ComponentFieldDefinition>();
    ClassDB::register_class<ComponentDefinition>();
    ClassDB::register_class<Entity>();
    ClassDB::register_class<ComponentInfo>();
}
void unregister_godot_component_system_types() {
}