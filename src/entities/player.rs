use crate::components::SnakeHead;
use crate::{BOARD_SIZE, PIXEL_SIZE};
use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Bundle)]
pub struct Player {
    a: SpriteBundle,
    b: SnakeHead,
}

impl Player {
    pub fn new() -> Self {
        Player {
            a: SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    ..default()
                },
                transform: Transform::from_xyz(
                    (PIXEL_SIZE / 2.) * BOARD_SIZE,
                    -BOARD_SIZE * PIXEL_SIZE / 2.,
                    0.,
                )
                .with_scale(Vec3::splat(PIXEL_SIZE)),
                ..default()
            },
            b: SnakeHead::new(),
        }
    }
}
