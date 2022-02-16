#include <memory>
#include "core/class_db.h"
#include "core/script_language.h"
#include "gcs-cxx/include/godot/string.h" // NOLINT(modernize-deprecated-headers)
#include "component.h"

void GodotComponent::set_field(const rust::string& name, const Variant &value) {
    this->set(string_name_from_rust_string(name), value);
}

std::unique_ptr<Variant> GodotComponent::get_field(const rust::string& name) const {
    return std::make_unique<Variant>(this->get(string_name_from_rust_string(name)));
}

void GodotComponent::_bind_methods() {
}