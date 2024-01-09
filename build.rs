extern crate embed_resource;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("icon.rc");
    }

    // // Get the crate version
    // let version = env::var("CARGO_PKG_VERSION").unwrap();

    // // Define the original and new binary names
    // let original_binary_name = "cursor_hero"; // Replace with your binary name
    // let new_binary_name = format!("{}-{}", original_binary_name, version);

    // // Set the output directory (where Cargo puts the compiled binaries)
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    // // Construct the paths to the original and new binaries
    // let original_binary_path = target_dir.join(original_binary_name);
    // let new_binary_path = target_dir.join(new_binary_name);

    // // Rename the binary
    // Command::new("mv")
    //     .arg(original_binary_path)
    //     .arg(new_binary_path)
    //     .status()
    //     .unwrap();
}
