use bevy::input::gamepad::ButtonSettings;
use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::input::gamepad::GamepadSettings;
use bevy::prelude::*;

pub const PRESS_THRESHOLD: f32 = 0.1;
pub const RELEASE_THRESHOLD: f32 = 0.08;

/// Responsible for updating the trigger thresholds for Mining Laser
/// https://github.com/Leafwing-Studios/leafwing-input-manager/issues/405
pub fn update_gamepad_settings(
    mut gamepad_events: EventReader<GamepadConnectionEvent>,
    mut gamepad_settings: ResMut<GamepadSettings>,
) {
    gamepad_events.read().for_each(|event| {
        info!("Updating Gamepad Settings");

        gamepad_settings.button_settings.insert(
            GamepadButton {
                gamepad: event.gamepad,
                button_type: GamepadButtonType::RightTrigger2,
            },
            ButtonSettings::new(PRESS_THRESHOLD, RELEASE_THRESHOLD).unwrap(), //Ok because this would be programmer error
        );

        gamepad_settings.button_settings.insert(
            GamepadButton {
                gamepad: event.gamepad,
                button_type: GamepadButtonType::LeftTrigger2,
            },
            ButtonSettings::new(PRESS_THRESHOLD, RELEASE_THRESHOLD).unwrap(), //Ok because this would be programmer error
        );
    });
}
