#ifndef GODOT_COMPONENT_SYSTEM_COMPONENT_INFO_H
#define GODOT_COMPONENT_SYSTEM_COMPONENT_INFO_H
#include "core/reference.h"
#include "rust/cxx.h"
#include "gcs-cxx/src/ecs_world.rs.h"

class ECSWorldBase;

class ComponentInfo : public Reference {
    GDCLASS(ComponentInfo, Reference)
    friend class ECSWorldBase;
private:
    rust::box<gcs::ffi::ComponentInfo> componentInfo;

    void set_component_info(gcs::ffi::ComponentInfo* info);
    gcs::ffi::ComponentInfo& getComponentInfo();

protected:
    static void _bind_methods();
public:
    ComponentInfo();
};


#endif //GODOT_COMPONENT_SYSTEM_COMPONENT_INFO_H
