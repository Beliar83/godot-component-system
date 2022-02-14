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
    try { world->register_entity(entity->get_entity_id()); }
    catch (rust::error& error)
    {
        ERR_PRINT(error.what());
    }
}

Ref<ComponentInfo> ECSWorldBase::register_script_component(const StringName &name, Ref<Script> resource) {
    auto property_list = List<PropertyInfo>();
    resource->get_script_property_list(&property_list);
    auto definition = gcs::ffi::create_component_definition();
    for (int i=0; i < property_list.size(); ++i) {
        auto field_definition = gcs::ffi::create_component_field_definition();
        auto property_info = property_list[i];
        field_definition.name = godot_string_to_rust_string(property_info.name);
        field_definition.field_type = property_info.type;
        definition->add_field(field_definition);
    }
    auto definition_object = dynamic_cast<ComponentDefinition *>(ClassDB::creator<ComponentDefinition>());
    definition_object->componentDefinition.swap(definition);

    return register_component(name, definition_object);
}

PoolStringArray ECSWorldBase::get_components_of_entity(Ref<Entity> entity) {
    try {
        auto array = PoolStringArray();
        auto components = world->get_components_of_entity(entity->get_entity_id());
        for (const auto& component : components) {
            array.push_back(string_name_from_rust_string(component));
        }
        return array;
    }
    catch (rust::error& error)
    {
        ERR_PRINT(error.what());
        return {};
    }
}
