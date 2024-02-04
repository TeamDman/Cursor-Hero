pub mod observation_types_plugin;
pub mod observation_types;

pub mod prelude {
    pub use crate::observation_types_plugin::ObservationTypesPlugin;
    pub use crate::observation_types::*;
}