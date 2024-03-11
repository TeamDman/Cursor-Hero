use std::collections::VecDeque;

use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::gather_children;
use crate::gather_children::StopBehaviour;

pub fn gather_root_children(
    automation: &UIAutomation,
    walker: &UITreeWalker,
) -> Result<VecDeque<UIElement>, uiautomation::Error> {
    let root = automation.get_root_element()?;
    // println!("Boutta gather top level children");
    let top_level_children = gather_children(walker, &root, &StopBehaviour::RootEndEncountered);
    // let condition = &automation.create_true_condition()?;
    // let found = root.find_all(TreeScope::Children, condition)?;
    // println!("Found {} top level children", top_level_children.len());
    Ok(top_level_children)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uiautomation::UIAutomation;

    #[test]
    fn test_gather_root_children() {
        let automation = UIAutomation::new().unwrap();
        let walker = automation.create_tree_walker().unwrap();
        for _ in 0..100 {
            let start = std::time::Instant::now();
            let children = gather_root_children(&automation, &walker).unwrap();
            let end = std::time::Instant::now();
            let elapsed = end - start;
            println!("Gathered {} children in {:?}", children.len(), elapsed);
            assert!(!children.is_empty());
            assert!(elapsed.as_millis() < 1000);
        }
    }
}
