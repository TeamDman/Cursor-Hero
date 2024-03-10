use cursor_hero_ui_automation_types::ui_automation_types::AppResolveError;
use cursor_hero_ui_automation_types::ui_automation_types::GatherAppsError;
use cursor_hero_ui_automation_types::ui_automation_types::UISnapshot;
use itertools::Itertools;
use uiautomation::UIAutomation;

use crate::gather_children::gather_children;
use crate::gather_children::StopBehaviour;
use crate::resolve_app::resolve_app;


pub fn take_snapshot() -> Result<UISnapshot, GatherAppsError> {
    let automation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    println!("Boutta gather top level children");
    // let walker = automation.create_tree_walker()?;
    let walker = automation.get_raw_view_walker()?;
    // let top_level_children = gather_children(&walker, &root, &StopBehaviour::EndOfSiblings);
    // ^^^ lags getting next sibling of Program Manager pane (last child of root)
    let top_level_children = gather_children(&walker, &root, &StopBehaviour::RootEndEncountered);
    // let condition = &automation.create_true_condition()?;
    // let found = root.find_all(TreeScope::Children, condition)?;
    println!("Found {} top level children", top_level_children.len());

    let walker = automation.create_tree_walker()?;
    let focused = automation.get_focused_element()?;
    let focused_app = walker.normalize(&focused)?;

    let mut apps = vec![];
    let mut errors = vec![];
    for elem in top_level_children {
        let focused = elem.get_runtime_id() == focused_app.get_runtime_id();
        match resolve_app(&elem, &automation, focused) {
            Ok(app) => {
                apps.push((elem, app));
            }
            Err(e) => errors.push(e),
        }
    }
    let bad_errors = errors
        .into_iter()
        .filter(|e| !matches!(e, AppResolveError::NoMatch))
        .collect_vec();
    if !bad_errors.is_empty() {
        return Err(GatherAppsError::ResolveFailed(bad_errors));
    }

    let snapshot = UISnapshot {
        app_windows: apps.into_iter().map(|(_elem, app)| app).collect(),
    };
    Ok(snapshot)
}
