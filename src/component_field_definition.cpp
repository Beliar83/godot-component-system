#include "component_field_definition.h"
#include "component_definition.rs.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

StringName ComponentFieldDefinition::get_name() const {
    return string_name_from_rust_string(componentFieldDefinition.name);
}

void ComponentFieldDefinition::set_name(const StringName& name) {
    componentFieldDefinition.name = godot_string_to_rust_string(name);
}

Variant::Type ComponentFieldDefinition::get_type() const {
    return componentFieldDefinition.field_type;
}

void ComponentFieldDefinition::set_type(Variant::Type type) {
    componentFieldDefinition.field_type = type;
}

void ComponentFieldDefinition::_bind_methods() {
    ClassDB::bind_method(D_METHOD("set_name", "name"), &ComponentFieldDefinition::set_name);
    ClassDB::bind_method(D_METHOD("get_name"), &ComponentFieldDefinition::get_name);
    ClassDB::bind_method(D_METHOD("set_field_type", "type"), &ComponentFieldDefinition::set_type);
    ClassDB::bind_method(D_METHOD("get_field_type"), &ComponentFieldDefinition::get_type);

    ADD_PROPERTY(PropertyInfo(Variant::STRING, "name"), "set_name", "get_name");
    ADD_PROPERTY(PropertyInfo(Variant::INT, "field_type", PROPERTY_HINT_ENUM, "NIL,BOOL,INT,REAL,STRING"), "set_field_type", "get_field_type");
}

gcs::ffi::ComponentFieldDefinition ComponentFieldDefinition::get_definition() {
    return componentFieldDefinition;
}

ComponentFieldDefinition::ComponentFieldDefinition() : componentFieldDefinition(
        gcs::ffi::create_component_field_definition()) {

}
