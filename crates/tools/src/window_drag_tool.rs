use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use cursor_hero_toolbelt::types::*;
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

pub struct WindowDragToolPlugin;

impl Plugin for WindowDragToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WindowDragTool>()
            .register_type::<WindowDragToolInteractable>()
            .add_plugins(InputManagerPlugin::<WindowDragToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct WindowDragTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Default {
            toolbelt_id,
        } = event
        {
            ToolSpawnConfig::<WindowDragTool, WindowDragToolAction>::new(WindowDragTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Drag the window from its body")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum WindowDragToolAction {
    Drag,
}

impl WindowDragToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Drag => GamepadButtonType::LeftThumb.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Drag => MouseButton::Left.into(),
        }
    }
}
impl ToolAction for WindowDragToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<WindowDragToolAction>> {
        let mut input_map = InputMap::default();

        for variant in WindowDragToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

#[derive(Component, Reflect)]
pub struct WindowDragToolInteractable;

fn handle_input(
    tool_query: Query<&ActionState<WindowDragToolAction>, With<ActiveTool>>,
    window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    for action_state in tool_query.iter() {
        if action_state.just_pressed(WindowDragToolAction::Drag) {
            let window_id = window_query.get_single().expect("Need a single window");
            if let Some(winit_window) = winit_windows.get_window(window_id) {
                // winit_window.window_state_lock
                if let Err(x) = winit_window.drag_window() {
                    error!("Failed to drag window: {:?}", x);
                }
            }
            return;
        }
    }
}
