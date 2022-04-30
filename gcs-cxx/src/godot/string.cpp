#include <core/ustring.h>
#include <string>
#include "rust/cxx.h"
#include "godot-cxx/string.h" // NOLINT(modernize-deprecated-headers)
#include <core/string_name.h>

rust::string godot_string_to_rust_string(const String &value) {
    auto as_wstring = std::wstring(value.c_str());
    auto as_string = std::string(as_wstring.begin(), as_wstring.end());
    return as_string;
}

rust::string string_name_to_string(const StringName &string_name) {
    String string = string_name;
    auto as_wstring = std::wstring(string.c_str());
    auto as_string = std::string(as_wstring.begin(), as_wstring.end());
    return as_string;
}

const StringName &string_name_from_rust_string(rust::string string) {
    return *(new StringName(string.c_str()));
}
