#ifndef GODOT_COMPONENT_SYSTEM_ENTITY_H
#define GODOT_COMPONENT_SYSTEM_ENTITY_H
#include "core/reference.h"
#include "gcs-cxx/src/world/gcs_world.rs.h"

class GCSWorldBase;

class Entity : public Reference {
    GDCLASS(Entity, Reference)
    friend class GCSWorldBase;

private:
    rust::box<gcs::ffi::EntityId> entityId;
    void set_entity_id(gcs::ffi::EntityId* argEntityId);
    gcs::ffi::EntityId& get_entity_id();

protected:
    static void _bind_methods();

public:
    Entity();

    StringName get_id() const;
};


#endif //GODOT_COMPONENT_SYSTEM_ENTITY_H
