use bevy::prelude::*;

use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;


#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub enum PointerSystemSet {
    Position,
}


#[derive(Component, InspectorOptions, Debug, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Pointer {
    #[inspector(min = 0.0)]
    pub reach: f32,
    #[inspector(min = 0.0)]
    pub default_reach: f32,
    #[inspector(min = 0.0)]
    pub sprint_reach: f32,
}
impl Default for Pointer {
    fn default() -> Self {
        Self {
            reach: 50.0,
            default_reach: 50.0,
            sprint_reach: 2000.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct FollowHostCursor;
#[derive(Component, Debug, Reflect)]
pub struct HostCursorFollows;