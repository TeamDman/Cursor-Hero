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
            .add_event::<ToolbarHoverEvent>()
            .add_event::<ToolbarActivationEvent>()
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
                    (toolbar_visibility, toolbar_hover, tool_hover_visuals).chain(),
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

#[derive(Event, Debug, Reflect)]
pub enum ToolbarHoverEvent {
    HoverStart(Entity),
    HoverEnd(Entity),
}

#[derive(Event, Debug, Reflect)]
pub enum ToolbarActivationEvent {
    Activate(Entity),
    Deactivate(Entity),
}

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



fn normalize_angle(angle: f32) -> f32 {
    let two_pi = std::f32::consts::PI * 2.0;
    (angle + two_pi) % two_pi
}

fn angular_diff(angle1: f32, angle2: f32) -> f32 {
    let diff = (angle1 - angle2).abs();
    if diff > std::f32::consts::PI {
        std::f32::consts::PI * 2.0 - diff
    } else {
        diff
    }
}
fn toolbar_hover(
    mut commands: Commands,
    toolbars: Query<(&Toolbar, &Visibility, &Children)>,
    follow: Query<&LinearVelocity>,
    tools: Query<(&Transform, Option<&HoveredToolTag>), With<ToolbarEntry>>,
    mut events: EventWriter<ToolbarHoverEvent>,
) {
    for (t, t_vis, t_kids) in toolbars.iter() {
        if t_vis == &Visibility::Visible {
            if let Some(follow_id) = t.follow {
                if let Ok(follow_vel) = follow.get(follow_id) {
                    let mut closest = None;
                    if follow_vel.x != 0.0 || follow_vel.y != 0.0 {
                        // we want to find the toolbar entry that is closest to the direction of the movement of the follow entity
                        // find the angle between the follow entity and each toolbar entry
                        // find the angle of the direction of travel
                        // find the tool with the smallest difference between the two angles
                        let travel_angle = normalize_angle(follow_vel.0.angle_between(Vec2::new(1.0, 0.0)));
                        let mut closest_angle = std::f32::consts::PI; // Initialized to the max angle difference (180 degrees)
                        
                        for kid in t_kids.iter() {
                            if let Ok((kid_transform, _hovered_status)) = tools.get(*kid) {
                                let kid_angle = normalize_angle(kid_transform.translation.xy().angle_between(Vec2::new(1.0, 0.0)));
                                let diff = angular_diff(kid_angle, travel_angle);
                        
                                if diff < closest_angle {
                                    closest = Some(*kid);
                                    closest_angle = diff;
                                }
                            }
                        }
                    }
                    // remove the follow tag from the unhovered tools
                    for kid in t_kids.iter() {
                        if Some(*kid) != closest {
                            if let Ok((_, hovered_status)) = tools.get(*kid) {
                                if hovered_status.is_some() {
                                    commands.entity(*kid).remove::<HoveredToolTag>();
                                    events.send(ToolbarHoverEvent::HoverEnd(*kid));
                                }
                            }
                        }
                    }
                    if let Some(closest) = closest {
                        // add the follow tag to the closest tool
                        // if the closest tool already has the follow tag, do nothing
                        if let Ok((_, hovered_status)) = tools.get(closest) {
                            if hovered_status.is_none() {
                                commands.entity(closest).insert(HoveredToolTag);
                                events.send(ToolbarHoverEvent::HoverStart(closest));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn tool_hover_visuals(
    mut query: Query<&mut Sprite, With<ToolbarEntry>>,
    mut events: EventReader<ToolbarHoverEvent>,
) {
    for event in events.read() {
        match event {
            ToolbarHoverEvent::HoverStart(entity) => {
                debug!("Hovering over tool: {:?}", entity);
                if let Ok(mut sprite) = query.get_mut(*entity) {
                    debug!("Setting color to purple");
                    sprite.color = Color::rgb(0.5, 0.0, 0.5);
                }
            }
            ToolbarHoverEvent::HoverEnd(entity) => {
                debug!("No longer hovering over tool: {:?}", entity);
                if let Ok(mut sprite) = query.get_mut(*entity) {
                    debug!("Setting color to white");
                    sprite.color = Color::WHITE;
                }
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
