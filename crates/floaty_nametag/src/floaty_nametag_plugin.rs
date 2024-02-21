use bevy::prelude::*;
use cursor_hero_floaty_nametag_types::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy::transform::TransformSystem;

pub struct FloatyNametagPlugin;

impl Plugin for FloatyNametagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_nametags);
        app.add_systems(PostUpdate, update_nametag_positions
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
        );
    }
}

fn spawn_nametags(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &FloatyName), Added<FloatyName>>,
    asset_server: Res<AssetServer>,
) {
    for entry in query.iter() {
        let (owner, transform, name) = entry;
        debug!("Spawning nametag for {:?}", name.text.clone());
        commands.spawn((
            Name::new(format!("FloatyNametag: {}", name.text.clone())),
            FloatyNametag { owner },
            Text2dBundle {
            text: Text::from_section(
                name.text.clone(),
                TextStyle {
                    font: asset_server.load(name.appearance.get_font_path()),
                    font_size: 36.0,
                    color: name.appearance.get_text_color(),
                },
            ).with_alignment(TextAlignment::Center),
            transform: *transform,
            ..default()
        }));
    }   
}

fn update_nametag_positions(
    owner_query: Query<(&GlobalTransform, &FloatyName), (Or<(Changed<Transform>, Changed<FloatyName>)>, Without<FloatyNametag>)>,
    mut floaty_query: Query<(&mut Text, &mut Transform, &FloatyNametag), Without<FloatyName>>,
) {
    for floaty in floaty_query.iter_mut() {
        let (mut text, mut transform, nametag) = floaty;
        // debug!("Updating nametag for {:?}", nametag.owner);
        let Ok(owner) = owner_query.get(nametag.owner) else {
            continue;
        };
        let (owner_transform, owner_floaty_name) = owner;
        text.sections[0].value = owner_floaty_name.text.clone();
        transform.translation = owner_transform.translation() + Vec3::new(0.0, owner_floaty_name.vertical_offset, 0.0);
    }
}
