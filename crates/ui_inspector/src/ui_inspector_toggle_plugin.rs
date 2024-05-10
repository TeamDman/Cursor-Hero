use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorTogglePlugin;

impl Plugin for UiInspectorTogglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_ui_inspector.run_if(input_just_pressed(KeyCode::Grave)),
        );
    }
}

fn toggle_ui_inspector(mut ui_data: ResMut<UIData>) {
    ui_data.visible ^= true;
}
