extern crate embed_resource;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the crate version
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    // set the CURSOR_HERO_VERSION environment variable
    println!("cargo:rustc-env=CURSOR_HERO_VERSION={}", version);

    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("icon.rc");
    }

    // Check if it is a release build
    // let profile = env::var("PROFILE").unwrap();
    // if profile == "release" {
    //     // Define the original and new binary names
    //     let original_binary_name = "cursor_hero"; // Replace with your binary name
    //     let new_binary_name = format!("{}_v{}.exe", original_binary_name, version);

    //     // Set the output directory (where Cargo puts the compiled binaries)
    //     let out_dir = env::var("OUT_DIR").unwrap();
    //     let target_dir = Path::new(&out_dir)
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap();

    //     // Construct the paths to the original and new binaries
    //     let original_binary_path = target_dir.join(original_binary_name).with_extension("exe");
    //     let new_binary_path = target_dir.join(new_binary_name).with_extension("exe");

    //     // Copy the binary
    //     fs::copy(&original_binary_path, &new_binary_path).expect("Failed to copy the file");
    // }
}
