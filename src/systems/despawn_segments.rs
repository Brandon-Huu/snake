#![allow(unused)]
use crate::entities::{Segment, SnakeSegment};
use bevy::prelude::*;

pub fn despawn_segments(
    time: Res<Time>,
    mut commands: Commands,
    segments: Query<(Entity, &Segment), With<Segment>>,
) {
    segments
        .into_iter()
        .filter(|(entity, segment)| segment.lifetime <= time.elapsed().as_secs())
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}
