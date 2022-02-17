#include "ecs_world_base.h"
#include "gcs-cxx/include/godot/string.h" // NOLINT(modernize-deprecated-headers)

ECSWorldBase::ECSWorldBase() : world(gcs::ffi::create_ecs_world()) {

}

Ref<ComponentInfo>
ECSWorldBase::register_component(const StringName &name, const Ref<ComponentDefinition> &componentDefinition) {
    auto result = world->register_component(godot_string_to_rust_string(name),
                                                                        componentDefinition->componentDefinition.operator*());
    ERR_FAIL_COND_V_MSG(result->is_error(), {}, string_name_from_rust_string(result->get_error()));
    auto info_godot = SAFE_CAST<ComponentInfo *>(ClassDB::creator<ComponentInfo>());
    info_godot->set_component_info(result->get_result().into_raw());
    return {info_godot};
}

Ref<Entity> ECSWorldBase::create_entity() {
    auto rust_entity = world->create_entity();
    auto entity = SAFE_CAST<Entity *>(ClassDB::creator<Entity>());
    entity->set_entity_id(rust_entity.into_raw());
    return {entity};
}

void ECSWorldBase::register_entity(Ref<Entity> entity) {
    auto result= world->register_entity(entity->get_entity_id());

    ERR_FAIL_COND_MSG(result->is_error(), string_name_from_rust_string(result->get_error()));
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
    auto definition_object = SAFE_CAST<ComponentDefinition *>(ClassDB::creator<ComponentDefinition>());
    definition_object->componentDefinition.swap(definition);

    return register_component(name, definition_object);
}

PoolStringArray ECSWorldBase::get_components_of_entity(Ref<Entity> entity) {
    auto array = PoolStringArray();
    auto result = world->get_components_of_entity(entity->get_entity_id());
    ERR_FAIL_COND_V_MSG(result->is_error(), {}, string_name_from_rust_string(result->get_error()));

    for (const auto& component : result->get_result()) {
        array.push_back(string_name_from_rust_string(component));
    }
    return array;
}
