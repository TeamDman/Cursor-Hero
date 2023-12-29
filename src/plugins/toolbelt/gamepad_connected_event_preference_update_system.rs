use bevy::{prelude::*, input::gamepad::{GamepadConnectionEvent, GamepadSettings, ButtonSettings}};

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
            ButtonSettings::new(0.1, 0.08).unwrap(), //Ok because this would be programmer error
        );

        gamepad_settings.button_settings.insert(
            GamepadButton {
                gamepad: event.gamepad,
                button_type: GamepadButtonType::LeftTrigger2,
            },
            ButtonSettings::new(0.1, 0.08).unwrap(), //Ok because this would be programmer error
        );
    });
}