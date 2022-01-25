#ifndef GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
#define GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
#include "core/string_name.h"
#include "core/variant.h"
#include "cxx.h"

class ComponentFieldDefinition
{
public:
    rust::string name;
    Variant::Type type;
};

#endif //GODOT_COMPONENT_SYSTEM_COMPONENT_FIELD_DEFINITION_H
