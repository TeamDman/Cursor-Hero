use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_toolbelt::types::*;
use leafwing_input_manager::prelude::*;
use cursor_hero_sprint_tool_types::sprint_tool_types_plugin::SprintData;
use crate::prelude::*;

pub struct ZoomToolPlugin;

impl Plugin for ZoomToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoomTool>()
            .add_plugins(InputManagerPlugin::<ZoomToolAction>::default())
            .add_systems(Update, (toolbelt_events, handle_input));
    }
}

#[derive(Component, InspectorOptions, Debug, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
struct ZoomTool;

#[derive(Bundle, Debug)]
struct ZoomToolBundle {
    tool: ZoomTool,
    data: SprintData,
}
impl Default for ZoomToolBundle {
    fn default() -> Self {
        Self {
            tool: ZoomTool::default(),
            data: SprintData {
                value: 1.0,
                default_value: 1.0,
                sprint_value: 2.0,
                sprint_enabled: true,
            },
        }
    }
}

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Default { toolbelt_id }
        | PopulateToolbeltEvent::Inspector { toolbelt_id } = event
        {
            ToolSpawnConfig::<_, ZoomToolAction>::new(
                ZoomToolBundle::default(),
                *toolbelt_id,
                event,
            )
            .guess_name(file!())
            .guess_image(file!(), &asset_server)
            .with_description("Send scroll events")
            .spawn(&mut commands);
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum ZoomToolAction {
    Out,
    In,
}

impl ZoomToolAction {
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Out => GamepadButtonType::DPadLeft.into(),
            Self::In => GamepadButtonType::DPadRight.into(),
        }
    }

    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Out => KeyCode::Home.into(),
            Self::In => KeyCode::End.into(),
        }
    }
}
impl ToolAction for ZoomToolAction {
    fn default_input_map(_event: &PopulateToolbeltEvent) -> Option<InputMap<ZoomToolAction>> {
        let mut input_map = InputMap::default();

        for variant in ZoomToolAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        Some(input_map)
    }
}

fn handle_input(
    tool_query: Query<(&ActionState<ZoomToolAction>, &SprintData), (With<ActiveTool>, With<ZoomTool>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for zoom_tool in tool_query.iter() {
        let (tool_actions, tool_data) = zoom_tool;
        let camera = camera_query.single_mut();
        let mut camera_transform = camera;
        if tool_actions.pressed(ZoomToolAction::Out) {
            let mut scale = camera_transform.scale;
            let diff = 0.1 * time.delta_seconds() * tool_data.value;
            scale *= Vec3::splat(1.0) + Vec2::splat(diff).extend(0.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            camera_transform.scale = scale;
            if tool_actions.just_pressed(ZoomToolAction::Out) {
                info!("Zooming out");
            }
        }
        if tool_actions.pressed(ZoomToolAction::In) {
            let mut scale = camera_transform.scale;
            let diff = 0.1 * time.delta_seconds() * tool_data.value;
            scale *= Vec3::splat(1.0) - Vec2::splat(diff).extend(0.0);
            scale = scale.clamp(Vec3::splat(0.1), Vec3::splat(10.0));
            camera_transform.scale = scale;
            if tool_actions.just_pressed(ZoomToolAction::In) {
                info!("Zooming in");
            }
        }
    }
}
