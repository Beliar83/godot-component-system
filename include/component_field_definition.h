#ifndef GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
#define GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
#include "core/reference.h"
#include "gcs-cxx/src/component/component_definition.rs.h"


class ComponentFieldDefinition : public Reference {
GDCLASS(ComponentFieldDefinition, Reference);
private:
    gcs::ffi::ComponentFieldDefinition componentFieldDefinition;
protected:
    static void _bind_methods();

public:
    ComponentFieldDefinition();

    StringName get_name() const;
    void set_name(const StringName& name);

    Variant::Type get_type() const;
    void set_type(Variant::Type type);

    gcs::ffi::ComponentFieldDefinition get_definition();
};


#endif //GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
