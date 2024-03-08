use std::collections::VecDeque;

use uiautomation::UIElement;
use uiautomation::UITreeWalker;

#[allow(dead_code)]
pub enum StopBehaviour {
    EndOfSiblings,
    LastChildEncountered,
    TaskbarEndEncountered,
}
impl StopBehaviour {
    fn include_last_child(&self) -> bool {
        !matches!(self, StopBehaviour::TaskbarEndEncountered)
    }
}
trait GatherChildrenStopBehaviourFn {
    fn should_stop(&self, next: &UIElement) -> bool;
}
struct EndOfSiblings;
struct LastChildEncountered {
    runtime_id_of_last: Vec<i32>,
}
struct TaskbarEndEncountered;
impl GatherChildrenStopBehaviourFn for EndOfSiblings {
    fn should_stop(&self, _element: &UIElement) -> bool {
        false
    }
}
impl GatherChildrenStopBehaviourFn for LastChildEncountered {
    fn should_stop(&self, element: &UIElement) -> bool {
        element.get_runtime_id() == Ok(self.runtime_id_of_last.clone())
    }
}
impl GatherChildrenStopBehaviourFn for TaskbarEndEncountered {
    fn should_stop(&self, element: &UIElement) -> bool {
        element.get_automation_id() == Ok("TaskbarEndAccessibilityElement".to_string())
    }
}
pub trait GatherChildrenable {
    fn gather_children(&self, walker: &UITreeWalker, stop_behaviour: &StopBehaviour) -> VecDeque<UIElement>;
}
impl GatherChildrenable for UIElement {
    fn gather_children(&self, walker: &UITreeWalker, stop_behaviour: &StopBehaviour) -> VecDeque<UIElement> {
        gather_children(walker, self, stop_behaviour)
    }
}
pub fn gather_children(
    walker: &UITreeWalker,
    parent: &UIElement,
    stop_behaviour: &StopBehaviour,
) -> VecDeque<UIElement> {
    let stop: Box<dyn GatherChildrenStopBehaviourFn> = match stop_behaviour {
        StopBehaviour::EndOfSiblings => Box::new(EndOfSiblings),
        StopBehaviour::LastChildEncountered => {
            let last = walker.get_last_child(parent).unwrap(); // Handle error appropriately
            let runtime_id_of_last = last.get_runtime_id().unwrap(); // Handle error appropriately
            Box::new(LastChildEncountered { runtime_id_of_last })
        }
        StopBehaviour::TaskbarEndEncountered => Box::new(TaskbarEndEncountered),
    };
    let mut children = VecDeque::new();
    if let Ok(first) = walker.get_first_child(parent) {
        children.push_back(first.clone());
        let mut next = first;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            if stop.should_stop(&sibling) {
                if stop_behaviour.include_last_child() {
                    children.push_back(sibling.clone());
                }
                break;
            } else {
                children.push_back(sibling.clone());
                next = sibling;
            }
        }
    }
    children
}
