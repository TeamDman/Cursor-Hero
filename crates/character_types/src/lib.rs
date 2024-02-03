pub mod character_types_plugin;
pub mod character_types;

pub mod prelude {
    pub use crate::character_types_plugin::CharacterTypesPlugin;
    pub use crate::character_types::*;
}