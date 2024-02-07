use bevy::prelude::*;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Debug, Reflect, Clone, Copy)]
pub enum PointerMovementBehaviour {
    None,
    HostFollowsPointer,
    HostOverWindow,
    PointerFollowsHost,
}

impl Display for PointerMovementBehaviour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PointerMovementBehaviour::None => "None",
                PointerMovementBehaviour::HostFollowsPointer => "HostFollowsPointer",
                PointerMovementBehaviour::HostOverWindow => "HostOverWindow",
                PointerMovementBehaviour::PointerFollowsHost => "PointerFollowsHost",
            }
        )
    }
}
