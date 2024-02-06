pub mod inference_types;
pub mod inference_types_plugin;

pub mod prelude {
    pub use crate::inference_types::*;
    pub use crate::inference_types_plugin::InferenceTypesPlugin;
}
