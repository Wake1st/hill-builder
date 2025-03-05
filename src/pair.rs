use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Pair {
    pub ground: Entity,
    pub water: Entity,
}

impl Default for Pair {
    fn default() -> Self {
        Self {
            ground: Entity::PLACEHOLDER,
            water: Entity::PLACEHOLDER,
        }
    }
}
