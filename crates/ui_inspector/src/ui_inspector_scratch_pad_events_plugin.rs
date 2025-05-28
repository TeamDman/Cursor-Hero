use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_app_types::app_types::CursorHeroAppKind;
use cursor_hero_bevy::prelude::display_enum_qualified_variant_instance;
use cursor_hero_bevy::prelude::display_enum_unqualified_variant_definition;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
use cursor_hero_explorer_app_types::prelude::ExplorerElementKind;
use cursor_hero_screen::get_image::AsBevyColor;
use cursor_hero_screen::get_image::ImageHolder;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_inspector_types::prelude::InspectorScratchPadEvent;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::UIData;
use itertools::Itertools;

pub struct UiInspectorScratchPadEventsPlugin;

impl Plugin for UiInspectorScratchPadEventsPlugin {
    fn build(&self, app: &mut App) {
        let condition =
            |ui_data: Res<UIData>| ui_data.windows.global_toggle && ui_data.windows.scratch_pad.open;
        app.add_systems(
            Update,
            handle_append_all_known_scratch_pad_events.run_if(condition),
        );
        app.add_systems(
            Update,
            handle_append_all_unknown_scratch_pad_events.run_if(condition),
        );
        app.add_systems(
            Update,
            handle_append_all_scratch_pad_events.run_if(condition),
        );
        app.add_systems(
            Update,
            handle_append_single_info_scratch_pad_events.run_if(condition),
        );
    }
}

