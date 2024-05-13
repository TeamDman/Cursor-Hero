#![feature(let_chains, trivial_bounds, if_let_guard)]
mod ui_inspector_children_fetcher_plugin;
mod ui_inspector_egui_plugin;
mod ui_inspector_egui_properties_panel;
mod ui_inspector_egui_tree_panel;
mod ui_inspector_hover_indicator_click_plugin;
pub mod ui_inspector_plugin;
mod ui_inspector_preview_image_plugin;
mod ui_inspector_scratch_pad_events_plugin;
mod ui_inspector_toggle_plugin;
mod ui_inspector_tree_update_plugin;
mod ui_inspector_worker_plugin;

pub mod prelude {
    pub use crate::ui_inspector_plugin::*;
}
