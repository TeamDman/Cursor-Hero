#![feature(trivial_bounds)]

pub mod ui_automation_types;
pub mod ui_automation_types_plugin;

pub mod prelude {
    pub use crate::ui_automation_types::*;
    pub use crate::ui_automation_types_plugin::*;
}
