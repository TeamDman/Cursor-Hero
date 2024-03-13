use bevy::math::IVec2;
use cursor_hero_ui_automation_types::prelude::*;
use itertools::Itertools;
use uiautomation::controls::ControlType;
use uiautomation::patterns::UIExpandCollapsePattern;
use uiautomation::types::ExpandCollapseState;
use uiautomation::types::TreeScope;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::gather_children;
use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;

pub(crate) fn resolve_vscode(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppWindow, AppResolveError> {
    let walker = automation.create_tree_walker()?;
    let root = elem;

    let temp = root.drill(&walker, vec![0, 0, 0, 0, 0, 1])?;

    let body = temp.drill(&walker, vec![1, 0, 1])?;
    let body = resolve_body(&body, &walker)?;

    let footer = temp.drill(&walker, vec![2, 0])?;
    let footer = resolve_footer(&footer, automation)?;
    drop(temp);

    Ok(AppWindow::VSCode(VSCodeWindow {
        focused,
        header: VSCodeWindowHeader {},
        body,
        footer,
    }))
}

fn resolve_body(
    body: &UIElement,
    walker: &UITreeWalker,
) -> Result<VSCodeWindowBody, AppResolveError> {
    let state =
        VSCodeCrawlState::try_from(gather_children(walker, body, &StopBehaviour::EndOfSiblings))?;

    let side_nav = state
        .get_side_nav_tabs_root_elem()?
        .drill(walker, vec![0, 0])?
        .gather_children(walker, &StopBehaviour::LastChildEncountered);
    // println!("side_nav: {:?}", side_nav);
    let side_nav = side_nav
        .into_iter()
        .filter(|elem| elem.get_control_type() == Ok(ControlType::TabItem))
        .map(|elem| {
            let name = elem.get_name()?;
            let kind = SideTabKind::try_from(name)?;
            let active = elem
                .get_property_value(UIProperty::ExpandCollapseExpandCollapseState)
                .map(|v| v.try_into() == Ok(ExpandCollapseState::Expanded as i32))
                .unwrap_or_default();
            if active {
                let view = state
                    .get_side_nav_view_root_elem()?
                    .drill(walker, vec![1])?;
                let view = match view.get_automation_id() {
                    Ok(id)
                        if Some(id.as_str()) == SideTabKind::Explorer.get_view_automation_id() =>
                    {
                        fn as_explorer_item(
                            walker: &UITreeWalker,
                            tree_item: UIElement,
                        ) -> Result<ExplorerItem, AppResolveError> {
                            let label = tree_item.get_name()?;
                            let ui_position_in_set = tree_item
                                .get_property_value(UIProperty::PositionInSet)?
                                .try_into()?;
                            let ui_size_of_set = tree_item
                                .get_property_value(UIProperty::SizeOfSet)?
                                .try_into()?;
                            let ui_level = tree_item
                                .get_property_value(UIProperty::Level)?
                                .try_into()?;
                            let bounds = tree_item.get_bounding_rectangle()?.to_bevy_irect();
                            let kind = tree_item
                                .get_pattern::<UIExpandCollapsePattern>()
                                .ok()
                                .map(|p| ExplorerItemKind::Directory {
                                    expanded: p.get_state() == Ok(ExpandCollapseState::Expanded),
                                })
                                .unwrap_or(ExplorerItemKind::File);
                            let path = tree_item
                                .drill(
                                    walker,
                                    match kind {
                                        ExplorerItemKind::File => vec![0, 1, 0],
                                        ExplorerItemKind::Directory { .. } => {
                                            vec![0, 2, 0]
                                        }
                                    },
                                )?
                                .get_name()?;
                            Ok(ExplorerItem {
                                label,
                                path,
                                ui_position_in_set,
                                ui_size_of_set,
                                ui_level,
                                bounds,
                                kind,
                            })
                        }
                        let sticky = view
                            .drill(walker, vec![0, 1, 0, 0, 1, 0, 3])?
                            .gather_children(walker, &StopBehaviour::EndOfSiblings)
                            .into_iter()
                            .filter_map(|tree_item| as_explorer_item(walker, tree_item).ok())
                            .collect();
                        let items = view
                            .drill(walker, vec![0, 1, 0, 0, 1, 0, 0])?
                            .gather_children(walker, &StopBehaviour::EndOfSiblings)
                            .into_iter()
                            .filter_map(|tree_item| as_explorer_item(walker, tree_item).ok())
                            .collect();
                        View::Explorer { sticky, items }
                    }
                    _ => {
                        View::Unknown {}
                        // elem: view.into()
                    }
                };

                Ok(SideTab::Open {
                    kind,
                    // button: elem.into(),
                    view,
                })
            } else {
                Ok(SideTab::Closed {
                    kind,
                    // button: elem.into(),
                })
            }
        })
        .filter_map(|res: Result<SideTab, AppResolveError>| res.ok())
        .collect();

    let editor_area_elem = state
        .get_editor_root_elem()?
        .drill(walker, vec![0, 1, 0, 0])?;
    if editor_area_elem.get_automation_id()? != EditorArea::get_expected_automation_id() {
        return Err(AppResolveError::BadStructure(format!(
            "Editor area has wrong automation id: {}",
            editor_area_elem.get_automation_id()?
        )));
    }
    let editor_groups = editor_area_elem
        .drill(walker, vec![0, 0, 0, 1])?
        .gather_children(walker, &StopBehaviour::EndOfSiblings)
        .into_iter()
        .map(|group_elem| {
            let group_tabs_holder = group_elem.drill(walker, vec![0, 0, 0])?;
            let selected: Option<String> = group_tabs_holder
                .get_property_value(UIProperty::SelectionSelection)?
                .try_into()
                .ok();
            let group_tabs = group_tabs_holder
                .gather_children(walker, &StopBehaviour::EndOfSiblings)
                .into_iter()
                .map(|group_tab_elem| {
                    let title = group_tab_elem.get_name()?;
                    let active = selected == Some(title.clone());
                    Ok(EditorTab { title, active })
                })
                .filter_map(|r: Result<EditorTab, AppResolveError>| r.ok())
                .collect();
            let content_elem = group_elem.drill(walker, vec![1, 0, 0, 1])?;
            let content = content_elem
                .get_property_value(UIProperty::LegacyIAccessibleValue)
                .map(|variant| variant.to_string())
                .map(|text_content| EditorContent {
                    content: text_content,
                })
                .ok();

            Ok(EditorGroup {
                tabs: group_tabs,
                content,
            })
        })
        .filter_map(|r: Result<EditorGroup, AppResolveError>| r.ok())
        .collect();
    let editor_area = EditorArea {
        groups: editor_groups,
    };

    Ok(VSCodeWindowBody {
        editor_area,
        side_nav,
    })
}

fn resolve_footer(
    footer: &UIElement,
    automation: &UIAutomation,
) -> Result<VSCodeWindowFooter, AppResolveError> {
    let condition = automation.create_property_condition(
        UIProperty::AutomationId,
        Variant::from("status.editor.selection"),
        None,
    )?;
    let cursor_position_elem = footer.find_first(TreeScope::Children, &condition)?;
    let text = cursor_position_elem.get_name()?;
    // "Ln 218, Col 5"
    let cursor_position = text
        .split(", ")
        .map(|part| part.split(' ').last().and_then(|s| s.parse::<usize>().ok()))
        .collect_vec();
    let cursor_position = match cursor_position.as_slice() {
        [Some(line), Some(column)] => IVec2::new(*column as i32, *line as i32),
        _ => {
            return Err(AppResolveError::BadStructure(format!(
                "Bad cursor position {:?}",
                cursor_position
            )))
        }
    };
    Ok(VSCodeWindowFooter { cursor_position })
}
