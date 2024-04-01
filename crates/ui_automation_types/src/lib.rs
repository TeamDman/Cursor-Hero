#![feature(trivial_bounds)]

mod calculator_ui_types;
mod taskbar_ui_types;
mod ui_automation_drill;
mod ui_automation_error_types;
mod ui_automation_types;
mod ui_automation_types_plugin;
mod vscode_ui_types;

pub mod prelude {
    pub use crate::calculator_ui_types::*;
    pub use crate::taskbar_ui_types::*;
    pub use crate::ui_automation_drill::*;
    pub use crate::ui_automation_error_types::*;
    pub use crate::ui_automation_types::*;
    pub use crate::ui_automation_types_plugin::*;
    pub use crate::vscode_ui_types::*;
    // pub use uiautomation;
}
