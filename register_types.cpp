#include "register_types.h"
#include "core/class_db.h"
#include "component.rs.h."

void register_godot_component_system_types() {
    ClassDB::register_class<GodotComponent>();
}
void unregister_godot_component_system_types() {
}