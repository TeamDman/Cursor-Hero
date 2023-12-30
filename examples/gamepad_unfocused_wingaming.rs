use itertools::Itertools;
use windows::Gaming::Input::{
    GameControllerSwitchPosition, Gamepad as WgiGamepad, GamepadButtons, GamepadReading,
    RawGameController,
};

fn main() -> Result<(), windows::core::Error> {
    let raw_game_controllers = RawGameController::RawGameControllers()?;
    let count = raw_game_controllers.Size()?;
    println!("Found {} gamepads", count);
    let gamepads = (0..count)
        .map(|i| {
            let controller = raw_game_controllers.GetAt(i);
            println!("Gamepad {}:", i);
            controller
        })
        .collect_vec();
    dbg!(gamepads);
    Ok(())
}
