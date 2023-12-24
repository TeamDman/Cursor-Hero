use bevy::{prelude::*, transform::TransformSystem};

use bevy_xpbd_2d::{components::LinearVelocity, PhysicsSet};
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use crate::plugins::character_plugin::{Character, CharacterSystemSet};

pub struct ToolbarPlugin;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolbarSystemSet {
    Spawn,
}

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Toolbar>()
            .register_type::<Tool>()
            .register_type::<ToolbarEntry>()
            .register_type::<CirclularDistributionProperties>()
            .register_type::<ActiveToolTag>()
            .register_type::<HoveredToolTag>()
            .add_plugins(InputManagerPlugin::<ToolbarAction>::default())
            .add_systems(
                Startup,
                (
                    apply_deferred,
                    setup
                        .in_set(ToolbarSystemSet::Spawn)
                        .after(CharacterSystemSet::Spawn),
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    (toolbar_visibility, toolbar_hover).chain(),
                    circle_radius_update,
                ),
            )
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
            input_map.insert(variant.default_mkb_binding(), variant);
            input_map.insert(variant.default_gamepad_binding(), variant);
        }
        input_map
    }
}

#[derive(Component, Reflect)]
pub struct Toolbar {
    follow: Option<Entity>,
}

#[derive(Component, Reflect, Clone, Copy)]
pub struct CirclularDistributionProperties {
    radius: f32,
}
impl Default for CirclularDistributionProperties {
    fn default() -> Self {
        Self { radius: 200.0 }
    }
}

#[derive(Component, Reflect)]
pub struct Tool(pub Handle<Image>);

#[derive(Component, Reflect)]
pub struct ToolbarEntry(Entity);

#[derive(Component, Reflect)]
pub struct ActiveToolTag;

#[derive(Component, Reflect)]
pub struct HoveredToolTag;

fn setup(
    mut commands: Commands,
    tools: Query<(Entity, &Name, &Tool)>,
    character: Query<Entity, With<Character>>,
) {
    let character_id = character.single();
    let circle = CirclularDistributionProperties::default();
    let mut parent = commands.spawn((
        SpatialBundle {
            visibility: Visibility::Hidden,
            ..default()
        },
        Toolbar {
            follow: Some(character_id),
        },
        circle,
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
            let x = angle.to_radians().cos() * circle.radius;
            let y = angle.to_radians().sin() * circle.radius;
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
    info!("Toolbar setup complete");
}

fn toolbar_visibility(
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

fn toolbar_follow(
    mut toolbars: Query<(&mut Transform, &Toolbar)>,
    follow: Query<&Transform, Without<Toolbar>>,
) {
    for (mut toolbar_transform, toolbar) in toolbars.iter_mut() {
        if let Some(follow_id) = toolbar.follow {
            if let Ok(follow_transform) = follow.get(follow_id) {
                toolbar_transform.translation = follow_transform.translation;
            }
        }
    }
}

fn toolbar_hover(toolbars: Query<(&Toolbar, &Visibility)>, follow: Query<&LinearVelocity>) {
    for (t, t_vis) in toolbars.iter() {
        if t_vis == &Visibility::Visible {
            if let Some(follow_id) = t.follow {
                if let Ok(follow_vel) = follow.get(follow_id) {}
            }
        }
    }
}

fn circle_radius_update(
    toolbars: Query<(Ref<CirclularDistributionProperties>, &Children), With<Toolbar>>,
    mut tools: Query<&mut Transform, With<ToolbarEntry>>,
) {
    for (circle, children) in toolbars.iter() {
        if circle.is_changed() {
            let count = children.iter().count();
            for (i, tool) in children.iter().enumerate() {
                let angle = 360.0 / (count as f32) * i as f32;
                let x = angle.to_radians().cos() * circle.radius;
                let y = angle.to_radians().sin() * circle.radius;
                if let Ok(mut tool_transform) = tools.get_mut(*tool) {
                    tool_transform.translation = Vec3::new(x, y, 200.0);
                }
            }
        }
    }
}
