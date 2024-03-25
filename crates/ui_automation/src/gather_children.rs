use std::collections::VecDeque;

use cursor_hero_metrics::Metrics;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

#[allow(dead_code)]
#[derive(Debug)]
pub enum StopBehaviour {
    EndOfSiblings,
    LastChildEncountered,
    TaskbarEndEncountered,
    RootEndEncountered, // Calling get_next_sibling on the last child of root will hang, so use this to mitigate
}
impl StopBehaviour {
    fn include_last_child(&self) -> bool {
        !matches!(self, StopBehaviour::TaskbarEndEncountered)
    }
}
trait GatherChildrenStopBehaviourFn {
    fn should_stop(&self, next: &UIElement) -> bool;
}

#[derive(Debug)]
struct EndOfSiblings;
impl GatherChildrenStopBehaviourFn for EndOfSiblings {
    fn should_stop(&self, _element: &UIElement) -> bool {
        false
    }
}

#[derive(Debug)]
struct LastChildEncountered {
    runtime_id_of_last: Vec<i32>,
}
impl GatherChildrenStopBehaviourFn for LastChildEncountered {
    fn should_stop(&self, element: &UIElement) -> bool {
        element.get_runtime_id() == Ok(self.runtime_id_of_last.clone())
    }
}

#[derive(Debug)]
struct TaskbarEndEncountered;
impl GatherChildrenStopBehaviourFn for TaskbarEndEncountered {
    fn should_stop(&self, element: &UIElement) -> bool {
        element.get_automation_id() == Ok("TaskbarEndAccessibilityElement".to_string())
    }
}

#[derive(Debug)]
struct RootEndEncountered;
impl GatherChildrenStopBehaviourFn for RootEndEncountered {
    fn should_stop(&self, element: &UIElement) -> bool {
        element.get_name() == Ok("Program Manager".to_string())
            && element.get_classname() == Ok("Progman".to_string())
        // This could be more specific, but until a false positive is encountered, this is fine
    }
}

pub trait GatherChildrenable {
    fn gather_children(
        &self,
        walker: &UITreeWalker,
        stop_behaviour: &StopBehaviour,
    ) -> VecDeque<UIElement>;
}
impl GatherChildrenable for UIElement {
    fn gather_children(
        &self,
        walker: &UITreeWalker,
        stop_behaviour: &StopBehaviour,
    ) -> VecDeque<UIElement> {
        gather_children(walker, self, stop_behaviour)
    }
}

pub fn gather_children(
    walker: &UITreeWalker,
    parent: &UIElement,
    stop_behaviour: &StopBehaviour,
) -> VecDeque<UIElement> {
    // println!("Gathering children of {:?}", parent);
    let mut children = VecDeque::new();
    let mut metrics = Metrics::default();

    // println!("Constructing stop behaviour fn for {:?}", stop_behaviour);
    metrics.begin("construct stop behaviour");
    let stop: Box<dyn GatherChildrenStopBehaviourFn> = match stop_behaviour {
        StopBehaviour::EndOfSiblings => Box::new(EndOfSiblings),
        StopBehaviour::LastChildEncountered => {
            // println!("Getting last child of {:?}", parent);
            let last = walker.get_last_child(parent);
            let last = match last {
                Ok(last) => last,
                Err(_) => {
                    eprintln!("Failed to get last child of {:?}", parent);
                    return children
                },
            };
            let runtime_id_of_last = last.get_runtime_id();
            let runtime_id_of_last = match runtime_id_of_last {
                Ok(runtime_id_of_last) => runtime_id_of_last,
                Err(_) => {
                    eprintln!("Failed to get runtime id of last child {:?} of {:?}", last, parent);
                    return children
                },
            };
            Box::new(LastChildEncountered { runtime_id_of_last })
        }
        StopBehaviour::TaskbarEndEncountered => Box::new(TaskbarEndEncountered),
        StopBehaviour::RootEndEncountered => Box::new(RootEndEncountered),
    };
    metrics.end("construct stop behaviour");
    // println!("Constructed stop behaviour {:?}", stop_behaviour);

    // println!("Finding first child");
    metrics.begin("find first child");
    let first = walker.get_first_child(parent);
    metrics.end("find first child");
    // println!("Found first child");

    let Ok(first) = first else {
        return children;
    };
    children.push_back(first.clone());
    let mut next = first;
    let mut i = 0;
    loop {
        // println!("About to grab next sibling of {:?}", next);
        metrics.begin(format!("get next sibling {}", i).as_str());
        let sibling = walker.get_next_sibling(&next);
        metrics.end(format!("get next sibling {}", i).as_str());
        i += 1;

        if let Ok(sibling) = sibling {
            // println!("Got sibling {:?}", sibling);
            // println!("Checking if should stop");
            if stop.should_stop(&sibling) {
                // println!("Should stop");
                if stop_behaviour.include_last_child() {
                    // println!("Including last child");
                    children.push_back(sibling.clone());
                }
                break;
            } else {
                // println!("Should not stop");
                children.push_back(sibling.clone());
                next = sibling;
            }
        } else {
            break;
        }
    }
    // println!("Gathered {} children", children.len());
    // println!("| {}", metrics.report().split(" | ").join("\n| "));
    children
}
