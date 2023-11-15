use crate::Direction;
use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
    pub direction_next: Direction,
}

impl SnakeHead {
    pub fn new() -> Self {
        SnakeHead {
            direction: Direction::Up,
            direction_next: Direction::Up,
        }
    }
}
