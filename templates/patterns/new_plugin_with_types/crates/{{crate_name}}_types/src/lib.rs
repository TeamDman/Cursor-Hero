mod {{crate_name}}_types_plugin;
mod {{crate_name}}_types;

pub mod prelude {
    pub use crate::{{crate_name}}_types::*;
    pub use crate::{{crate_name}}_types_plugin::*;
}