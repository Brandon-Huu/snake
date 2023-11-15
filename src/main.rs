use crate::entities::{Apple, Player};
use crate::systems::{
    apple_collision, despawn_segments, handle_input, move_player, spawn_segments,
};
use bevy::prelude::*;

mod components;
mod entities;
mod systems;

const BOARD_SIZE: f32 = 40.;
const PIXEL_SIZE: f32 = 20.;
//const SNAKE_INITIAL_SIZE: usize = 1;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for KeyCode {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => KeyCode::W,
            Direction::Down => KeyCode::S,
            Direction::Left => KeyCode::A,
            Direction::Right => KeyCode::D,
        }
    }
}

#[derive(Resource)]
pub struct Score(u64);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake by Brandon-Huu".into(),
                resolution: (
                    BOARD_SIZE * PIXEL_SIZE,
                    BOARD_SIZE * PIXEL_SIZE + 200.0,
                )
                    .into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                apple_collision,
                spawn_segments,
                despawn_segments,
            )
                .chain(),
        )
        .insert_resource(Time::<Fixed>::from_seconds(1.))
        .insert_resource(Score(0))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            BOARD_SIZE * (PIXEL_SIZE - 0.5) / 2.,
            -BOARD_SIZE * (PIXEL_SIZE + 0.5) / 2. + 100.,
            0.,
        ),
        ..default()
    });
    commands.spawn(Player::new());
    commands.spawn(Apple::new());
    commands.spawn(/* Border */ SpriteBundle {
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        transform: Transform::from_xyz(-PIXEL_SIZE / 2., -BOARD_SIZE * (PIXEL_SIZE - 0.25), -100.)
            .with_scale(Vec3::splat(BOARD_SIZE * PIXEL_SIZE)),
        ..default()
    });
}
