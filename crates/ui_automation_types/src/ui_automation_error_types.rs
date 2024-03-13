use crate::prelude::VSCodeResolveError;
use crate::ui_automation_drill::DrillError;
use std::fmt;

#[derive(Debug)]
pub enum AppResolveError {
    UI(uiautomation::Error),
    BadStructure(String),
    NoMatch,
}
impl From<uiautomation::Error> for AppResolveError {
    fn from(e: uiautomation::Error) -> Self {
        AppResolveError::UI(e)
    }
}
impl From<DrillError> for AppResolveError {
    fn from(e: DrillError) -> Self {
        match e {
            DrillError::UI(e) => AppResolveError::UI(e),
            DrillError::EmptyPath => AppResolveError::BadStructure("Empty path".to_string()),
            DrillError::BadPath => AppResolveError::BadStructure("Bad path".to_string()),
            DrillError::OutOfBounds {
                given,
                max,
                error: e,
            } => AppResolveError::BadStructure(format!(
                "Out of bounds: given: {}, max: {}, error: {}",
                given, max, e
            )),
        }
    }
}
impl From<VSCodeResolveError> for AppResolveError {
    fn from(e: VSCodeResolveError) -> Self {
        match e {
            VSCodeResolveError::UnknownSideTabKind(s) => {
                AppResolveError::BadStructure(format!("Unknown VSCode side tab kind: {}", s))
            }
            VSCodeResolveError::UnknownState => {
                AppResolveError::BadStructure("Unknown VSCode state".to_string())
            }
            VSCodeResolveError::UI(e) => AppResolveError::UI(e),
            VSCodeResolveError::BadChildCount { tried_accessing } => {
                AppResolveError::BadStructure(format!(
                    "Bad child count: tried accessing: {}",
                    tried_accessing
                ))
            }
        }
    }
}

impl fmt::Display for AppResolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a descriptive message for the error.
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for AppResolveError {}

#[derive(Debug)]
pub enum GatherAppsError {
    UI(uiautomation::Error),
    NoneMatch,
    ResolveFailed(Vec<AppResolveError>),
}
impl From<uiautomation::Error> for GatherAppsError {
    fn from(e: uiautomation::Error) -> Self {
        GatherAppsError::UI(e)
    }
}

impl fmt::Display for GatherAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write a descriptive message for the error.
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for GatherAppsError {}
