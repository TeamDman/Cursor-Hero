use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct AfterimageParent;

#[derive(Component, Debug, Reflect)]
pub struct Afterimage {
    pub life_remaining: usize,
}

pub struct AfterimagePlugin;
impl Plugin for AfterimagePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AfterimageParent>()
            .register_type::<Afterimage>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    tick_afterimages,
                ),
            );
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        AfterimageParent,
        Name::new("Afterimage Parent"),
    ));
}
fn tick_afterimages(mut commands: Commands, mut afterimages: Query<(Entity, &mut Afterimage)>) {
    for (entity, mut afterimage) in &mut afterimages.iter_mut() {
        afterimage.life_remaining -= 1;
        if afterimage.life_remaining == 0 {
            commands.entity(entity).despawn();
        }
    }
}
