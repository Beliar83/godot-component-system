#include "godot/variant.h"
#include "godot/string.h" // NOLINT(modernize-deprecated-headers)

namespace gcs {
    namespace ffi {
        int64_t variant_as_i64(const Variant &variant) {
            return variant.operator int64_t();
        }

        rust::string variant_as_string(const Variant &variant) {
            auto as_string = variant.operator String();
            return godot_string_to_rust_string(as_string);
        }

        bool variant_as_bool(const Variant &variant) {
            return variant.operator bool();
        }

        double variant_as_f64(const Variant &variant) {
            return variant.operator double();
        }

        const Variant &empty_variant() {
            return *(new Variant());
        }

        const Variant &variant_from_i64(int64_t value) {
            return *(new Variant(value));
        }

        const Variant &variant_from_string(rust::string value) {
            return *(new Variant(string_name_from_rust_string(value)));
        }

        const Variant &variant_from_bool(bool value) {
            return *(new Variant(value));
        }

        const Variant &variant_from_f64(double value) {
            return *(new Variant(value));
        }
    }
}