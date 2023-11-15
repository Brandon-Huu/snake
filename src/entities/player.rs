use bevy::prelude::*;
use crate::components::SnakeHead;
use crate::PIXEL_SIZE;

#[allow(dead_code)]
#[derive(Bundle)]
pub struct Player{
 a: SpriteBundle,
 b: SnakeHead 
}

impl Player{
    pub fn new() -> Self {
        Player {
            a: SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    ..default()
                },
                transform: Transform::from_xyz(0.,0.,0.).with_scale(Vec3::splat(PIXEL_SIZE as f32)),
                ..default()
            },
            b: SnakeHead::new()
        }
    }
}
