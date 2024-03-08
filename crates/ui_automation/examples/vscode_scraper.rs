use cursor_hero_ui_automation::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    take_snapshot()?
        .into_iter()
        .filter(|x| matches!(x, AppWindow::VSCode(_)))
        .for_each(|x| println!("{}", x));
    Ok(())
}
