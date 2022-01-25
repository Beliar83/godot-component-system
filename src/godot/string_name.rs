#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("string_name.h");
        include!("cxx.h");

        pub type StringName;

        pub fn string_name_to_string(string_name: &StringName) -> String;
    }
}
