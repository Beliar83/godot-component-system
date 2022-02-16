use std::env;
use std::path::Path;

fn main() {
    let platform = env::var("GODOT_PLATFORM");

    let platform = match platform {
        Ok(platform) => platform,
        Err(err) => {
            panic!("Could not get platform: {}", err)
        }
    };

    let platform_include = format!("../../../platform/{}", platform);
    cxx_build::bridges(vec![
        "src/godot/variant.rs",
        "src/component/component_value.rs",
        "src/component/component_definition.rs",
        "src/ecs_world.rs",
    ])
    .include(Path::new("include"))
    .include(Path::new("../../../"))
    .include(Path::new(&platform_include))
    .files(vec!["src/godot/string.cpp", "src/godot/variant.cpp"])
    .flag_if_supported("-std=g++14")
    .compile("godot-component-system");

    println!("cargo:rerun-if-changed=src/godot/variant.rs");
    println!("cargo:rerun-if-changed=src/component/component_value.rs");
    println!("cargo:rerun-if-changed=src/component/component_definition.rs");
    println!("cargo:rerun-if-changed=src/component/component_data.rs");
    println!("cargo:rerun-if-changed=src/ecs_world.rs");
    println!("cargo:rerun-if-changed=src/entity.rs");
}
