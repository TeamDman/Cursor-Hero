#![feature(trivial_bounds)]
pub mod voice_to_text_types_plugin;
pub mod voice_to_text_types;

pub mod prelude {
    pub use crate::voice_to_text_types::*;
    pub use crate::voice_to_text_types_plugin::*;
}