use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct StartMenuButton;



#[derive(Component, Debug, Reflect)]
pub struct StartMenu;

#[derive(Event, Debug, Reflect)]
pub enum StartMenuEvent {
    Open { start_menu_button_id: Entity },
    Close { start_menu_button_id: Entity },
}
