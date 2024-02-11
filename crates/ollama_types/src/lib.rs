pub mod ollama_types_plugin;
pub mod ollama_types;

pub mod prelude {
    pub use crate::ollama_types::*;
    pub use crate::ollama_types_plugin::*;
}