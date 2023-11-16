#![allow(unused)]
use crate::entities::{Segment, SnakeSegment};
use bevy::prelude::*;

#[derive(Event)]
pub struct GameOverEvent {}

pub fn despawn_segments(
    time: Res<Time>,
    mut commands: Commands,
    segments: Query<(Entity, &Segment), With<Segment>>,
) {
    segments
        .into_iter()
        .filter(|(entity, segment)| segment.lifetime <= time.elapsed().as_millis())
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

pub fn despawn_all(
    mut event_game_over: EventReader<GameOverEvent>,
    mut commands: Commands,
    segments: Query<Entity, With<Segment>>,
) {
    for _ in event_game_over.read() {
        segments
            .into_iter()
            .for_each(|entity| commands.entity(entity).despawn());
    }
}
