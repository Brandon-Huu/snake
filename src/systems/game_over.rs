use crate::components::{AppleComponent, SnakeHead};
use crate::entities::{Player, Apple};
use crate::systems::GameOverEvent;

use crate::{Score, BOARD_SIZE, PIXEL_SIZE};
use bevy::prelude::*;

pub fn game_over(
    mut commands: Commands,
    query_player: Query<(Entity, &Transform), With<SnakeHead>>,
    mut score: ResMut<Score>,
    query_apple: Query<(Entity, &Transform), With<AppleComponent>>,
    mut game_over_event: EventWriter<GameOverEvent>
) {
    let player = query_player.single();
    let player_position = player.1;

    if player_position.translation.x >= 0.
        && player_position.translation.x < BOARD_SIZE * PIXEL_SIZE
    && player_position.translation.y > -(BOARD_SIZE * PIXEL_SIZE)
        && player_position.translation.y <= 0.
    {
        return;
    }

    let apple = query_apple.single();

    commands.entity(apple.0).despawn();
    commands.entity(player.0).despawn();

    commands.spawn(Apple::new());
    commands.spawn(Player::new());

    score.0 = 0;
    game_over_event.send(GameOverEvent{});
}
