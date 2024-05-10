use bevy::prelude::*;
use cursor_hero_cursor_types::prelude::ClickEvent;
use cursor_hero_cursor_types::prelude::Way;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::InspectorEvent;
use cursor_hero_ui_inspector_types::prelude::UIData;


pub struct UiInspectorHoverIndicatorClickPlugin;

impl Plugin for UiInspectorHoverIndicatorClickPlugin {
    fn build(&self, app: &mut App) {
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;

        app.add_systems(Update, hovered_click_listener.run_if(visible_condition));
        
    }
}

fn hovered_click_listener(
    mut click_events: EventReader<ClickEvent>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
    mut ui_data: ResMut<UIData>,
    mut inspector_events: EventWriter<InspectorEvent>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id: _,
            way,
        } = event
        else {
            continue;
        };
        if way == &Way::Left {
            // If the click event targets a hover indicator
            if game_hover_query.get(*target_id).is_ok() || host_hover_query.get(*target_id).is_ok()
            {
                // Toggle the paused state
                ui_data.paused ^= true;
                info!("Hover indicator clicked, paused set to {}", ui_data.paused);
            }
        } else if way == &Way::Right {
            inspector_events.send(InspectorEvent::PushSelectedToScratchPad);
        } else if way == &Way::Middle {
            info!("Sending click event!");
            inspector_events.send(InspectorEvent::ClickSelected);
        }
    }
}