mod worker_types;
mod worker_types_plugin;

pub mod prelude {
    pub use crate::worker_types::*;
    pub use crate::worker_types_plugin::*;
    pub use anyhow;
    pub use crossbeam_channel;
}
