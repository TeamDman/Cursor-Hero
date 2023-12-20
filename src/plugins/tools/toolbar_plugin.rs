use bevy::{prelude::*, transform::TransformSystem};

use bevy_xpbd_2d::PhysicsSet;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::plugins::character_plugin::Character;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbar>()
            .register_type::<Tool>()
            .register_type::<ToolbarEntry>()
            .add_plugins(InputManagerPlugin::<ToolbarAction>::default())
            .add_systems(Startup, (apply_deferred, setup).chain())
            .add_systems(Update, toolbar_visibility)
            .add_systems(
                PostUpdate,
                toolbar_follow
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ToolbarAction {
    Show,
}

impl ToolbarAction {
    fn default_mkb_binding(&self) -> UserInput {
        match self {
            Self::Show => UserInput::Single(InputKind::Keyboard(KeyCode::AltLeft)),
        }
    }
    fn default_gamepad_binding(&self) -> UserInput {
        match self {
            Self::Show => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger))
            }
        }
    }
    fn default_input_map() -> InputMap<ToolbarAction> {
        let mut input_map = InputMap::default();

        for variant in ToolbarAction::variants() {
            input_map.insert(variant.default_mkb_binding(), variant.clone());
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, Reflect)]
pub struct Toolbar;

#[derive(Component, Reflect)]
pub struct Toolbelt(Entity);

#[derive(Component, Reflect)]
pub struct ToolbarEntry(Entity);

#[derive(Component, Reflect)]
pub struct Tool(pub Handle<Image>);

pub fn setup(
    mut commands: Commands,
    tools: Query<(Entity, &Name, &Tool)>,
    character: Query<Entity, With<Character>>,
) {
    let mut parent = commands.spawn((
        SpatialBundle {
            visibility: Visibility::Hidden,
            ..default()
        },
        Toolbar,
        Name::new("Toolbar"),
        InputManagerBundle::<ToolbarAction> {
            input_map: ToolbarAction::default_input_map(),
            // action_state: ActionState::default(),
            ..Default::default()
        },
    ));
    parent.with_children(|parent| {
        let count = tools.iter().count();
        info!("Found {} tools", count);
        for (i, (t_e, t_name, t)) in tools.iter().enumerate() {
            info!("Adding toolbar entry: {}", t_name.as_str());
            let angle = 360.0 / (count as f32) * i as f32;
            let dist = 250.0;
            let x = angle.to_radians().cos() * dist;
            let y = angle.to_radians().sin() * dist;
            parent.spawn((
                ToolbarEntry(t_e),
                Name::new(format!("Toolbar Entry - {}", t_name.as_str())),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 200.0)),
                    texture: t.0.clone(),
                    ..default()
                },
            ));
        }
    });

    let toolbar_id = parent.id();
    drop(parent);
    commands
        .entity(character.single())
        .insert(Toolbelt(toolbar_id));
}

pub fn toolbar_visibility(
    mut query: Query<&mut Visibility, With<Toolbar>>,
    toolbar_actions: Query<&ActionState<ToolbarAction>>,
) {
    if let Ok(action_state) = toolbar_actions.get_single() {
        if action_state.just_pressed(ToolbarAction::Show) {
            info!("Show toolbar");
            for mut visibility in query.iter_mut() {
                *visibility = Visibility::Visible;
            }
        } else if action_state.just_released(ToolbarAction::Show) {
            info!("Hide toolbar");
            for mut visibility in query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn toolbar_follow(
    mut query: Query<&mut Transform, (With<Toolbar>, Without<Toolbelt>)>,
    follow: Query<(&Toolbelt, &Transform), Without<Toolbar>>,
) {
    for f in follow.iter() {
        let target = f.0 .0;
        for t in query.get_mut(target).iter_mut() {
            t.translation = f.1.translation;
        }
    }
}
