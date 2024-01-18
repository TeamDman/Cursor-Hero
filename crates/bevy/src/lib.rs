use bevy::ecs::entity::Entity;
use bevy::prelude::Name;

// Define a trait that provides a method to return a string from an Option<&Name>
pub trait NameOrEntityDisplay {
    fn name_or_entity(&self, entity: Entity) -> String;
}

// Implement the trait for Option<&Name>
impl NameOrEntityDisplay for Option<&Name> {
    fn name_or_entity(&self, entity: Entity) -> String {
        match self {
            Some(name) => name.to_string(),
            None => format!("Entity({:?})", entity),
        }
    }
}