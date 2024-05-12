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
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;
        app.add_systems(
            Update,
            handle_append_all_known_scratch_pad_events.run_if(visible_condition),
        );
        app.add_systems(
            Update,
            handle_append_all_scratch_pad_events.run_if(visible_condition),
        );
        app.add_systems(
            Update,
            handle_append_single_info_scratch_pad_events.run_if(visible_condition),
        );
    }
}

fn get_content(
    info: &ElementInfo,
    mode: &ScratchPadMode,
    app_kind: &Option<CursorHeroAppKind>,
    ui_data: &UIData,
    screen_access: &ScreensToImageParam,
) -> String {
    let map_enhanced_thing = |transform: fn(&dyn Reflect) -> Option<String>| match app_kind {
        Some(CursorHeroAppKind::Calculator)
            if let Some(calc_elem_kind) = CalculatorElementKind::from_info(&info) =>
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
    };
    match mode {
        ScratchPadMode::Identify => {
            format!(
                "{},",
                map_enhanced_thing(display_enum_unqualified_variant_definition)
                    .unwrap_or_else(|| info.as_pascal())
            )
        }
        ScratchPadMode::MapIdentify => {
            // info if info.name == "Plus" && info.class_name == "Button" => Some(CalculatorElementKind::PlusButton),
            format!(
                "info if info.name == \"{}\" && info.class_name == \"{}\" => Some({}),",
                info.name,
                info.class_name,
                map_enhanced_thing(display_enum_qualified_variant_instance)
                    .or_else(|| app_kind.as_ref().map(|k| format!("{}::{}", k.element_kind_enum_name(), info.as_pascal())))
                    .unwrap_or_else(|| info.as_pascal())
            )
        }
        ScratchPadMode::PerformDrill => {
            let drill_id = info
                .drill_id
                .as_child()
                .map(|d| d.iter().skip(1).map(|x| x.to_string()).join(", "))
                .unwrap_or_default();

            format!(
                "let {0} = root.drill(&walker, vec![{1}]).context(\"{0}\")?.try_into()?;",
                info.as_identifier(),
                drill_id,
            )
        }
        ScratchPadMode::MapDrill => {
            let drill_id = info
                .drill_id
                .as_child()
                .map(|d| d.iter().skip(1).cloned().collect::<DrillId>())
                .unwrap_or_default();

            format!(
                "{} => {}",
                map_enhanced_thing(display_enum_qualified_variant_instance)
                    .unwrap_or_else(|| info.as_identifier()),
                drill_id
            )
        }
        ScratchPadMode::MapBounds => {
            let compare = ui_data
                .mark
                .as_ref()
                .and_then(|mark_drill_id| ui_data.ui_tree.lookup_drill_id(mark_drill_id.clone()))
                .or_else(|| {
                    ui_data.selected.as_ref().and_then(|selected_drill_id| {
                        ui_data.ui_tree.find_first_child(selected_drill_id)
                    })
                })
                .unwrap_or(&ui_data.ui_tree);

            // Get the bounds of the selected element relative to the comparison element
            let bounds_relative = info
                .bounding_rect
                .translated(&-compare.bounding_rect.top_left());

            // Format as string
            format!(
                "{} => Rect::new({:.1},{:.1},{:.1},{:.1}),",
                map_enhanced_thing(display_enum_qualified_variant_instance)
                    .unwrap_or_else(|| info.as_identifier()),
                bounds_relative.top_left().x as f32,
                -bounds_relative.top_left().y as f32,
                bounds_relative.bottom_right().x as f32,
                -bounds_relative.bottom_right().y as f32
            )
        }
        ScratchPadMode::MapColor => {
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
                "{} => Color::rgb({:.1},{:.1},{:.1}),\n",
                map_enhanced_thing(display_enum_qualified_variant_instance)
                    .unwrap_or_else(|| info.as_identifier()),
                color.r(),
                color.g(),
                color.b()
            )
        }
    }
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
        let Some(window) = ui_data.ui_tree.find_first_child(&selected_drill_id) else {
            warn!(
                "Selected drill id not found in tree: {:?}",
                selected_drill_id
            );
            return;
        };

        let app_kind = CursorHeroAppKind::from_window(&window);
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
                let Some(selected_info) =
                    ui_data.ui_tree.lookup_drill_id(selected_drill_id.clone())
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

        let new_content = push_infos
            .into_iter()
            .map(|info| {
                get_content(
                    &info,
                    &ui_data.scratch_pad_mode,
                    &app_kind,
                    &ui_data,
                    &screen_access,
                )
            })
            .unique()
            .join("\n");

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
        let Some(window) = ui_data.ui_tree.find_first_child(&selected_drill_id) else {
            warn!(
                "Selected drill id not found in tree: {:?}",
                selected_drill_id
            );
            return;
        };

        let app_kind = CursorHeroAppKind::from_window(&window);
        let mut push_infos = window.get_descendents();

        push_infos.sort_by_key(|info| {
            let pos = info.bounding_rect.top_left();
            pos.y * 10000 + pos.x
        });

        let new_content = push_infos
            .into_iter()
            .map(|info| {
                get_content(
                    &info,
                    &ui_data.scratch_pad_mode,
                    &app_kind,
                    &ui_data,
                    &screen_access,
                )
            })
            .unique()
            .join("\n");

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
        let Some(window) = ui_data.ui_tree.find_first_child(&info.drill_id) else {
            warn!("Info drill id not found in tree: {:?}", info.drill_id);
            return;
        };

        let content = format!(
            "{}\n",
            get_content(
                &info,
                &ui_data.scratch_pad_mode,
                &CursorHeroAppKind::from_window(&window),
                &ui_data,
                &screen_access,
            )
        );

        // append to scratch pad
        // make new rows show at the top by adding to the front
        ui_data.scratch_pad.insert_str(0, content.as_str());
    }
}
