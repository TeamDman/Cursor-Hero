use bevy::math;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::Pos2;
use bevy_egui::EguiContexts;
use bevy_xpbd_2d::components::Collider;
use bevy_xpbd_2d::components::RigidBody;
use cursor_hero_bevy::NegativeYVec2;
use cursor_hero_camera::camera_plugin::MainCamera;
use cursor_hero_toolbelt_types::prelude::*;
use cursor_hero_tools::prelude::*;

use crate::hover_ui_automation_plugin::HoverInfo;

pub struct HoverToolPlugin;

impl Plugin for HoverToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HoverTool>();
        app.register_type::<WindowBrick>();
        app.add_systems(Update, toolbelt_events);
        app.add_systems(Update, tick);
        app.add_systems(Update, ui);
        app.add_systems(Startup, spawn_brick);
    }
}

#[derive(Component, Reflect, Default)]
struct HoverTool;

fn toolbelt_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<PopulateToolbeltEvent>,
) {
    for event in reader.read() {
        if let PopulateToolbeltEvent::Inspector { toolbelt_id } = event {
            ToolSpawnConfig::<HoverTool, NoInputs>::new(HoverTool, *toolbelt_id, event)
                .guess_name(file!())
                .guess_image(file!(), &asset_server, "png")
                .with_description("UI hover visuals")
                .spawn(&mut commands);
        }
    }
}

fn tick(
    tool_query: Query<Entity, (With<ActiveTool>, With<HoverTool>)>,
    mut hover_info: ResMut<HoverInfo>,
) {
    if tool_query.iter().next().is_some() {
        if !hover_info.is_enabled() {
            info!("Enabling hover info");
            hover_info.set_enabled(true);
        }
    } else if hover_info.is_enabled() {
        info!("Disabling hover info");
        hover_info.set_enabled(false);
    }
}

#[derive(Component, Debug, Reflect)]
struct WindowBrick;
fn spawn_brick(mut commands: Commands) {
    commands.spawn((
        WindowBrick,
        Name::new("AHOY!"),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                color: Color::GRAY,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, -200.0, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(100.0, 100.0),
    ));
    commands.spawn((
        WindowBrick,
        Name::new("HOWDY!!"),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, -500.0, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(100.0, 100.0),
    ));
}

fn ui(
    mut contexts: EguiContexts,
    time: Res<Time>,
    window_brick_query: Query<(&Name, &GlobalTransform), With<WindowBrick>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
) {
    let Ok(camera) = camera_query.get_single() else {
        warn!("No camera found");
        return;
    };
    let (camera_transform, camera) = camera;

    for brick in window_brick_query.iter() {
        let (name, global_transform) = brick;
        let title = name.to_string();

        let Some(pos) = camera.world_to_viewport(camera_transform, global_transform.translation()) else {
            warn!("No world position found");
            continue;
        };

        egui::Window::new(title)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Hello, world!");
            });
    }
}
