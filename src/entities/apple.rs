use crate::components::AppleComponent;
use crate::PIXEL_SIZE;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Apple {
    a: SpriteBundle,
    b: AppleComponent,
}

impl Apple {
    pub fn new() -> Self {
        Apple {
            a: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_xyz(40., 40., 0.)
                    .with_scale(Vec3::splat(PIXEL_SIZE as f32)),
                ..default()
            },
            b: AppleComponent {},
        }
    }
}
