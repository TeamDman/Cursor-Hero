use std::path::PathBuf;

use bevy::prelude::*;
use cursor_hero_cursor_types::cursor_click_types::ClickEvent;
use cursor_hero_cursor_types::cursor_click_types::Way;
use cursor_hero_cursor_types::cursor_types::Cursor;
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_explorer_app_types::prelude::ExplorerStartMenuPanelButton;
use cursor_hero_explorer_app_types::prelude::ExplorerState;
use cursor_hero_explorer_app_types::prelude::ExplorerThemeKind;
use cursor_hero_explorer_app_types::prelude::SpawnExplorerRequestEvent;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanel;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanelAppLauncherIconBuilder;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPanelVisibilityChangeRequestEvent;
use cursor_hero_start_menu_types::start_menu_types::StartMenuPopulateEvent;

pub struct ExplorerStartMenuPlugin;

impl Plugin for ExplorerStartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_start_menu_panels);
        app.add_systems(Update, handle_explorer_app_launcher_icon_clicks);
    }
}

fn populate_start_menu_panels(
    mut commands: Commands,
    mut events: EventReader<StartMenuPopulateEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if let Err(e) = StartMenuPanelAppLauncherIconBuilder::new(ExplorerStartMenuPanelButton)
            .with_name("Explorer")
            .with_texture(asset_server.load("textures/apps/explorer.webp"))
            .with_position(Vec2::new(-120.0, 0.0))
            .spawn(&event.panel, &mut commands)
        {
            error!("Error spawning explorer app launcher icon: {:?}", e);
        }
    }
}

fn handle_explorer_app_launcher_icon_clicks(
    mut start_menu_events: EventWriter<StartMenuPanelVisibilityChangeRequestEvent>,
    mut explorer_events: EventWriter<SpawnExplorerRequestEvent>,
    mut click_events: EventReader<ClickEvent>,
    cursor_query: Query<&TrackedEnvironment, With<Cursor>>,
    icon_query: Query<&Parent, With<ExplorerStartMenuPanelButton>>,
    panel_query: Query<&Parent, With<StartMenuPanel>>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id,
            way: Way::Left,
            ..
        } = event
        else {
            continue;
        };

        let Ok(cursor) = cursor_query.get(*cursor_id) else {
            continue;
        };
        let cursor_environment = cursor;

        let Ok(icon) = icon_query.get(*target_id) else {
            continue;
        };
        let icon_parent = icon;

        let Ok(panel) = panel_query.get(icon_parent.get()) else {
            continue;
        };
        let panel_parent = panel;

        let start_menu_button_id = panel_parent.get();
        start_menu_events.send(StartMenuPanelVisibilityChangeRequestEvent::Close {
            start_menu_button_id,
        });

        let environment_id = cursor_environment.environment_id;
        explorer_events.send(SpawnExplorerRequestEvent {
            environment_id,
            theme: ExplorerThemeKind::WindowsDark,
            state: ExplorerState {
                path: PathBuf::from(file!())
                    .parent()
                    .map(|x| x.to_path_buf())
                    .unwrap_or_default(),
            },
            position: Vec2::new(660.0, -300.0),
        });
    }
}
