use uiautomation::UIAutomation;
use uiautomation::UIElement;

pub fn get_tree_string(element: &UIElement) -> Result<String, uiautomation::Error> {
    let automation = UIAutomation::new()?;
    format_tree_recursive(element, &automation, 0)
}
fn format_tree_recursive(
    element: &UIElement,
    automation: &UIAutomation,
    depth: usize,
) -> Result<String, uiautomation::Error> {
    // Format the current element's label.
    let mut result = format!(
        "{}{}\n",
        " ".repeat(depth * 2), // Increase indentation with depth.
        format_tree_label(element)
    );

    // Use the TreeWalker to navigate the children.
    let walker = automation.create_tree_walker()?;
    if let Ok(child) = walker.get_first_child(element) {
        // Recursively format the child and any siblings.
        result.push_str(&format_tree_recursive(&child, automation, depth + 1)?);
        let mut next_sibling = child;
        while let Ok(sibling) = walker.get_next_sibling(&next_sibling) {
            result.push_str(&format_tree_recursive(&sibling, automation, depth + 1)?);
            next_sibling = sibling;
        }
    }
    Ok(result)
}
fn format_tree_label(element: &UIElement) -> String {
    format!(
        "name={} control_type={} class_name={} runtime_id={} rect={}",
        element
            .get_name()
            .map(|name| format!("{:?}", name))
            .unwrap_or_else(|_| "(null)".to_string()),
        element
            .get_control_type()
            .map(|ct| format!("{:?}", ct))
            .unwrap_or_else(|_| "unknown control type".to_string()),
        element
            .get_classname()
            .map(|name| format!("{:?}", name))
            .unwrap_or_else(|_| "(null)".to_string()),
        element
            .get_runtime_id()
            .map(|id| format!("{:?}", id))
            .unwrap_or_else(|_| "(null)".to_string()),
        element
            .get_bounding_rectangle()
            .map(|rect| format!("{:?}", rect))
            .unwrap_or_else(|_| "(null)".to_string()),
    )
}
