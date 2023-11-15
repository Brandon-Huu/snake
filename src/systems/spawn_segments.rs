use crate::components::SnakeHead;
use crate::entities::SnakeSegment;
use crate::Score;
use bevy::prelude::*;

pub fn spawn_segments(
    time: Res<Time>,
    score: Res<Score>,
    mut commands: Commands,
    query_position: Query<&Transform, With<SnakeHead>>,
) {
    let player_position = query_position.iter().next().unwrap();

    let snake = SnakeSegment::from(
        player_position.translation.x,
        player_position.translation.y,
        time.elapsed().as_secs() + score.0 + 1,
    );
    commands.spawn(snake);
}
