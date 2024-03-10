mod formatting;
mod gather_children;
mod taskbar;
mod ui_automation_plugin;
mod elements_at_point;
mod gather_element_info;
mod resolve_app;
mod take_screenshot;
mod tests;

pub mod prelude {
    pub use cursor_hero_ui_automation_types::prelude::*;
    pub use crate::formatting::*;
    pub use crate::taskbar::*;
    pub use crate::ui_automation_plugin::*;
    pub use crate::elements_at_point::*;
    pub use crate::gather_element_info::*;
    pub use crate::take_screenshot::*;
}
