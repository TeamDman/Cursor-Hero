use bevy::prelude::*;

fn main() {
    App::new().add_plugins((CharacterPlugin,HatPlugin)).run();
}

struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        println!("Building CharacterPlugin");
        app.configure_sets(Startup, CharacterSystemSet::Spawn)
            .add_systems(Startup, (spawn_character.in_set(CharacterSystemSet::Spawn), apply_deferred).chain());
    }
}

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum CharacterSystemSet {
    Spawn,
}

#[derive(Component, Default, Reflect)]
struct Character;

fn spawn_character(mut commands: Commands) {
    let id = commands.spawn((SpatialBundle::default(), Character)).id();
    println!("Spawned character with id: {:?}", id)
}

struct HatPlugin;
impl Plugin for HatPlugin {
    fn build(&self, app: &mut App) {
        println!("Building HatPlugin");
        app.configure_sets(
            Startup,
            HatSystemSet::Spawn.after(CharacterSystemSet::Spawn),
        )
        .add_systems(
            Startup,
            (apply_deferred, spawn_hat.in_set(HatSystemSet::Spawn)).chain(),
        );
    }
}

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum HatSystemSet {
    Spawn,
}

#[derive(Component, Default, Reflect)]
struct Hat;

fn spawn_hat(mut commands: Commands, all: Query<Entity>, characters: Query<Entity, With<Character>>) {
    println!("Found {:?} entities", all.iter().count());
    for entity in all.iter() {
        println!("Entity: {:?}", entity);
    }
    let found = characters.single();
    println!("Found character: {:?}", found);
    commands.entity(found).with_children(|parent| {
        parent.spawn(Hat);
    });
}
