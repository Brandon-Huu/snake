use crate::components::AppleComponent;
use crate::{BOARD_SIZE, PIXEL_SIZE};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Bundle)]
pub struct Apple {
    a: SpriteBundle,
    b: AppleComponent,
}

impl Apple {
    pub fn new() -> Self {
        let x_coordinate = rand::thread_rng().gen_range(0..PIXEL_SIZE as i32) as f32 * BOARD_SIZE;
        let y_coordinate =
            rand::thread_rng().gen_range((-PIXEL_SIZE as i32 + 1)..=0) as f32 * BOARD_SIZE;

        Apple {
            a: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_xyz(x_coordinate, y_coordinate, 0.)
                    .with_scale(Vec3::splat(PIXEL_SIZE)),
                ..default()
            },
            b: AppleComponent {},
        }
    }
}
