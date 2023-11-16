use crate::components::{AppleComponent, SnakeHead};
use crate::entities::Apple;
use crate::systems::AddPointEvent;
use bevy::prelude::*;

pub fn apple_collision(
    mut commands: Commands,
    apple: Query<(Entity, &Transform), With<AppleComponent>>,
    query_position: Query<&Transform, With<SnakeHead>>,
    mut event_add_point: EventWriter<AddPointEvent>,
) {
    let player_position = query_position.single();
    let apple = apple.single();

    if player_position != apple.1 {
        return;
    }

    commands.entity(apple.0).despawn();
    commands.spawn(Apple::new());

    event_add_point.send(AddPointEvent(1));
}
