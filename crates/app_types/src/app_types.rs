use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct MyComponent;

#[derive(Event, Debug, Reflect)]
pub enum MyEvent {
    Guh,
    Uh,
}
