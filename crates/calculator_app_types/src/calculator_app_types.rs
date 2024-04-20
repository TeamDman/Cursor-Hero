use bevy::prelude::*;


#[derive(Event, Debug, Reflect)]
pub struct SpawnCalculatorRequestEvent {
    pub environment_id: Entity,
    pub expression: String,
    pub display: String,
}

#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct Calculator;
#[derive(Component, Debug, Reflect, Default, Clone, PartialEq)]
pub struct CalculatorStartMenuPanelButton;

#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorDisplay;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorExpression;
#[derive(Component, Debug, Reflect, Default)]
pub struct CalculatorButton;
