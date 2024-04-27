use bevy::prelude::*;
use cursor_hero_calculator_app_types::calculator_app_types::{CalculatorStartMenuPanelButton, CalculatorState, CalculatorThemeKind, SpawnCalculatorRequestEvent};
use cursor_hero_cursor_types::{cursor_click_types::{ClickEvent, Way}, cursor_types::Cursor};
use cursor_hero_environment_types::environment_types::TrackedEnvironment;
use cursor_hero_start_menu_types::start_menu_types::{StartMenuPanel, StartMenuPanelAppLauncherIconBuilder, StartMenuPanelVisibilityChangeRequestEvent, StartMenuPopulateEvent};

pub struct CalculatorStartMenuPlugin;

impl Plugin for CalculatorStartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_start_menu_panels);
        app.add_systems(Update, handle_calculator_app_launcher_icon_clicks);

    }
}

fn populate_start_menu_panels(
    mut commands: Commands,
    mut events: EventReader<StartMenuPopulateEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        if let Err(e) = StartMenuPanelAppLauncherIconBuilder::new(CalculatorStartMenuPanelButton)
            .with_name("Calculator")
            .with_texture(asset_server.load("textures/apps/calculator.png"))
            .with_position(Vec2::new(120.0, 0.0))
            .spawn(&event.panel, &mut commands)
        {
            error!("Error spawning calculator app launcher icon: {:?}", e);
        }
    }
}

fn handle_calculator_app_launcher_icon_clicks(
    mut start_menu_events: EventWriter<StartMenuPanelVisibilityChangeRequestEvent>,
    mut calculator_events: EventWriter<SpawnCalculatorRequestEvent>,
    mut click_events: EventReader<ClickEvent>,
    cursor_query: Query<&TrackedEnvironment, With<Cursor>>,
    icon_query: Query<&Parent, With<CalculatorStartMenuPanelButton>>,
    panel_query: Query<&Parent, With<StartMenuPanel>>,
) {
    for event in click_events.read() {
        let ClickEvent::Clicked {
            target_id,
            cursor_id,
            way: Way::Left,
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
        calculator_events.send(SpawnCalculatorRequestEvent {
            environment_id,
            theme: CalculatorThemeKind::WindowsDark,
            state: CalculatorState {
                expression: "".to_string(),
                value: "0".to_string(),
            },
            position: Vec2::new(300.0,-300.0),
        });
    }
}
