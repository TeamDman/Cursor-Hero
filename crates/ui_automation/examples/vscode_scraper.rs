use cursor_hero_ui_automation::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    gather_apps()?
        .into_iter()
        .filter(|x| matches!(x, AppUIElement::VSCode(_)))
        .for_each(|x| println!("{}", x));
    Ok(())
}
