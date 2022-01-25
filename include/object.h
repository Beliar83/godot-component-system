#ifndef GODOT_COMPONENT_SYSTEM_OBJECT_H
#define GODOT_COMPONENT_SYSTEM_OBJECT_H
#include <vector>
#include "core/reference.h"
#include "cxx.h"
#include "variant.h"

struct ComponentFieldDefinition;

class GodotComponent : public Reference {
    GDCLASS(GodotComponent, Reference);

public:
    rust::Vec<ComponentFieldDefinition> get_fields() const;
    void set_field(const StringName& name, const Variant &value);

    std::unique_ptr<Variant> get_field(const StringName& name) const;
    void print_definition() const;


protected:
    static void _bind_methods();
};
#endif
