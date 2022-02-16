#ifndef GODOT_COMPONENT_SYSTEM_COMPONENT_DEFINITION_H
#define GODOT_COMPONENT_SYSTEM_COMPONENT_DEFINITION_H

#include "core/reference.h"
#include "gcs-cxx/src/component/component_definition.rs.h"
#include "component_field_definition.h"

class ECSWorldBase;

class ComponentDefinition : public Reference {
GDCLASS(ComponentDefinition, Reference);

friend class ECSWorldBase;
private:
    ::rust::box<gcs::ffi::ComponentDefinition> componentDefinition;

public:
    ComponentDefinition();

    void add_field(Ref<ComponentFieldDefinition> field_definition);

protected:
    static void _bind_methods();
};


#endif //GODOT_COMPONENT_SYSTEM_COMPONENT_DEFINITION_H
