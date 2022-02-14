#include "ecs_world_2d.h"
#include "core/engine.h"

void ECSWorld2D::_bind_methods() {
    ClassDB::bind_method(D_METHOD("register_component", "name", "componentDefinition"), &ECSWorld2D::register_component);
    ClassDB::bind_method(D_METHOD("register_script_component", "name", "resource"),
                         &ECSWorld2D::register_script_component);
    ClassDB::bind_method(D_METHOD("create_entity"), &ECSWorld2D::create_entity);
    ClassDB::bind_method(D_METHOD("register_entity", "entity"), &ECSWorld2D::register_entity);
}

Ref<ComponentInfo>
ECSWorld2D::register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) {
    return ECSWorldBase::register_component(name, componentDefinition);
}

Ref<Entity> ECSWorld2D::create_entity() {
    return ECSWorldBase::create_entity();
}

void ECSWorld2D::register_entity(Ref<Entity> entity) {
    ECSWorldBase::register_entity(entity);
}

Ref<ComponentInfo> ECSWorld2D::register_script_component(const StringName &name, Ref<Script> resource) {
    return ECSWorldBase::register_script_component(name, resource);
}
