use crate::components::SnakeHead;
use crate::entities::SnakeSegment;
use crate::{TICKS_PER_SECOND, Score};
use bevy::prelude::*;

pub fn spawn_segments(
    time: Res<Time>,
    score: Res<Score>,
    mut commands: Commands,
    query_position: Query<&Transform, With<SnakeHead>>,
) {
    let player_position = query_position.single();

    let snake = SnakeSegment::from(
        player_position.translation.x,
        player_position.translation.y,
        time.elapsed().as_millis() + ( (score.0 + 1) as u128 * 1000/ TICKS_PER_SECOND as u128),
    );
    commands.spawn(snake);
}
