#include "component_definition.h"
#include "component_field_definition.h"

ComponentDefinition::ComponentDefinition() : componentDefinition(gcs::ffi::create_component_definition()) {

}

void ComponentDefinition::add_field(Ref<ComponentFieldDefinition> field_definition) {
    componentDefinition->add_field(field_definition->get_definition());
}

void ComponentDefinition::_bind_methods() {
    ClassDB::bind_method(D_METHOD("add_field", "field_definition"), &ComponentDefinition::add_field);

}
