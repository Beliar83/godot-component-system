#include "entity.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

Entity::Entity() : entityId(gcs::ffi::create_entity()) {

}

void Entity::set_entity_id(gcs::ffi::EntityId* argEntityId) {
    entityId = rust::box<gcs::ffi::EntityId>::from_raw(argEntityId);
}

gcs::ffi::EntityId &Entity::get_entity_id() {
    return entityId.operator*();
}

StringName Entity::get_id() const {
    return string_name_from_rust_string(entityId->as_string());
}

void Entity::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_id"), &Entity::get_id);
}
