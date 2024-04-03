use bevy::prelude::*;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Debug, Reflect, Clone, Copy)]
pub enum CursorMovementBehaviour {
    None,
    SetHostCursorFromCursorWorldCoords,
    SetHostCursorFromWindowCoords,
    SetCursorFromHostCursorWindowCoords,
}

impl Display for CursorMovementBehaviour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CursorMovementBehaviour::None => "None",
                CursorMovementBehaviour::SetHostCursorFromCursorWorldCoords =>
                    "SetHostCursorFromCursorWorldCoords",
                CursorMovementBehaviour::SetHostCursorFromWindowCoords =>
                    "SetHostCursorFromWindowCoords",
                CursorMovementBehaviour::SetCursorFromHostCursorWindowCoords =>
                    "SetCursorFromHostCursorWindowCoords",
            }
        )
    }
}
