#include "gcs_world_2d.h"
#include "core/engine.h"

void GCSWorld2D::_bind_methods() {
    ClassDB::bind_method(D_METHOD("register_component", "name", "componentDefinition"), &GCSWorld2D::register_component);
    ClassDB::bind_method(D_METHOD("register_script_component", "name", "resource"),
                         &GCSWorld2D::register_script_component);
    ClassDB::bind_method(D_METHOD("create_entity"), &GCSWorld2D::create_entity);
    ClassDB::bind_method(D_METHOD("register_entity", "entity"), &GCSWorld2D::register_entity);
    ClassDB::bind_method(D_METHOD("get_components_of_entity", "entity"), &GCSWorld2D::get_components_of_entity);
}

Ref<ComponentInfo>
GCSWorld2D::register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) {
    return GCSWorldBase::register_component(name, componentDefinition);
}

Ref<Entity> GCSWorld2D::create_entity() {
    return GCSWorldBase::create_entity();
}

void GCSWorld2D::register_entity(Ref<Entity> entity) {
    GCSWorldBase::register_entity(entity);
}

Ref<ComponentInfo> GCSWorld2D::register_script_component(const StringName &name, Ref<Script> resource) {
    return GCSWorldBase::register_script_component(name, resource);
}

PoolStringArray GCSWorld2D::get_components_of_entity(Ref<Entity> entity) {
    return GCSWorldBase::get_components_of_entity(entity);
}
