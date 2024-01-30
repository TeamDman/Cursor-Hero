use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

pub struct SprintToolTypesPlugin;

impl Plugin for SprintToolTypesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SprintData>();
    }
}

#[derive(Component, InspectorOptions, Reflect, Default, Debug)]
#[reflect(Component, InspectorOptions)]
pub struct SprintData {
    pub value: f32,
    pub default_value: f32,
    pub sprint_value: f32,
    pub sprint_enabled: bool,
}