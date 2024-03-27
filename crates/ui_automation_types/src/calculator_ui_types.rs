use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Reflect, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Calculator {
    pub memory: CalculatorMemory,
}
impl std::fmt::Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Calculator (value={})", self.memory.buffer)
    }
}

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
    calculator: Calculator,
}
#[derive(Event, Debug, Reflect)]
pub struct CalculatorRequestDefaultPositioningRequest {
    calculator: Entity,
}
