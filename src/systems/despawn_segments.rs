#![allow(unused)]
use crate::entities::{Segment, SnakeSegment};
use crate::Skip;
use bevy::prelude::*;

pub fn despawn_segments(
    mut skip: ResMut<Skip>,
    time: Res<Time>,
    mut commands: Commands,
    segments: Query<(Entity, &Segment), With<Segment>>,
) {
    if skip.0 {
        skip.0 = false;
        return;
    }
    segments
        .into_iter()
        .filter(|(entity, segment)| segment.lifetime <= time.elapsed().as_secs())
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}
