#ifndef GODOT_COMPONENT_SYSTEM_STRING_H
#define GODOT_COMPONENT_SYSTEM_STRING_H
#include "core/string_name.h"
#include "rust/cxx.h"

rust::string godot_string_to_rust_string(const String& value);
rust::string string_name_to_string(const StringName &string_name);
const StringName &string_name_from_rust_string(rust::string string);
#endif //GODOT_COMPONENT_SYSTEM_STRING_H
