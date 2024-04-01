use crate::prelude::DrillId;
use anyhow::Context;
use anyhow::Result;
use std::collections::VecDeque;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

#[derive(Debug)]
pub enum DrillError {
    UI(uiautomation::Error),
    EmptyPath,
    BadPath,
    OutOfBounds {
        given: u32,
        max: u32,
        error: uiautomation::Error,
    },
}
impl std::error::Error for DrillError {}
impl std::fmt::Display for DrillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrillError::UI(e) => write!(f, "UIAutomation error: {}", e),
            DrillError::EmptyPath => write!(f, "Empty path"),
            DrillError::BadPath => write!(f, "Bad path"),
            DrillError::OutOfBounds { given, max, error } => write!(
                f,
                "Out of bounds: given {}, max {}, error: {}",
                given, max, error
            ),
        }
    }
}
impl From<uiautomation::Error> for DrillError {
    fn from(e: uiautomation::Error) -> Self {
        DrillError::UI(e)
    }
}
pub trait Drillable {
    fn drill<T: Into<DrillId>>(&self, walker: &UITreeWalker, path: T) -> Result<UIElement>;
}
impl Drillable for UIElement {
    fn drill<T: Into<DrillId>>(&self, walker: &UITreeWalker, path: T) -> Result<UIElement> {
        let drill_id: DrillId = path.into();
        match drill_id {
            DrillId::Child(path) => {
                let mut path = path
                    .into_iter()
                    .map(|x| x as u32)
                    .collect::<VecDeque<u32>>();
                if path.iter().any(|x| (*x as i32) < 0) {
                    return Err(DrillError::BadPath.into());
                }
                drill_inner(self, walker, &mut path)
            }
            DrillId::Root | DrillId::Unknown => Err(DrillError::BadPath.into()),
        }
    }
}
fn drill_inner(
    start: &UIElement,
    walker: &UITreeWalker,
    path: &mut VecDeque<u32>,
) -> Result<UIElement> {
    let target_index = match path.pop_front() {
        Some(x) => x,
        None => return Err(DrillError::EmptyPath.into()),
    };
    let mut child = walker
        .get_first_child(start)
        .context("get first child of start")?;
    let mut i = 0;
    while i < target_index {
        i += 1;
        child = match walker.get_next_sibling(&child) {
            Ok(x) => x,
            Err(e) => {
                return Err(DrillError::OutOfBounds {
                    given: i,
                    max: target_index,
                    error: e,
                }
                .into())
            }
        };
    }
    if path.is_empty() {
        Ok(child)
    } else {
        drill_inner(&child, walker, path)
    }
}
