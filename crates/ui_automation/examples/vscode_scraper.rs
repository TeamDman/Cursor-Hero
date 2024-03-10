use cursor_hero_ui_automation::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", take_snapshot()?);
    Ok(())
}
