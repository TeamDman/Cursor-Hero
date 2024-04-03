use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct CursorMirror;

#[derive(Resource, Deref, Default, Reflect)]
#[reflect(Resource)]
pub struct HostCursorPosition(pub IVec2);