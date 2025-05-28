use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorTogglePlugin;

impl Plugin for UiInspectorTogglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_global.run_if(
                input_just_pressed(KeyCode::Grave).and_then(not(input_pressed(KeyCode::ShiftLeft))),
            ),
        );
        app.add_systems(
            Update,
            toggle_each.run_if(
                input_just_pressed(KeyCode::Grave).and_then(input_pressed(KeyCode::ShiftLeft)),
            ),
        );
    }
}

fn toggle_global(mut ui_data: ResMut<UIData>) {
    ui_data.windows.global_toggle ^= true;
}
fn toggle_each(mut ui_data: ResMut<UIData>) {
    ui_data.windows.global_toggle ^= true;
    let new = ui_data.windows.global_toggle;
    ui_data.windows.iter_mut().for_each(|window| window.open = new);
}
