use bevy::prelude::*;
use cursor_hero_ui_inspector_types::prelude::FetchingState;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;

pub struct UiInspectorChildrenFetcherPlugin;

impl Plugin for UiInspectorChildrenFetcherPlugin {
    fn build(&self, app: &mut App) {
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;
        
        app.add_systems(
            Update,
            trigger_gather_children_request.run_if(visible_condition),
        );
    }
}

fn trigger_gather_children_request(
    mut data: ResMut<UIData>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
) {
    for (key, state) in data.fetching.iter_mut() {
        let FetchingState::RequestingFetch = state else {
            continue;
        };
        *state = FetchingState::FetchDispatched;
        events.send(ThreadboundUISnapshotMessage::GatherChildren {
            parent_drill_id: key.0.clone(),
            parent_runtime_id: key.1.clone(),
        });
    }
}