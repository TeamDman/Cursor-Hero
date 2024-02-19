use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct AppWindow;


#[derive(Component, Debug, Reflect)]
pub struct Taskbar;

#[derive(Event, Debug, Reflect)]
pub enum TaskbarEvent {
    Create { screen_id: Entity },
    Open { taskbar_id: Entity },
}