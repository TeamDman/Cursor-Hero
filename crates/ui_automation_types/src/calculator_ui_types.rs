use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatorState {
    pub expression: String,
    pub display: String,
}
impl std::fmt::Display for CalculatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calculator ({}{})", self.expression, self.display)
    }
}
#[derive(Component, Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Calculator;

#[derive(Component, Debug, Reflect, Default)]
pub struct NumberDisplayPanel;

#[derive(Component, Debug, Reflect, Default)]
pub struct DigitInputButton(u8);

#[derive(Component, Debug, Reflect, Default)]
pub struct EqualsButton;

#[derive(Component, Debug, Reflect, Default)]
pub struct PlusButton;

#[derive(Component, Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculatorMemory {
    pub buffer: f64,
}

#[derive(Event, Debug, Reflect, Default)]
pub struct SpawnCalculatorRequest {
    calculator: CalculatorState,
}
#[derive(Event, Debug, Reflect)]
pub struct CalculatorRequestDefaultPositioningRequest {
    calculator: Entity,
}
