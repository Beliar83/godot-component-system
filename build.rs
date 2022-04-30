use std::path::Path;

fn main() {
    godot_cxx_build::create_headers_at(Path::new("include"));
}
