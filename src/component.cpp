#include <memory>
#include "core/class_db.h"
#include "core/script_language.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)
#include "component.rs.h"
#include "component.h"

rust::Vec<ComponentFieldDefinition> GodotComponent::get_fields() const {
    List<PropertyInfo> list;
    if (this->get_script_instance()) {
        this->get_script_instance()->get_property_list(&list);
    } else {
        this->get_property_list(&list);
    }

    auto field_vec = new rust::Vec<ComponentFieldDefinition>();
    for (int i = 0; i < list.size(); ++i) {
        PropertyInfo info = list[i];
        auto definition = create_component_field_definition(godot_string_to_rust_string(info.name), info.type);
        field_vec->push_back(*definition);
    }
    return *field_vec;
}

void GodotComponent::set_field(const rust::string& name, const Variant &value) {
    this->set(string_name_from_rust_string(name), value);
}

std::unique_ptr<Variant> GodotComponent::get_field(const rust::string& name) const {
    return std::make_unique<Variant>(this->get(string_name_from_rust_string(name)));
}

void GodotComponent::print_definition() const {
    print_definition_gd(*this);
}

void GodotComponent::_bind_methods() {
    ClassDB::bind_method(D_METHOD("print_definition"), &GodotComponent::print_definition);
}
