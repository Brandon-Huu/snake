use crate::components::SnakeHead;
use crate::{Direction, PIXEL_SIZE};
use bevy::prelude::*;

pub fn move_player(
    mut query_player: Query<&mut SnakeHead>,
    mut query_position: Query<&mut Transform, With<SnakeHead>>,
) {
    let mut snake_head = query_player.single_mut();
    let mut player_position = query_position.single_mut();

    match snake_head.direction_next {
        Direction::Up => player_position.translation.y += PIXEL_SIZE,
        Direction::Down => player_position.translation.y -= PIXEL_SIZE,
        Direction::Left => player_position.translation.x -= PIXEL_SIZE,
        Direction::Right => player_position.translation.x += PIXEL_SIZE,
    };

    snake_head.direction = snake_head.direction_next;
    //info!("{:?}", &player_position);
}
