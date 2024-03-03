pub mod ui_automation_plugin;
mod gather_children;
pub mod formatting;
pub mod taskbar;
pub mod ui_automation;

pub mod prelude {
    pub use crate::ui_automation_plugin::*;
    pub use crate::formatting::*;
    pub use crate::taskbar::*;
    pub use crate::ui_automation::*;
    pub use cursor_hero_ui_automation_types::prelude::*;
}