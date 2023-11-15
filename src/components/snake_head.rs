use bevy::prelude::*;
use crate::Direction;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction
}
impl SnakeHead {
    pub fn new() -> Self { SnakeHead { direction : Direction::Up } }
}
