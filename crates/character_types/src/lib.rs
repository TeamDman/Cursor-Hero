pub mod character_types;
pub mod character_types_plugin;

pub mod prelude {
    pub use crate::character_types::*;
    pub use crate::character_types_plugin::CharacterTypesPlugin;
}
