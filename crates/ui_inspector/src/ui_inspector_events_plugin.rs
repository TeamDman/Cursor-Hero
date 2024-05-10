use bevy::prelude::*;
use bevy::utils::HashMap;
use cursor_hero_bevy::prelude::BottomRightI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
use cursor_hero_calculator_app_types::calculator_app_types::CalculatorElementKind;
use cursor_hero_cursor_types::prelude::Way;
use cursor_hero_explorer_app_types::prelude::ExplorerElementKind;
use cursor_hero_screen::get_image::AsBevyColor;
use cursor_hero_screen::get_image::ImageHolder;
use cursor_hero_screen::get_image::ScreensToImageParam;
use cursor_hero_ui_automation::prelude::*;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::ScratchPadMode;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use itertools::Itertools;

pub struct UiInspectorEventsPlugin;

impl Plugin for UiInspectorEventsPlugin {
    fn build(&self, app: &mut App) {
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;
        app.add_systems(Update, handle_inspector_events.run_if(visible_condition));
    }
}

fn handle_inspector_events(
    mut inspector_events: EventReader<InspectorEvent>,
    mut ui_data: ResMut<UIData>,
    mut threadbound_events: EventWriter<ThreadboundUISnapshotMessage>,
    screen_access: ScreensToImageParam,
) {
    for event in inspector_events.read() {
        // get selected info
        let Some(selected_drill_id) = ui_data.selected.clone() else {
            return;
        };

        // get window
        let window = selected_drill_id
            .as_child()
            .map(|inner| inner.iter().take(1).cloned().collect())
            .and_then(|window_drill_id| ui_data.ui_tree.lookup_drill_id(window_drill_id))
            .unwrap_or(&ui_data.ui_tree);

        let push_infos = match event {
            InspectorEvent::PushSelectedToScratchPad => {
                let Some(selected_info) =
                    ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
                else {
                    return;
                };

                vec![(selected_info, selected_info.as_identifier())]
            }
            InspectorEvent::ClickSelected => {
                if let Some(selected) = ui_data.selected.clone() {
                    threadbound_events.send(ThreadboundUISnapshotMessage::Click {
                        drill_id: selected,
                        way: Way::Left,
                    });
                }
                return;
            }
            InspectorEvent::PushKnownToScratchPad => {
                if CalculatorElementKind::top_level_info_matches_window_kind(&window) {
                    window
                        .get_descendents()
                        .into_iter()
                        .filter_map(|info| {
                            CalculatorElementKind::from_info(info)
                                .map(|x| (info, x.get_qualified_name()))
                        })
                        .collect()
                } else if window.class_name == "CabinetWClass" {
                    std::iter::once(window)
                        .chain(window.get_descendents().into_iter())
                        .filter_map(|info| {
                            ExplorerElementKind::from_window_relative_drill_id(
                                &info.drill_id.relative_to(&window.drill_id),
                            )
                            .map(|x| (info, x.get_name()))
                        })
                        .sorted_by_key(|e| {
                            let pos = e.0.bounding_rect.top_left();
                            pos.y * 10000 + pos.x
                        })
                        .collect()
                } else {
                    // Unknown window, just do selected
                    let Some(selected_info) =
                        ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
                    else {
                        return;
                    };

                    vec![(selected_info, selected_info.as_identifier())]
                }
            }
        };
        let mut content = String::new();
        for (push_info, mut identifier) in push_infos {
            content.push_str(
                match ui_data.scratch_pad_mode {
                    ScratchPadMode::Drill => {
                        // get drill id
                        let drill_id = push_info
                            .drill_id
                            .as_child()
                            .map(|d| d.iter().skip(1).map(|x| x.to_string()).join(", "))
                            .unwrap_or_default();

                        // build content
                        let content = format!(
                        "let {0} = root.drill(&walker, vec![{1}]).context(\"{0}\")?.try_into()?;\n",
                        identifier, drill_id
                    );

                        content
                    }
                    ScratchPadMode::Bounds => {
                        // Use mark if present, window otherwise
                        let compare_drill_id = ui_data
                            .mark
                            .clone()
                            .or_else(|| {
                                selected_drill_id
                                    .as_child()
                                    .map(|inner| inner.iter().take(1).cloned().collect())
                            })
                            .unwrap_or(DrillId::Root);

                        // Look up the comparison element
                        let compare = ui_data
                            .ui_tree
                            .lookup_drill_id(compare_drill_id)
                            .unwrap_or(&ui_data.ui_tree);

                        // Get the bounds of the selected element relative to the comparison element
                        let bounds_relative = push_info
                            .bounding_rect
                            .translated(&-compare.bounding_rect.top_left());

                        // Use known identifiers based on window

                        if CalculatorElementKind::top_level_info_matches_window_kind(&window)
                            && let Some(calc_elem_kind) =
                                CalculatorElementKind::from_identifier(&identifier)
                        {
                            identifier = calc_elem_kind.get_qualified_name();
                        } else if window.class_name == "CabinetWClass"
                            && let Some(explorer_elem_kind) =
                                ExplorerElementKind::from_window_relative_drill_id(
                                    &push_info.drill_id,
                                )
                        {
                            identifier = explorer_elem_kind.get_name();
                        }

                        // Format as string
                        format!(
                            "{} => Rect::new({:.1},{:.1},{:.1},{:.1}),\n",
                            identifier,
                            bounds_relative.top_left().x as f32,
                            -bounds_relative.top_left().y as f32,
                            bounds_relative.bottom_right().x as f32,
                            -bounds_relative.bottom_right().y as f32
                        )
                    }
                    ScratchPadMode::Color => {
                        let image = screen_access.get_image_buffer(push_info.bounding_rect);
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
                                    push_info.bounding_rect
                                );
                                Color::BLACK
                            }
                        };
                        format!(
                            "{} => Color::rgb({:.1},{:.1},{:.1}),\n",
                            identifier,
                            color.r(),
                            color.g(),
                            color.b()
                        )
                    }
                }
                .as_str(),
            );
        }

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, content.as_str());
    }
}
