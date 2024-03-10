use cursor_hero_ui_automation_types::ui_automation_types::AppResolveError;
use cursor_hero_ui_automation_types::ui_automation_types::AppWindow;
use cursor_hero_ui_automation_types::ui_automation_types::Drillable;
use cursor_hero_ui_automation_types::ui_automation_types::EditorArea;
use cursor_hero_ui_automation_types::ui_automation_types::EditorContent;
use cursor_hero_ui_automation_types::ui_automation_types::EditorGroup;
use cursor_hero_ui_automation_types::ui_automation_types::EditorTab;
use cursor_hero_ui_automation_types::ui_automation_types::SideTab;
use cursor_hero_ui_automation_types::ui_automation_types::SideTabKind;
use cursor_hero_ui_automation_types::ui_automation_types::ToBevyIRect;
use cursor_hero_ui_automation_types::ui_automation_types::VSCodeState;
use cursor_hero_ui_automation_types::ui_automation_types::View;
use uiautomation::controls::ControlType;
use uiautomation::types::ExpandCollapseState;
use uiautomation::types::UIProperty;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use crate::gather_children::gather_children;
use crate::gather_children::GatherChildrenable;
use crate::gather_children::StopBehaviour;

pub(crate) fn resolve_app(
    elem: &UIElement,
    automation: &UIAutomation,
    focused: bool,
) -> Result<AppWindow, AppResolveError> {
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
                .gather_children(&walker, &StopBehaviour::LastChildEncountered);
            println!("side_nav: {:?}", side_nav);
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
                            .drill(&walker, vec![1])?;
                        let view = match view.get_automation_id() {
                            Ok(id)
                                if Some(id.as_str())
                                    == SideTabKind::Explorer.get_view_automation_id() =>
                            {
                                View::Explorer {}
                                // elem: view.into()
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
                                // elem: group_tab_elem.into(),
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
                            // elem: content_elem.into(),
                        })
                        .ok();

                    Ok(EditorGroup {
                        tabs: group_tabs,
                        // elem: group_elem.into(),
                        content,
                    })
                })
                .filter_map(|r: Result<EditorGroup, AppResolveError>| r.ok())
                .collect();
            let editor_area = EditorArea {
                groups: editor_groups,
                // elem: editor_area_elem.into(),
            };

            Ok(AppWindow::VSCode {
                // root: root.into(),
                focused,
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
            //     elem: nav_elem.into(),
            // };
            // let view = match n {
            //     3 => {
            //         let left_view_elem = x_kids.remove(0);
            //         let left_view = View::Explorer {
            //             elem: left_view_elem.into(),
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
            //         elem: editor.into(),
            //     },
            //     tabs: left_area,
            // };
            // Ok(vscode)
        }
        _ => Err(AppResolveError::NoMatch),
    }
}
