use bevy::math::IVec2;
use std::collections::VecDeque;
use uiautomation::types::Point;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

pub fn find_element_at(pos: IVec2) -> Result<UIElement, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    automation.element_from_point(Point::new(pos.x, pos.y))
}

pub fn gather_elements_at(pos: IVec2) -> Result<Vec<(UIElement, usize)>, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    let start = automation.element_from_point(Point::new(pos.x, pos.y))?;
    let mut rtn = vec![];
    let mut next = VecDeque::new();
    next.push_back((start, 0));
    while let Some((elem, depth)) = next.pop_front() {
        rtn.push((elem.clone(), depth));
        if let Ok(child) = walker.get_first_child(&elem) {
            next.push_back((child.clone(), depth + 1));
            let mut next_sibling = child;
            while let Ok(sibling) = walker.get_next_sibling(&next_sibling) {
                next.push_back((sibling.clone(), depth + 1));
                next_sibling = sibling;
            }
        }
    }
    Ok(rtn)
}
