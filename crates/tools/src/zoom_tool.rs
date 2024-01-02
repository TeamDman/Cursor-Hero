use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::scroll_wheel_down;
use cursor_hero_winutils::win_mouse::scroll_wheel_up;

pub struct ZoomToolPlugin;

impl Plugin for ZoomToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoomTool>()
            .add_plugins(InputManagerPlugin::<ToolAction>::default())
            .add_systems(
                Update,
                (spawn_tool_event_responder_update_system, handle_input),
            );
    }
}

#[derive(Component, Reflect)]
pub struct ZoomTool;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolAction {
    ZoomIn,
    ZoomOut,
    ScrollUp,
    ScrollDown,
}

impl ToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::ZoomIn => GamepadButtonType::East.into(),
            Self::ZoomOut => GamepadButtonType::North.into(),
            Self::ScrollUp => GamepadButtonType::West.into(),
            Self::ScrollDown => GamepadButtonType::South.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::ZoomIn => KeyCode::PageDown.into(),
            Self::ZoomOut => KeyCode::PageUp.into(),
            Self::ScrollUp => KeyCode::Up.into(),
            Self::ScrollDown => KeyCode::Down.into(),
        }
    }

    fn default_input_map() -> InputMap<ToolAction> {
        let mut input_map = InputMap::default();

        for variant in ToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

fn spawn_tool_event_responder_update_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltEvent>,
) {
    for e in reader.read() {
        match e {
            ToolbeltEvent::Populate(toolbelt_id) => {
                commands.entity(*toolbelt_id).with_children(|t_commands| {
                    t_commands.spawn((
                        ToolBundle {
                            name: Name::new(format!("Zoom Tool")),
                            sprite_bundle: SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(100.0, 100.0)),
                                    ..default()
                                },
                                texture: asset_server.load("textures/zoom_tool.png"),
                                ..default()
                            },
                            ..default()
                        },
                        InputManagerBundle::<ToolAction> {
                            input_map: ToolAction::default_input_map(),
                            ..default()
                        },
                        ToolActiveTag,
                        ZoomTool,
                    ));
                });
                info!("Added tool to toolbelt {:?}", toolbelt_id);
            }
        }
    }
}

fn handle_input(
    tools: Query<(&ActionState<ToolAction>, Option<&ToolActiveTag>)>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    for (t_act, t_enabled) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        if t_act.pressed(ToolAction::ZoomIn) {
            let mut scale = cam.single_mut().scale;
            scale *= Vec2::splat(1.1).extend(1.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            cam.single_mut().scale = scale;
            if t_act.just_pressed(ToolAction::ZoomIn) {
                info!("Zooming in");
            }
        }
        if t_act.pressed(ToolAction::ZoomOut) {
            let mut scale = cam.single_mut().scale;
            scale *= Vec2::splat(0.9).extend(1.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            cam.single_mut().scale = scale;
            if t_act.just_pressed(ToolAction::ZoomOut) {
                info!("Zooming out");
            }
        }
        if t_act.pressed(ToolAction::ScrollUp) {
            match scroll_wheel_up() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error scrolling up: {:?}", e);
                }
            }
            if t_act.just_pressed(ToolAction::ScrollUp) {
                info!("Scrolling up");
            }
        }
        if t_act.pressed(ToolAction::ScrollDown) {
            match scroll_wheel_down() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error scrolling down: {:?}", e);
                }
            }
            if t_act.just_pressed(ToolAction::ScrollDown) {
                info!("Scrolling down");
            }
        }
    }
}
