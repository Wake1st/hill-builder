use bevy::prelude::*;

#[derive(Component)]
pub struct Neighborhood {
    pub left_neighbor: Entity,
    pub right_neighbor: Entity,
    pub front_neighbor: Entity,
    pub back_neighbor: Entity,
}

impl Neighborhood {
    pub fn get_neighbors(&self) -> Vec<Entity> {
        vec![
            self.left_neighbor,
            self.right_neighbor,
            self.front_neighbor,
            self.back_neighbor,
        ]
    }
}