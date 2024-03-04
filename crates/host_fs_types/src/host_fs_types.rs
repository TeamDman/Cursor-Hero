use std::path::PathBuf;

use bevy::prelude::*;

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct HostPath {
    pub path: PathBuf,
}

#[derive(Event, Debug, Reflect)]
pub enum HostPathAction {
    OpenWithCode { path: HostPath },
}
