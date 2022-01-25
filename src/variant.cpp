#include "variant.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

void yes_cxx_variant_can_be_a_unique_ptr_target(std::unique_ptr<Variant> variant) {}

int64_t variant_as_i64(const Variant &variant) {
    return variant.operator int64_t();
}

rust::string variant_as_string(const Variant &variant){
    auto as_string = variant.operator String();
    return godot_string_to_rust_string(as_string);
}

bool variant_as_bool(const Variant &variant) {
    return variant.operator bool();
}

double variant_as_f64(const Variant &variant) {
    return variant.operator double();
}
