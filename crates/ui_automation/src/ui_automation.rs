use bevy::math::IVec2;
use bevy::math::Rect;
use cursor_hero_ui_automation_types::ui_automation_types::all_of;
use cursor_hero_ui_automation_types::ui_automation_types::AppResolveError;
use cursor_hero_ui_automation_types::ui_automation_types::AppWindow;
use cursor_hero_ui_automation_types::ui_automation_types::DrillError;
use cursor_hero_ui_automation_types::ui_automation_types::Drillable;
use cursor_hero_ui_automation_types::ui_automation_types::EditorArea;
use cursor_hero_ui_automation_types::ui_automation_types::EditorContent;
use cursor_hero_ui_automation_types::ui_automation_types::EditorGroup;
use cursor_hero_ui_automation_types::ui_automation_types::EditorTab;
use cursor_hero_ui_automation_types::ui_automation_types::ElementInfo;
use cursor_hero_ui_automation_types::ui_automation_types::GatherAppsError;
use cursor_hero_ui_automation_types::ui_automation_types::HexList;
use cursor_hero_ui_automation_types::ui_automation_types::SideTab;
use cursor_hero_ui_automation_types::ui_automation_types::SideTabKind;
use cursor_hero_ui_automation_types::ui_automation_types::ToBevyIRect;
use cursor_hero_ui_automation_types::ui_automation_types::VSCodeState;
use cursor_hero_ui_automation_types::ui_automation_types::View;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt;
use std::io::Error;
use uiautomation::controls::ControlType;
use uiautomation::types::ExpandCollapseState;
use uiautomation::types::Point;
use uiautomation::types::TreeScope;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;

use crate::gather_children::gather_children;
use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;

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

