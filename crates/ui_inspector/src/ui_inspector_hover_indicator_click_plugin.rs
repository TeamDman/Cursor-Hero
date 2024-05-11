use bevy::prelude::*;
use cursor_hero_bevy::prelude::NegativeYIVec2;
use cursor_hero_cursor_types::prelude::ClickEvent;
use cursor_hero_cursor_types::prelude::Way;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
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
    host_hover_query: Query<&HostHoverIndicator>,
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
        if way == &Way::Left && ui_data.visible {
            // If the click event targets a hover indicator
            if game_hover_query.get(*target_id).is_ok() || host_hover_query.get(*target_id).is_ok()
            {
                // Toggle the paused state
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right && ui_data.visible {
            inspector_events.send(InspectorScratchPadEvent::ScratchPadAppendSelected);
        } else if way == &Way::Middle {
            info!("Sending click event!");
            threadbound_events.send(ThreadboundUISnapshotMessage::ClickPos {
                pos: end_position.to_owned().neg_y(),
                way: Way::Left,
            });
        }
    }
}
