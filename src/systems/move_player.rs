use bevy::prelude::*;
use crate::components::SnakeHead;
use crate::{Direction, PIXEL_SIZE};


pub fn move_player(
    query_player: Query<&mut SnakeHead>,
    mut query_position: Query<&mut Transform, With<SnakeHead>>,
) {
    let snake_head = query_player.iter().next().unwrap();
    let mut player_position = query_position.iter_mut().next().unwrap();

    match snake_head.direction {
        Direction::Up => player_position.translation.y += PIXEL_SIZE as f32,
        Direction::Down => player_position.translation.y -= PIXEL_SIZE as f32,
        Direction::Left => player_position.translation.x -= PIXEL_SIZE as f32,
        Direction::Right => player_position.translation.x += PIXEL_SIZE as f32,
    };
}
