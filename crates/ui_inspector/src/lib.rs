#![feature(let_chains, trivial_bounds)]
pub mod ui_inspector_plugin;

pub mod prelude {
    pub use crate::ui_inspector_plugin::*;
}