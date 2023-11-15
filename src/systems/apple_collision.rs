use crate::components::{AppleComponent, SnakeHead};
use crate::entities::Apple;
use crate::Score;
use bevy::prelude::*;

pub fn apple_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    apple: Query<(Entity, &Transform), With<AppleComponent>>,
    query_position: Query<&Transform, With<SnakeHead>>,
) {
    let player_position = query_position.single();
    let apple = apple.single();

    info!("{:?}", &player_position);
    info!("{:?}", &apple.1);
    if player_position != apple.1 {
        return;
    }

    commands.entity(apple.0).despawn();
    commands.spawn(Apple::new());

    score.0 += 1;
}
