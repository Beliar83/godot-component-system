#include "ecs_world_2d.h"
#include "core/engine.h"

void ECSWorld2D::_bind_methods() {
    ClassDB::bind_method(D_METHOD("register_component", "name", "componentDefinition"), &ECSWorld2D::register_component);
    ClassDB::bind_method(D_METHOD("create_entity"), &ECSWorld2D::create_entity);
    ClassDB::bind_method(D_METHOD("register_entity", "entity"), &ECSWorld2D::register_entity);
}

Ref<ComponentInfo>
ECSWorld2D::register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) {
    try {
        rust::box<gcs::ffi::ComponentInfo> info = world->register_component(godot_string_to_rust_string(name),
                                                                        componentDefinition->componentDefinition.operator*());
        auto info_godot = dynamic_cast<ComponentInfo *>(ClassDB::creator<ComponentInfo>());
        info_godot->set_component_info(info.into_raw());
        return {info_godot};
    }
    catch (rust::error& error)
    {
        ERR_PRINT(error.what());
        return {};
    }
}

ECSWorld2D::ECSWorld2D()  : world(gcs::ffi::create_ecs_world()) {

}

Ref<Entity> ECSWorld2D::create_entity() {
    auto rust_entity = world->create_entity();
    auto entity = dynamic_cast<Entity *>(ClassDB::creator<Entity>());
    entity->set_entity_id(rust_entity.into_raw());
    return {entity};
}

void ECSWorld2D::register_entity(Ref<Entity> entity) {
    try { world->register_entity(*entity->entityId); }
    catch (rust::error& error)
    {
        ERR_PRINT(error.what());
    }
}
