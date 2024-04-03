use bevy::prelude::*;
use cursor_hero_host_fs_types::host_fs_types::HostPath;
use cursor_hero_host_fs_types::host_fs_types::HostPathAction;
use cursor_hero_cursor_types::pointer_click_types::ClickEvent;
use cursor_hero_cursor_types::pointer_click_types::Way;
use cursor_hero_toolbelt_types::toolbelt_types::ToolHelpTrigger;

/// Doesn't work with loadouts without the click tool lol
pub fn help_click_listener(
    mut click_events: EventReader<ClickEvent>,
    help_query: Query<&HostPath, With<ToolHelpTrigger>>,
    mut host_fs_events: EventWriter<HostPathAction>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            way: Way::Left,
            ..
        } = event
        else {
            continue;
        };
        let Ok(help) = help_query.get(*target_id) else {
            continue;
        };
        let src_path = help;
        let msg = HostPathAction::OpenWithCode {
            path: src_path.clone(),
        };
        info!("Sending message: {:?}", msg);
        host_fs_events.send(msg);
    }
}
