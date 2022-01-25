#include "string_name.rs.h"

rust::string string_name_to_string(const StringName &string_name) {
    String string = string_name;
    auto as_wstring = std::wstring(string.c_str());
    auto as_string = std::string(as_wstring.begin(), as_wstring.end());
    return as_string;
}

const StringName &string_name_from_rust_string(rust::string string) {
    return *(new StringName(string.c_str()));
}
