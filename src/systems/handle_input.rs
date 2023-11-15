use bevy::prelude::*;
use crate::components::SnakeHead;
use crate::Direction;

pub fn handle_input(keys: Res<Input<KeyCode>>, mut query_player: Query<&mut SnakeHead>) {
    let mut snake_head = query_player.iter_mut().next().expect("Could not find SnakeHead component");

    if keys.pressed(Direction::Up.into()) && snake_head.direction != Direction::Down {
        snake_head.direction = Direction::Up;
    } else if keys.pressed(Direction::Down.into()) && snake_head.direction != Direction::Up {
        snake_head.direction = Direction::Down;
    } else if keys.pressed(Direction::Left.into()) && snake_head.direction != Direction::Right {
        snake_head.direction = Direction::Left;
    } else if keys.pressed(Direction::Right.into()) && snake_head.direction != Direction::Left {
        snake_head.direction = Direction::Right;
    }
}