pub fn gather_apps() -> Result<Vec<AppWindow>, GatherAppsError> {
    let automation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    println!("Boutta gather top level children");
    // let walker = automation.create_tree_walker()?;
    // let found = gather_children(&walker, &root, &GatherChildrenStopBehaviour::EndOfSiblings);
    let condition = &automation.create_true_condition()?;
    let found = root.find_all(TreeScope::Children, condition)?;
    println!("Found {} top level children", found.len());
    let mut apps = vec![];
    let mut errors = vec![];
    for elem in found {
        match resolve_app(elem, &automation) {
            Ok(app) => {
                apps.push(app);
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
    Ok(apps)
}

fn resolve_app(elem: UIElement, automation: &UIAutomation) -> Result<AppWindow, AppResolveError> {
    match (
        elem.get_name(),
        elem.get_control_type(),
        elem.get_localized_control_type(),
        elem.get_classname(),
        elem.get_bounding_rectangle().map(|r| r.to_bevy_irect()),
    ) {
        (Ok(name), Ok(ControlType::Pane), _, Ok(class_name), _)
            if name.ends_with("Visual Studio Code") && class_name == "Chrome_WidgetWin_1" =>
        {
            let walker = automation.create_tree_walker()?;
            let root = elem;
            let deep_root = root.drill(&walker, vec![0, 0, 0, 0, 0, 1, 1, 0, 1])?;

            let state = VSCodeState::try_from(gather_children(
                &walker,
                &deep_root,
                &StopBehaviour::EndOfSiblings,
            ))?;

            let side_nav = state
                .get_side_nav_tabs_root_elem()?
                .drill(&walker, vec![0, 0])?
                .gather_children(&walker, &StopBehaviour::EndOfSiblings)
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
                            .drill(&walker, vec![1])?;
                        let view = match view.get_automation_id() {
                            Ok(id)
                                if Some(id.as_str())
                                    == SideTabKind::Explorer.get_view_automation_id() =>
                            {
                                View::Explorer { elem: view }
                            }
                            _ => View::Unknown { elem: view },
                        };

                        Ok(SideTab::Open {
                            kind,
                            button: elem,
                            view,
                        })
                    } else {
                        Ok(SideTab::Closed { kind, button: elem })
                    }
                })
                .filter_map(|res: Result<SideTab, AppResolveError>| res.ok())
                .collect();

            let editor_area_elem = state
                .get_editor_root_elem()?
                .drill(&walker, vec![0, 1, 0, 0])?;
            if editor_area_elem.get_automation_id()? != EditorArea::get_expected_automation_id() {
                return Err(AppResolveError::BadStructure(format!(
                    "Editor area has wrong automation id: {}",
                    editor_area_elem.get_automation_id()?
                )));
            }
            let editor_groups = editor_area_elem
                .drill(&walker, vec![0, 0, 0, 1])?
                .gather_children(&walker, &StopBehaviour::EndOfSiblings)
                .into_iter()
                .map(|group_elem| {
                    let group_tabs_holder = group_elem.drill(&walker, vec![0, 0, 0])?;
                    let selected: Option<String> = group_tabs_holder
                        .get_property_value(UIProperty::SelectionSelection)?
                        .try_into()
                        .ok();
                    let group_tabs = group_tabs_holder
                        .gather_children(&walker, &StopBehaviour::EndOfSiblings)
                        .into_iter()
                        .map(|group_tab_elem| {
                            let title = group_tab_elem.get_name()?;
                            let active = selected == Some(title.clone());
                            Ok(EditorTab {
                                title,
                                elem: group_tab_elem,
                                active,
                            })
                        })
                        .filter_map(|r: Result<EditorTab, AppResolveError>| r.ok())
                        .collect();
                    let content_elem = group_elem.drill(&walker, vec![1, 0, 0, 1])?;
                    let content = content_elem
                        .get_property_value(UIProperty::LegacyIAccessibleValue)
                        .map(|variant| variant.to_string())
                        .map(|text_content| EditorContent {
                            content: text_content,
                            elem: content_elem,
                        })
                        .ok();

                    Ok(EditorGroup {
                        tabs: group_tabs,
                        elem: group_elem,
                        content,
                    })
                })
                .filter_map(|r: Result<EditorGroup, AppResolveError>| r.ok())
                .collect();
            let editor_area = EditorArea {
                groups: editor_groups,
                elem: editor_area_elem,
            };

            Ok(AppWindow::VSCode {
                root,
                editor_area,
                side_nav,
            })

            // let editor_condition = automation.create_property_condition(
            //     UIProperty::AutomationId,
            //     Variant::from("workbench.parts.editor"),
            //     None,
            // )?;
            // let editor = root.find_first(TreeScope::Descendants, &editor_condition)?;
            // println!(
            //     "Found editor with runtime id {}",
            //     editor.get_runtime_id()?.to_hex_list()
            // );
            // let mut x = editor.clone();
            // for _ in 0..5 {
            //     x = walker.get_parent(&x)?;
            // }

            // // let tru = &automation.create_true_condition()?;
            // // let mut x_kids = x.find_all(TreeScope::Children, )?;
            // let mut x_kids =
            //     gather_children(&walker, &x, &GatherChildrenStopBehaviour::EndOfSiblings);
            // let n = x_kids.len();
            // if !matches!(n, 2..=3) {
            //     return Err(AppResolveError::BadStructure(format!(
            //         "Editor area has wrong number of children: {}",
            //         n
            //     )));
            // }
            // let nav_elem = x_kids.remove(0);
            // let buttons = nav_elem
            //     .find_all(
            //         TreeScope::Descendants,
            //         &automation.create_property_condition(
            //             UIProperty::ControlType,
            //             (ControlType::TabItem as i32).into(),
            //             None,
            //         )?,
            //     )?
            //     .into_iter()
            //     .map(|elem| {
            //         let name = elem.get_name().unwrap_or_default();
            //         let active = elem
            //             .get_property_value(UIProperty::ExpandCollapseExpandCollapseState)
            //             .map(|v| v.try_into() == Ok(ExpandCollapseState::Expanded as i32))
            //             .unwrap_or_default();
            //         TabButton { elem, name, active }
            //     })
            //     .collect();
            // let nav = LeftNav {
            //     buttons,
            //     elem: nav_elem,
            // };
            // let view = match n {
            //     3 => {
            //         let left_view_elem = x_kids.remove(0);
            //         let left_view = View::Explorer {
            //             elem: left_view_elem,
            //         };
            //         Some(left_view)
            //     }
            //     _ => None,
            // };
            // let left_area = Tab { nav, view };

            // let vscode = AppWindow::VSCode {
            //     root,
            //     editor_area: EditorArea {
            //         groups: vec![],
            //         elem: editor,
            //     },
            //     tabs: left_area,
            // };
            // Ok(vscode)
        }
        _ => Err(AppResolveError::NoMatch),
    }
}

