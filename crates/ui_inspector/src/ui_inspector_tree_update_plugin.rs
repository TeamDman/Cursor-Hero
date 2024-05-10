use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use cursor_hero_bevy::prelude::Area;
use cursor_hero_ui_hover_types::prelude::GameHoverIndicator;
use cursor_hero_ui_hover_types::prelude::HostHoverIndicator;
use cursor_hero_ui_inspector_types::prelude::ThreadboundUISnapshotMessage;
use cursor_hero_ui_inspector_types::prelude::UIData;
use std::time::Duration;


pub struct UiInspectorTreeUpdatePlugin;

impl Plugin for UiInspectorTreeUpdatePlugin {
    fn build(&self, app: &mut App) {
        let visible_condition = |ui_data: Res<UIData>| ui_data.visible;

        app.add_systems(
            Update,
            trigger_tree_update_for_hovered.run_if(visible_condition),
        );        
    }
}



fn trigger_tree_update_for_hovered(
    mut ui_data: ResMut<UIData>,
    mut cooldown: Local<Option<Timer>>,
    time: Res<Time>,
    mut events: EventWriter<ThreadboundUISnapshotMessage>,
    game_hover_query: Query<&GameHoverIndicator>,
    host_hover_query: Query<&HostHoverIndicator>,
    egui_context_query: Query<&EguiContext, With<PrimaryWindow>>,
) {
    // Do nothing if paused
    if ui_data.paused {
        return;
    }

    // Do nothing when hovering over egui
    if egui_context_query
        .get_single()
        .map(|ctx| ctx.clone().get_mut().is_pointer_over_area())
        .unwrap_or(false)
    {
        return;
    }

    // Get position of cursor
    let pos = match (game_hover_query.get_single(), host_hover_query.get_single()) {
        (Ok(GameHoverIndicator { cursor_pos, .. }), _) => *cursor_pos,
        (_, Ok(HostHoverIndicator { cursor_pos, .. })) => *cursor_pos,
        _ => return,
    };

    // Update selected based on the deepest matching cached element
    ui_data.selected = ui_data
        .ui_tree
        .get_descendents()
        .into_iter()
        .filter(|info| info.bounding_rect.contains(pos))
        .filter(|info| !info.is_stupid_size())
        .min_by_key(|info| info.bounding_rect.size().area())
        .map(|info| info.drill_id.clone());

    // Do nothing if already waiting for a response
    if ui_data.in_flight {
        return;
    }

    // Do nothing if on cooldown
    let default_duration = Duration::from_secs_f32(0.5);
    let Some(cooldown) = cooldown.as_mut() else {
        cooldown.replace(Timer::new(default_duration, TimerMode::Repeating));
        return;
    };
    if cooldown.tick(time.delta()).just_finished() {
        cooldown.reset();
    } else {
        return;
    }

    // Send snapshot request
    events.send(ThreadboundUISnapshotMessage::UIDataUpdate { pos });
    ui_data.in_flight = true;
}
