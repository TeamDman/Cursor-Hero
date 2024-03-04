use crate::tool_spawning::ToolSpawnConfig;
use bevy::prelude::*;
use bevy_xpbd_2d::math::*;
use bevy_xpbd_2d::prelude::*;
use cursor_hero_character_types::prelude::*;
use cursor_hero_math::prelude::Lerp;
use itertools::Itertools;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

pub struct MovementToolPlugin;

impl Plugin for MovementToolPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

