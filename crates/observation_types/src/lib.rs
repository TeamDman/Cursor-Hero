#![feature(trivial_bounds)]
pub mod observation_types;
pub mod observation_types_plugin;

pub mod prelude {
    pub use crate::observation_types::*;
    pub use crate::observation_types_plugin::ObservationTypesPlugin;
}
