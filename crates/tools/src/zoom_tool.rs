use bevy::prelude::*;
use cursor_hero_character::character_plugin::Character;
use leafwing_input_manager::prelude::*;

use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_toolbelt::types::*;
use cursor_hero_winutils::win_mouse::scroll_wheel_down;
use cursor_hero_winutils::win_mouse::scroll_wheel_up;

use crate::prelude::*;

pub struct ZoomToolPlugin;

impl Plugin for ZoomToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoomTool>()
            .add_plugins(InputManagerPlugin::<ZoomToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, Reflect, Default)]
struct ZoomTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ToolbeltPopulateEvent>,
) {
    for event in reader.read() {
        if let ToolbeltPopulateEvent::Default {
            toolbelt_id,
        }
        | ToolbeltPopulateEvent::Inspector {
            toolbelt_id,
        } = event
        {
            ToolSpawnConfig::<ZoomTool, ZoomToolAction>::new(ZoomTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server)
                .with_description("Send scroll events")
                .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ZoomToolAction {
    ZoomOut,
    ZoomIn,
    ScrollUp,
    ScrollDown,
}

impl ZoomToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::ZoomOut => GamepadButtonType::DPadLeft.into(),
            Self::ZoomIn => GamepadButtonType::DPadRight.into(),
            Self::ScrollUp => GamepadButtonType::DPadUp.into(),
            Self::ScrollDown => GamepadButtonType::DPadDown.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::ZoomOut => KeyCode::Home.into(),
            Self::ZoomIn => KeyCode::End.into(),
            Self::ScrollUp => KeyCode::PageDown.into(),
            Self::ScrollDown => KeyCode::PageUp.into(),
        }
    }
}
impl ToolAction for ZoomToolAction {
    fn default_input_map(_event: &ToolbeltPopulateEvent) -> Option<InputMap<ZoomToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ZoomToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

fn handle_input(
    tools: Query<(&ActionState<ZoomToolAction>, Option<&ActiveTool>, &Parent)>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    toolbelts: Query<&Parent, With<Toolbelt>>,
    mut character_query: Query<(&mut Character, &Children)>,
) {
    for (t_act, t_enabled, t_parent) in tools.iter() {
        if t_enabled.is_none() {
            continue;
        }
        let belt_parent = toolbelts
            .get(t_parent.get())
            .expect("Toolbelt should have a parent")
            .get();
        let mut modifier = 1.0;
        if let Ok((character, _)) = character_query.get_mut(belt_parent) {
            modifier = character.zoom_speed;
        }
        if t_act.pressed(ZoomToolAction::ZoomOut) {
            let mut scale = cam.single_mut().scale;
            scale *=
                Vec3::splat(1.0) + Vec2::splat(0.1 * time.delta_seconds() * modifier).extend(0.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            cam.single_mut().scale = scale;
            if t_act.just_pressed(ZoomToolAction::ZoomOut) {
                info!("Zooming out");
            }
        }
        if t_act.pressed(ZoomToolAction::ZoomIn) {
            let mut scale = cam.single_mut().scale;
            scale *=
                Vec3::splat(1.0) - Vec2::splat(0.1 * time.delta_seconds() * modifier).extend(0.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            cam.single_mut().scale = scale;
            if t_act.just_pressed(ZoomToolAction::ZoomIn) {
                info!("Zooming in");
            }
        }
        if t_act.pressed(ZoomToolAction::ScrollUp) {
            match scroll_wheel_up() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error scrolling up: {:?}", e);
                }
            }
            if t_act.just_pressed(ZoomToolAction::ScrollUp) {
                info!("Scrolling up");
            }
        }
        if t_act.pressed(ZoomToolAction::ScrollDown) {
            match scroll_wheel_down() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error scrolling down: {:?}", e);
                }
            }
            if t_act.just_pressed(ZoomToolAction::ScrollDown) {
                info!("Scrolling down");
            }
        }
    }
}
