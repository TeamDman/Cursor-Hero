use std::io::Write;

use cursor_hero_memory_types::prelude::get_persist_file;
use cursor_hero_memory_types::prelude::Usage;
use cursor_hero_ui_automation::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // loop {

    //     let mouse_position = world_position.xy().neg_y().as_ivec2();
    //     debug!("Worker received click: {:?} {:?}", action, mouse_position);

    //     let elem = find_element_at(mouse_position)?;
    //     info!("{} - {}", elem.get_classname()?, elem.get_name()?);

    //     match get_persist_file(file!(), "vscode.txt", Usage::Persist) {
    //         Ok(mut file) => {
    //             if let Err(e) = file.write_all(snapshot.to_string().as_bytes()) {
    //                 eprintln!("Failed to write to file: {:?}", e);
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to open file: {:?}", e);
    //         }
    //     }
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }
    Ok(())
}
