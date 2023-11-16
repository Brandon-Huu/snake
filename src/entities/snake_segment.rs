use crate::PIXEL_SIZE;
use bevy::prelude::*;

#[derive(Component)]
pub struct Segment {
    pub lifetime: u128,
}

#[derive(Bundle)]
pub struct SnakeSegment {
    a: SpriteBundle,
    b: Segment,
}

impl SnakeSegment {
    pub fn from(x: f32, y: f32, lifetime: u128) -> Self {
        SnakeSegment {
            a: SpriteBundle {
                sprite: Sprite {
                    color: Color::LIME_GREEN,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -1.).with_scale(Vec3::splat(PIXEL_SIZE)),
                ..default()
            },
            b: Segment { lifetime },
        }
    }
}