fn get_content_many(
    infos: Vec<&ElementInfo>,
    mode: &ScratchPadMode,
    app_kind: &Option<CursorHeroAppKind>,
    ui_data: &UIData,
    screen_access: &ScreensToImageParam,
) -> String {
    // infos
    //     .into_iter()
    //     .map(|info| get_content(info, mode, app_kind, ui_data, screen_access))
    //     .unique()
    //     .join("\n")
    let transform_reflect =
        |info: &ElementInfo, transform: fn(&dyn Reflect) -> Option<String>| -> Option<String> {
            match app_kind {
                Some(CursorHeroAppKind::Calculator)
                    if let Some(calc_elem_kind) = CalculatorElementKind::from_info(info) =>
                {
                    transform(calc_elem_kind.as_reflect())
                }
                Some(CursorHeroAppKind::Explorer)
                    if let Some(explorer_elem_kind) =
                        ExplorerElementKind::from_window_relative_drill_id(&info.drill_id) =>
                {
                    transform(explorer_elem_kind.as_reflect())
                }
                _ => None,
            }
        };
    let mut rtn = match mode {
        ScratchPadMode::Identify => infos
            .into_iter()
            .map(|info| {
                format!(
                    "{},",
                    transform_reflect(info, display_enum_unqualified_variant_definition)
                        .unwrap_or_else(|| info.as_pascal())
                )
            })
            .unique()
            .sorted()
            .join("\n"),
        ScratchPadMode::MapIdentify => infos
            .into_iter()
            .map(|info| {
                (
                    info,
                    transform_reflect(info, display_enum_qualified_variant_instance)
                        .or_else(|| {
                            app_kind.as_ref().map(|k| {
                                format!("{}::{}", k.element_kind_enum_name(), info.as_pascal())
                            })
                        })
                        .unwrap_or_else(|| info.as_pascal()),
                )
            })
            .sorted_by_key(|(_, name)| name.clone())
            .map(|(info, name)| {
                let mut conditions = vec![];
                if !info.name.is_empty() {
                    conditions.push(format!("info.name == \"{}\"", info.name));
                }
                if !info.class_name.is_empty() {
                    conditions.push(format!("info.class_name == \"{}\"", info.class_name));
                }
                if !info.automation_id.is_empty() {
                    conditions.push(format!("info.automation_id == \"{}\"", info.automation_id));
                }

                let mut rtn = "info if ".to_string();
                rtn.push_str(&format!("{:width$}", conditions.join(" && "), width = 124));
                rtn.push_str(&format!(" => Some({}),", name));
                rtn
            })
            .join("\n"),
        ScratchPadMode::PerformDrill => infos
            .into_iter()
            .map(|info| (info, info.as_identifier()))
            .sorted_by_key(|(_, name)| name.clone())
            .map(|(info, name)| {
                let drill_id = info
                    .drill_id
                    .as_child()
                    .map(|d| d.iter().skip(1).map(|x| x.to_string()).join(", "))
                    .unwrap_or_default();

                format!(
                    "let {0} = root.drill(&walker, vec![{1}]).context(\"{0}\")?.try_into()?;",
                    name, drill_id,
                )
            })
            .join("\n"),
        ScratchPadMode::MapDrill => infos
            .into_iter()
            .map(|info| (info, info.as_identifier()))
            .sorted_by_key(|(_, name)| name.clone())
            .map(|(info, name)| {
                let drill_id = info
                    .drill_id
                    .as_child()
                    .map(|d| d.iter().skip(1).cloned().collect::<DrillId>())
                    .unwrap_or_default();

                format!(
                    "{} => {},",
                    transform_reflect(info, display_enum_qualified_variant_instance)
                        .unwrap_or_else(|| name),
                    drill_id
                )
            })
            .join("\n"),
        ScratchPadMode::MapBounds => {
            let compare = ui_data
                .mark
                .as_ref()
                .and_then(|mark_drill_id| ui_data.tree.lookup_drill_id(mark_drill_id.clone()))
                .or_else(|| {
                    ui_data.selected.as_ref().and_then(|selected_drill_id| {
                        ui_data.tree.find_first_child(selected_drill_id)
                    })
                })
                .unwrap_or(&ui_data.tree);

            infos
                .into_iter()
                .map(|info| {
                    (
                        info,
                        transform_reflect(info, display_enum_qualified_variant_instance)
                            .unwrap_or_else(|| info.as_identifier()),
                    )
                })
                .sorted_by_key(|(_, name)| name.clone())
                .map(|(info, name)| {
                    // Get the bounds of the selected element relative to the comparison element
                    let bounds_relative = info
                        .bounding_rect
                        .translated(&-compare.bounding_rect.top_left());

                    // Format as string
                    format!(
                        "{} => Rect::new({:.1},{:.1},{:.1},{:.1}),",
                        transform_reflect(info, display_enum_qualified_variant_instance)
                            .unwrap_or_else(|| name),
                        bounds_relative.top_left().x as f32,
                        -bounds_relative.top_left().y as f32,
                        bounds_relative.bottom_right().x as f32,
                        -bounds_relative.bottom_right().y as f32
                    )
                })
                .join("\n")
        }
        ScratchPadMode::MapColor => {
            infos
                .into_iter()
                .map(|info| {
                    (
                        info,
                        transform_reflect(info, display_enum_qualified_variant_instance)
                            .unwrap_or_else(|| info.as_identifier()),
                    )
                })
                .sorted_by_key(|(_, name)| name.clone())
                .map(|(info, name)| {
                    let image = screen_access.get_image_buffer(info.bounding_rect);
                    let color = match image {
                        Ok(image) => {
                            // find the most common colour
                            let mut color_counts = HashMap::new();
                            for (_, _, pixel) in image.enumerate_pixels() {
                                *color_counts.entry(pixel).or_insert(0) += 1;
                            }
                            color_counts
                                .into_iter()
                                .max_by_key(|(_, count)| *count)
                                .map(|(image_color, _)| image_color.as_bevy_color())
                                .unwrap_or(Color::BLACK)
                        }
                        Err(e) => {
                            warn!(
                                "Failed to get image for region {:?}: {e:?}",
                                info.bounding_rect
                            );
                            Color::BLACK
                        }
                    };
                    format!(
                        "{} => Color::rgb({:.1},{:.1},{:.1}),",
                        transform_reflect(info, display_enum_qualified_variant_instance)
                            .unwrap_or_else(|| name),
                        color.r(),
                        color.g(),
                        color.b()
                    )
                })
                .join("\n")
        
        },
        ScratchPadMode::MapText => {
            infos
                .into_iter()
                .map(|info| {
                    (
                        info,
                        transform_reflect(info, display_enum_qualified_variant_instance)
                            .unwrap_or_else(|| info.as_identifier()),
                    )
                })
                .sorted_by_key(|(_, name)| name.clone())
                .map(|(info, name)| {
                    let text = info.children.as_ref().and_then(|children| {
                        children
                            .iter()
                            .find(|child| child.control_type == ControlType::Text)
                            .map(|child| child.name.to_owned())
                    });
                    if let Some(text) = text {
                        format!(
                            "{} => Some(\"{}\".to_string()),",
                            transform_reflect(info, display_enum_qualified_variant_instance)
                                .unwrap_or_else(|| name),
                            text
                        )
                    } else {
                        format!(
                            "{} => None,",
                            transform_reflect(info, display_enum_qualified_variant_instance)
                                .unwrap_or_else(|| name)
                        )
                    }
                })
                .join("\n")
        },
    };
    rtn.push('\n');
    rtn
}

