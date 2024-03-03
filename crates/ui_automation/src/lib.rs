mod formatting;
mod gather_children;
mod taskbar;
mod ui_automation;
mod ui_automation_plugin;

pub mod prelude {
    pub use crate::formatting::*;
    pub use crate::taskbar::*;
    pub use crate::ui_automation::*;
    pub use crate::ui_automation_plugin::*;
    pub use cursor_hero_ui_automation_types::prelude::*;
}
