use std::env;
use std::path::Path;

fn main() {
    let platform = env::var("GODOT_PLATFORM");

    let platform = match platform {
        Ok(platform) => platform,
        Err(err) => {
            panic!("Could not read GODOT_PLATFORM: {}", err)
        }
    };

    let godot_path = env::var("GODOT_PATH");
    let godot_path = match godot_path {
        Ok(path) => path,
        Err(err) => {
            panic!("Could not read GODOT_PATH: {}", err)
        }
    };

    let platform_include_win = format!("{godot_path}/platform/{}", platform);
    cxx_build::bridges(vec![
        "src/godot/variant.rs",
        "src/component/component_value.rs",
        "src/component/component_definition.rs",
        "src/world/gcs_world.rs",
    ])
    .include(Path::new("include"))
    .include(Path::new(&godot_path))
    .include(Path::new(&platform_include_win))
    .files(vec!["src/godot/string.cpp", "src/godot/variant.cpp"])
    .flag_if_supported("-std=g++14")
    .define("RUST_CXX_NO_EXCEPTIONS", None)
    .compile("godot-component-system");

    println!("cargo:rerun-if-changed=src/godot/variant.rs");
    println!("cargo:rerun-if-changed=src/component/component_value.rs");
    println!("cargo:rerun-if-changed=src/component/component_definition.rs");
    println!("cargo:rerun-if-changed=src/component/component_data.rs");
    println!("cargo:rerun-if-changed=src/gcs_world");
    println!("cargo:rerun-if-changed=src/entity.rs");
}