fn handle_append_all_known_scratch_pad_events(
    mut inspector_events: EventReader<InspectorScratchPadEvent>,
    mut ui_data: ResMut<UIData>,
    screen_access: ScreensToImageParam,
) {
    for event in inspector_events.read() {
        let InspectorScratchPadEvent::ScratchPadAppendAllKnown = event else {
            continue;
        };

        // get selected info
        let Some(selected_drill_id) = &ui_data.selected else {
            return;
        };

        // get window
        let Some(window) = ui_data.tree.find_first_child(selected_drill_id) else {
            warn!(
                "Selected drill id not found in tree: {:?}",
                selected_drill_id
            );
            return;
        };

        let app_kind = CursorHeroAppKind::from_window(window);
        let mut push_infos = match app_kind {
            Some(CursorHeroAppKind::Calculator) => window
                .get_descendents()
                .into_iter()
                .filter(|info| CalculatorElementKind::from_info(info).is_some())
                .collect(),
            Some(CursorHeroAppKind::Explorer) => std::iter::once(window)
                .chain(window.get_descendents().into_iter())
                .filter(|info| {
                    ExplorerElementKind::from_window_relative_drill_id(
                        &info.drill_id.relative_to(&window.drill_id),
                    )
                    .is_some()
                })
                .collect(),
            _ => {
                // Unknown window, just do selected
                let Some(selected_info) = ui_data.tree.lookup_drill_id(selected_drill_id.clone())
                else {
                    return;
                };
                vec![selected_info]
            }
        };

        push_infos.sort_by_key(|info| {
            let pos = info.bounding_rect.top_left();
            pos.y * 10000 + pos.x
        });

        let new_content = get_content_many(
            push_infos,
            &ui_data.scratch_pad_mode,
            &app_kind,
            &ui_data,
            &screen_access,
        );

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, new_content.as_str());
    }
}

fn handle_append_all_unknown_scratch_pad_events(
    mut inspector_events: EventReader<InspectorScratchPadEvent>,
    mut ui_data: ResMut<UIData>,
    screen_access: ScreensToImageParam,
) {
    for event in inspector_events.read() {
        let InspectorScratchPadEvent::ScratchPadAppendAllUnknown = event else {
            continue;
        };

        // get selected info
        let Some(selected_drill_id) = &ui_data.selected else {
            return;
        };

        // get window
        let Some(window) = ui_data.tree.find_first_child(selected_drill_id) else {
            warn!(
                "Selected drill id not found in tree: {:?}",
                selected_drill_id
            );
            return;
        };

        let app_kind = CursorHeroAppKind::from_window(window);
        let mut push_infos = match app_kind {
            Some(CursorHeroAppKind::Calculator) => window
                .get_descendents()
                .into_iter()
                .filter(|info| CalculatorElementKind::from_info(info).is_none())
                .collect(),
            Some(CursorHeroAppKind::Explorer) => std::iter::once(window)
                .chain(window.get_descendents().into_iter())
                .filter(|info| {
                    ExplorerElementKind::from_window_relative_drill_id(
                        &info.drill_id.relative_to(&window.drill_id),
                    )
                    .is_none()
                })
                .collect(),
            _ => {
                // Unknown window, just do selected
                let Some(selected_info) = ui_data.tree.lookup_drill_id(selected_drill_id.clone())
                else {
                    return;
                };
                vec![selected_info]
            }
        };

        push_infos.sort_by_key(|info| {
            let pos = info.bounding_rect.top_left();
            pos.y * 10000 + pos.x
        });

        let new_content = get_content_many(
            push_infos,
            &ui_data.scratch_pad_mode,
            &app_kind,
            &ui_data,
            &screen_access,
        );

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, new_content.as_str());
    }
}

fn handle_append_all_scratch_pad_events(
    mut inspector_events: EventReader<InspectorScratchPadEvent>,
    mut ui_data: ResMut<UIData>,
    screen_access: ScreensToImageParam,
) {
    for event in inspector_events.read() {
        let InspectorScratchPadEvent::ScratchPadAppendAll = event else {
            continue;
        };

        // get selected info
        let Some(selected_drill_id) = &ui_data.selected else {
            return;
        };

        // get window
        let Some(window) = ui_data.tree.find_first_child(selected_drill_id) else {
            warn!(
                "Selected drill id not found in tree: {:?}",
                selected_drill_id
            );
            return;
        };

        let app_kind = CursorHeroAppKind::from_window(window);
        let mut push_infos = window.get_descendents();

        push_infos.sort_by_key(|info| {
            let pos = info.bounding_rect.top_left();
            pos.y * 10000 + pos.x
        });

        let new_content = get_content_many(
            push_infos,
            &ui_data.scratch_pad_mode,
            &app_kind,
            &ui_data,
            &screen_access,
        );

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, new_content.as_str());
    }
}

fn handle_append_single_info_scratch_pad_events(
    mut inspector_events: EventReader<InspectorScratchPadEvent>,
    mut ui_data: ResMut<UIData>,
    screen_access: ScreensToImageParam,
) {
    for event in inspector_events.read() {
        // Process only append info events
        let InspectorScratchPadEvent::ScratchPadAppendInfo { info } = event else {
            continue;
        };

        // get window
        let Some(window) = ui_data.tree.find_first_child(&info.drill_id) else {
            warn!("Info drill id not found in tree: {:?}", info.drill_id);
            return;
        };

        let content = get_content_many(
            vec![info],
            &ui_data.scratch_pad_mode,
            &CursorHeroAppKind::from_window(window),
            &ui_data,
            &screen_access,
        );

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, content.as_str());
    }
}
