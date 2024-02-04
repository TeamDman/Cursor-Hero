pub mod inference_types_plugin;
pub mod inference_types;

pub mod prelude {
    pub use crate::inference_types_plugin::InferenceTypesPlugin;
    pub use crate::inference_types::*;
}