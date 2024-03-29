#ifndef GODOT_COMPONENT_SYSTEM_COMPONENT_H
#define GODOT_COMPONENT_SYSTEM_COMPONENT_H
#include <vector>
#include "core/reference.h"
#include "rust/cxx.h"
#include "gcs-cxx/include/godot/variant.h"

class GodotComponent : public Reference {
    GDCLASS(GodotComponent, Reference);

public:
    void set_field(const rust::string& name, const Variant &value);

    std::unique_ptr<Variant> get_field(const rust::string& name) const;

protected:
    static void _bind_methods();
};


#endif
