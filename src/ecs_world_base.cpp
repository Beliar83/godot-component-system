#include "ecs_world_base.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

ECSWorldBase::ECSWorldBase() : world(gcs::ffi::create_ecs_world()) {

}

Ref<ComponentInfo>
ECSWorldBase::register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) {
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

Ref<Entity> ECSWorldBase::create_entity() {
    auto rust_entity = world->create_entity();
    auto entity = dynamic_cast<Entity *>(ClassDB::creator<Entity>());
    entity->set_entity_id(rust_entity.into_raw());
    return {entity};
}

void ECSWorldBase::register_entity(Ref<Entity> entity) {
    try { world->register_entity(*entity->entityId); }
    catch (rust::error& error)
    {
        ERR_PRINT(error.what());
    }
}
