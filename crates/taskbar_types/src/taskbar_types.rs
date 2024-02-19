use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct AppWindow;


#[derive(Component, Debug, Reflect)]
pub struct Taskbar;

#[derive(Event, Debug, Reflect)]
pub enum TaskbarEvent {
    Populate { taskbar_id: Entity },
}