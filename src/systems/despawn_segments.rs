#![allow(unused)]
use crate::entities::{Segment, SnakeSegment};
use bevy::prelude::*;

#[derive(Event)]
pub struct GameOverEvent{}

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

fn despawn_all(
    mut commands: Commands,
    segments: Query<Entity, With<Segment>>,
    ){

    segments
        .into_iter()
        .for_each(|entity| commands.entity(entity).despawn());
}
