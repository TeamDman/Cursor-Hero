use uiautomation::UIElement;
use uiautomation::UITreeWalker;

#[allow(dead_code)]
pub enum GatherChildrenStopBehaviour {
    EndOfSiblings,
    LastChildEncountered,
    TaskbarEndEncountered,
}
impl GatherChildrenStopBehaviour {
    fn include_last_child(&self) -> bool {
        match self {
            GatherChildrenStopBehaviour::TaskbarEndEncountered => false,
            _ => true,
        }
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
pub fn gather_children(
    walker: &UITreeWalker,
    parent: &UIElement,
    stop_behaviour: GatherChildrenStopBehaviour,
) -> Vec<UIElement> {
    let stop: Box<dyn GatherChildrenStopBehaviourFn> = match stop_behaviour {
        GatherChildrenStopBehaviour::EndOfSiblings => Box::new(EndOfSiblings),
        GatherChildrenStopBehaviour::LastChildEncountered => {
            let last = walker.get_last_child(parent).unwrap(); // Handle error appropriately
            let runtime_id_of_last = last.get_runtime_id().unwrap(); // Handle error appropriately
            Box::new(LastChildEncountered { runtime_id_of_last })
        }
        GatherChildrenStopBehaviour::TaskbarEndEncountered => Box::new(TaskbarEndEncountered),
    };
    let mut children = vec![];
    if let Ok(first) = walker.get_first_child(&parent) {
        children.push(first.clone());
        let mut next = first;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            if stop.should_stop(&sibling) {
                if stop_behaviour.include_last_child() {
                    children.push(sibling.clone());
                }
                break;
            } else {
                children.push(sibling.clone());
                next = sibling;
            }
        }
    }
    children
}
