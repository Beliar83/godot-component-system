#include <core/ustring.h>
#include <string>
#include "cxx.h"

rust::string godot_string_to_rust_string(const String &value) {
    auto as_wstring = std::wstring(value.c_str());
    auto as_string = std::string(as_wstring.begin(), as_wstring.end());
    return as_string;
}
