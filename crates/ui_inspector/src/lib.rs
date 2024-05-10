#![feature(let_chains, trivial_bounds)]
mod ui_inspector_children_fetcher_plugin;
mod ui_inspector_egui_plugin;
mod ui_inspector_events_plugin;
mod ui_inspector_hover_indicator_click_plugin;
pub mod ui_inspector_plugin;
mod ui_inspector_preview_image_plugin;
mod ui_inspector_toggle_plugin;
mod ui_inspector_tree_update_plugin;
mod ui_inspector_worker_plugin;

pub mod prelude {
    pub use crate::ui_inspector_plugin::*;
}
