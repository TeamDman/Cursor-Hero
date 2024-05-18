use bevy::prelude::*;
use cursor_hero_bevy::prelude::NegativeYIVec2;
use cursor_hero_cursor_types::prelude::ClickEvent;
use cursor_hero_cursor_types::prelude::Way;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::InspectorScratchPadEvent;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorHoverIndicatorClickPlugin;

impl Plugin for UiInspectorHoverIndicatorClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hovered_click_listener);
    }
}

fn hovered_click_listener(
    mut click_events: EventReader<ClickEvent>,
    game_hover_query: Query<&GameHoverIndicator>,
    mut ui_data: ResMut<UIData>,
    mut inspector_events: EventWriter<InspectorScratchPadEvent>,
    mut threadbound_events: EventWriter<ThreadboundUISnapshotMessage>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            way,
            end_position,
            ..
        } = event
        else {
            continue;
        };
        if way == &Way::Left && ui_data.windows.global_toggle {
            // If the click event targets a hover indicator
            if game_hover_query.contains(*target_id) {
                // Toggle the paused state
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right && ui_data.windows.global_toggle {
            // Get the hover indicator
            let Ok(game_hover) = game_hover_query.get(*target_id) else {
                continue;
            };
            // The hover indicator has an unknown DrillID.
            // We can populate it by assuming it matches the selected id.
            let mut info = game_hover.info.clone();
            if let Some(selected) = &ui_data.selected {
                info.drill_id = selected.clone();
            }
            // Append the info to the scratch pad
            inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendInfo { info });
        } else if way == &Way::Middle {
            info!("Sending click event!");
            threadbound_events.send(ThreadboundUISnapshotMessage::ClickPos {
                pos: end_position.to_owned().neg_y(),
                way: Way::Left,
            });
        }
    }
}
