use bevy::prelude::*;
use cursor_hero_floaty_nametag_types::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy::transform::TransformSystem;

pub struct FloatyNametagPlugin;

impl Plugin for FloatyNametagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_nametags);
        app.add_systems(PostUpdate, update_nametags
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
        );
    }
}

fn spawn_nametags(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform, &FloatyName), Added<FloatyName>>,
    asset_server: Res<AssetServer>,
) {
    for entry in query.iter() {
        let (owner, transform, name) = entry;
        debug!("Spawning nametag for {:?}", name.text.clone());
        let ratio = 2.0;
        commands.spawn((
            Name::new(format!("FloatyNametag: {}", name.text.clone())),
            FloatyNametag { owner },
            Text2dBundle {
            text: Text::from_section(
                name.text.clone(),
                TextStyle {
                    font: asset_server.load(name.appearance.get_font_path()),
                    font_size: 18.0 * ratio,
                    color: name.appearance.get_text_color(),
                },
            ).with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(transform.translation() + Vec3::new(0.0, name.vertical_offset, 0.0)).with_scale(Vec3::new(1.0/ratio, 1.0/ratio, 1.0)),
            ..default()
        }));
    }   
}

fn update_nametags(
    mut commands: Commands,
    owner_query: Query<(Ref<GlobalTransform>, Ref<FloatyName>), Without<FloatyNametag>>,
    mut floaty_query: Query<(Entity, &mut Text, &mut Transform, &FloatyNametag), Without<FloatyName>>,
) {
    for floaty in floaty_query.iter_mut() {
        let (entity, mut text, mut transform, nametag) = floaty;
        let Ok(owner) = owner_query.get(nametag.owner) else {
            debug!("Owner of nametag {:?} not found, despawning", nametag.owner);
            commands.entity(entity).despawn_recursive();
            continue;
        };
        let (owner_transform, owner_floaty_name) = owner;
        if !owner_transform.is_changed() && !owner_floaty_name.is_changed() {
            continue;
        }
        text.sections[0].value = owner_floaty_name.text.clone();
        transform.translation = owner_transform.translation() + Vec3::new(0.0, owner_floaty_name.vertical_offset, 0.0);
    }
}
