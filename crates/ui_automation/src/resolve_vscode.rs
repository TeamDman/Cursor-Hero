use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;
use anyhow::Context;
use anyhow::Result;
use bevy::math::IVec2;
use cursor_hero_ui_automation_types::prelude::*;
use itertools::Itertools;
use uiautomation::types::TreeScope;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

pub(crate) fn resolve_vscode(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppWindow> {
    let walker = automation.create_tree_walker().context("creating walker")?;
    let root = elem;

    let temp = root
        .drill(&walker, vec![0, 0, 0, 0, 0, 1])
        .context("drilling temp")?;

    let body = temp
        .drill(&walker, vec![1, 0, 1])
        .context("drilling body")?;
    let body = match resolve_body(&body, &walker) {
        Ok(body) => body,
        Err(e) => {
            return Err(e.context("resolving body"));
        }
    };

    let footer = temp.drill(&walker, vec![2, 0]).context("drilling footer")?;
    let footer = resolve_footer(&footer, automation).context("resolving footer")?;
    drop(temp);

    Ok(AppWindow::VSCode(VSCodeWindow {
        focused,
        header: VSCodeWindowHeader {},
        body,
        footer,
    }))
}

fn resolve_body(body: &UIElement, walker: &UITreeWalker) -> Result<VSCodeWindowBody> {
    let workbench_parts_editor = body
        .drill(walker, vec![0, 0, 1, 0, 0])
        .context("drilling to find editor area")?;
    if workbench_parts_editor.get_automation_id()? != EditorArea::get_expected_automation_id() {
        return Err(AppResolveError::BadStructure(format!(
            "workbench_parts_editor has wrong automation id, got unexpected element {:?}",
            workbench_parts_editor.get_runtime_id(),
        ))
        .into());
    }
    let editor_groups = workbench_parts_editor
        .drill(walker, vec![0, 0, 0, 1])
        .context("drilling to find editor groups")?
        .gather_children(walker, &StopBehaviour::EndOfSiblings)
        .into_iter()
        .map(|group_elem| {
            let tab_container = group_elem
                .drill(walker, vec![0, 0, 0])
                .context("drilling to find editor groups tab container")?;
            let selected: Option<String> = tab_container
                .get_property_value(UIProperty::SelectionSelection)?
                .try_into()
                .ok();
            let group_tabs = tab_container
                .gather_children(walker, &StopBehaviour::EndOfSiblings)
                .into_iter()
                .map(|group_tab_elem| {
                    let title = group_tab_elem.get_name()?;
                    let active = selected == Some(title.clone());
                    Ok(EditorTab { title, active })
                })
                .filter_map(|r: Result<EditorTab>| r.ok())
                .collect();
            let content_elem = group_elem
                .drill(walker, vec![1, 0, 0, 1])
                .context("drilling to find group content")?;
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
        .filter_map(|r: Result<EditorGroup>| r.ok())
        .collect();
    let editor_area = EditorArea {
        groups: editor_groups,
    };

    // let side_nav = state
    //     .get_side_nav_tabs_root_elem()
    //     .drill(walker, vec![0, 0]).context("drilling to find side_nav")?
    //     .gather_children(walker, &StopBehaviour::LastChildEncountered);
    // // println!("side_nav: {:?}", side_nav);
    // let side_nav = side_nav
    //     .into_iter()
    //     .filter(|elem| elem.get_control_type() == Ok(ControlType::TabItem))
    //     .map(|elem| {
    //         let name = elem.get_name()?;
    //         let kind: SideTabKind = SideTabKind::try_from(name)?;
    //         let active = elem
    //             .get_property_value(UIProperty::ExpandCollapseExpandCollapseState)
    //             .map(|v| v.try_into() == Ok(ExpandCollapseState::Expanded as i32))
    //             .unwrap_or_default();
    //         if active {
    //             let view = state
    //                 .get_side_nav_view_root_elem()
    //                 .drill(walker, vec![1]).context("drilling to find side tab view")?;
    //             let view = match view.get_automation_id() {
    //                 Ok(id)
    //                     if Some(id.as_str()) == SideTabKind::Explorer.get_view_automation_id() =>
    //                 {
    //                     fn as_explorer_item(
    //                         walker: &UITreeWalker,
    //                         tree_item: UIElement,
    //                     ) -> Result<ExplorerItem> {
    //                         let label = tree_item.get_name()?;
    //                         let ui_position_in_set = tree_item
    //                             .get_property_value(UIProperty::PositionInSet)?
    //                             .try_into()?;
    //                         let ui_size_of_set = tree_item
    //                             .get_property_value(UIProperty::SizeOfSet)?
    //                             .try_into()?;
    //                         let ui_level = tree_item
    //                             .get_property_value(UIProperty::Level)?
    //                             .try_into()?;
    //                         let bounds = tree_item.get_bounding_rectangle()?.to_bevy_irect();
    //                         let kind = tree_item
    //                             .get_pattern::<UIExpandCollapsePattern>()
    //                             .ok()
    //                             .map(|p| ExplorerItemKind::Directory {
    //                                 expanded: p.get_state() == Ok(ExpandCollapseState::Expanded),
    //                             })
    //                             .unwrap_or(ExplorerItemKind::File);
    //                         let path = tree_item
    //                             .drill(
    //                                 walker,
    //                                 match kind {
    //                                     ExplorerItemKind::File => vec![0, 1, 0],
    //                                     ExplorerItemKind::Directory { .. } => {
    //                                         vec![0, 2, 0]
    //                                     }
    //                                 },
    //                             ).context("drilling to find explorer item path")?
    //                             .get_name()?;
    //                         Ok(ExplorerItem {
    //                             label,
    //                             path,
    //                             ui_position_in_set,
    //                             ui_size_of_set,
    //                             ui_level,
    //                             bounds,
    //                             kind,
    //                         })
    //                     }
    //                     let sticky = view
    //                         .drill(walker, vec![0, 1, 0, 0, 1, 0, 3]).context("drilling to find explorer sticky")?
    //                         .gather_children(walker, &StopBehaviour::EndOfSiblings)
    //                         .into_iter()
    //                         .filter_map(|tree_item| as_explorer_item(walker, tree_item).ok())
    //                         .collect();
    //                     let items = view
    //                         .drill(walker, vec![0, 1, 0, 0, 1, 0, 0]).context("drilling to find explorer items")?
    //                         .gather_children(walker, &StopBehaviour::EndOfSiblings)
    //                         .into_iter()
    //                         .filter_map(|tree_item| as_explorer_item(walker, tree_item).ok())
    //                         .collect();
    //                     View::Explorer { sticky, items }
    //                 }
    //                 _ => {
    //                     View::Unknown {}
    //                     // elem: view.into()
    //                 }
    //             };

    //             Ok(SideTab::Open {
    //                 kind,
    //                 // button: elem.into(),
    //                 view,
    //             })
    //         } else {
    //             Ok(SideTab::Closed {
    //                 kind,
    //                 // button: elem.into(),
    //             })
    //         }
    //     })
    //     .filter_map(|res: Result<SideTab>| res.ok())
    //     .collect();

    let right_tab = SideTab::Closed {
        kind: SideTabKind::Explorer,
    };

    Ok(VSCodeWindowBody {
        editor_area,
        right_tab,
    })
}

fn resolve_footer(footer: &UIElement, automation: &UIAutomation) -> Result<VSCodeWindowFooter> {
    let condition = automation
        .create_property_condition(
            UIProperty::AutomationId,
            Variant::from("status.editor.selection"),
            None,
        )
        .context("creating condition")?;
    let cursor_position_elem = footer
        .find_first(TreeScope::Children, &condition)
        .context("finding first")?;
    let text = cursor_position_elem.get_name().context("getting name")?;
    // "Ln 218, Col 5"
    // "Ln 218, Col 5 (15 selected)"

    let cursor_position = text
        .split(", ")
        .map(|part| part.split(' ').nth(1).and_then(|s| s.parse::<usize>().ok()))
        .collect_vec();
    let cursor_position = match cursor_position.as_slice() {
        [Some(line), Some(column)] => {
            IVec2::new(*column as i32, *line as i32)
        }
        _ => {
            return Err(AppResolveError::BadStructure(format!(
                "Bad cursor position {:?}",
                cursor_position
            )))
            .context("bad text cursor position composition");
        }
    };
    Ok(VSCodeWindowFooter { cursor_position })
}