pub fn gather_focus() -> Result<AppWindow, AppResolveError> {
    let automation = UIAutomation::new()?;
    let focused = automation.get_focused_element()?;
    let walker = automation.create_tree_walker()?;
    let ancestor = walker.normalize(&focused)?;
    let app = resolve_app(ancestor, &automation)?;
    Ok(app)
}

pub fn gather_shallow_element_info(
    element: UIElement,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    let runtime_id = element.get_runtime_id()?;

    let info = ElementInfo {
        name,
        bounding_rect: Rect::new(
            bb.get_left() as f32,
            bb.get_top() as f32,
            bb.get_right() as f32,
            bb.get_bottom() as f32,
        ),
        control_type: class_name.clone(),
        class_name,
        automation_id,
        runtime_id,
        children: None,
    };
    Ok(info)
}

pub fn gather_deep_element_info(
    element: UIElement,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let automation = UIAutomation::new()?;
    let walker = automation.create_tree_walker()?;
    gather_deep_element_info_inner(element, &walker, &StopBehaviour::EndOfSiblings)
}

fn gather_deep_element_info_inner(
    element: UIElement,
    walker: &UITreeWalker,
    stop_behaviour: &StopBehaviour,
) -> Result<ElementInfo, uiautomation::errors::Error> {
    let name = element.get_name()?;
    let bb = element.get_bounding_rectangle()?;
    let class_name = element.get_classname()?;
    let automation_id = element.get_automation_id()?;
    let runtime_id = element.get_runtime_id()?;
    let mut children = vec![];

    for child in gather_children(walker, &element, stop_behaviour) {
        let child_info = gather_shallow_element_info(child)?;
        children.push(child_info);
    }

    let info = ElementInfo {
        name,
        bounding_rect: Rect::new(
            bb.get_left() as f32,
            bb.get_top() as f32,
            bb.get_right() as f32,
            bb.get_bottom() as f32,
        ),
        control_type: class_name.clone(),
        class_name,
        automation_id,
        runtime_id,
        children: Some(children),
    };
    Ok(info)
}

// pub fn get_element_from_identifier(id: &str) -> Result<UIElement, uiautomation::Error> {
//     let automation = UIAutomation::new()?;
//     // find the elem.get_automation_id() that matches id
//     let filter = automation.create_property_condition(
//         uiautomation::types::UIProperty::AutomationId,
//         uiautomation::variants::Variant::from(id),
//         None,
//     )?;
//     let walker = automation.filter_tree_walker(filter)?;
//     let root = automation.get_root_element()?;
//     let elem = find_recursive(&walker, &root)?;

// }

// fn find_recursive(walker: &UITreeWalker, element: &UIElement) -> Result<UIElement, uiautomation::Error> {
//     if element.get_automation_id()? == id {
//         return Ok(element);
//     }

//     if let Ok(child) = walker.get_first_child(&element) {
//         if let Ok(elem) = find_recursive(walker, &child) {
//             return Ok(elem);
//         }

//         let mut next = child;
//         while let Ok(sibling) = walker.get_next_sibling(&next) {
//             if let Ok(elem) = find_recursive(walker, &sibling) {
//                 return Ok(elem);
//             }

//             next = sibling;
//         }
//     }

//     Err(uiautomation::Error::from_win32(0))
// }

#[cfg(test)]
mod tests {
    use crate::prelude::get_taskbar;

    #[test]
    fn test_get_taskbar() {
        let taskbar = get_taskbar().unwrap();
        assert!(taskbar.entries.len() > 0);
        // print the entries
        for entry in taskbar.entries {
            println!("entry: {:?}", entry);
        }
    }

    #[test]
    fn test_gather_apps() {
        // Restarting my computer made this faster
        // I hate this. I hate this so much.
        let start = std::time::Instant::now();
        let apps = super::gather_apps().unwrap();
        assert!(apps.len() > 0);
        for app in apps {
            println!("app: {:?}", app);
        }
        let end = std::time::Instant::now();
        println!("time: {:?}", end - start);
        assert!(end - start < std::time::Duration::from_secs(1));

        // std::thread::spawn(|| {
        // })
        // .join()
        // .unwrap();
    }
}
