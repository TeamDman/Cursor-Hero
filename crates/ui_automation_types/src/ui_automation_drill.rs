use std::collections::VecDeque;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

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
impl From<uiautomation::Error> for DrillError {
    fn from(e: uiautomation::Error) -> Self {
        DrillError::UI(e)
    }
}
pub trait Drillable {
    fn drill(&self, walker: &UITreeWalker, path: Vec<i32>) -> Result<UIElement, DrillError>;
}
impl Drillable for UIElement {
    fn drill(&self, walker: &UITreeWalker, path: Vec<i32>) -> Result<UIElement, DrillError> {
        let mut path = path
            .into_iter()
            .map(|x| x as u32)
            .collect::<VecDeque<u32>>();
        if path.iter().any(|x| (*x as i32) < 0) {
            return Err(DrillError::BadPath);
        }
        drill_inner(self, walker, &mut path)
    }
}
fn drill_inner(
    start: &UIElement,
    walker: &UITreeWalker,
    path: &mut VecDeque<u32>,
) -> Result<UIElement, DrillError> {
    let target_index = match path.pop_front() {
        Some(x) => x,
        None => return Err(DrillError::EmptyPath),
    };
    let mut child = walker.get_first_child(start)?;
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
                })
            }
        };
    }
    if path.is_empty() {
        Ok(child)
    } else {
        drill_inner(&child, walker, path)
    }
}
