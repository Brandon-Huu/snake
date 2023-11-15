use crate::entities::{Apple, Player};
use crate::systems::{
    apple_collision, despawn_segments, handle_input, move_player, spawn_segments,
};
use bevy::prelude::*;

mod components;
mod entities;
mod systems;

const BOARD_SIZE: usize = 40;
const PIXEL_SIZE: usize = 20;
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

#[derive(Resource)]
pub struct Skip(bool);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake by Brandon-Huu".into(),
                resolution: (
                    BOARD_SIZE as f32 * PIXEL_SIZE as f32,
                    BOARD_SIZE as f32 * PIXEL_SIZE as f32 + 200.0,
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
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(1.))
        .insert_resource(Score(0))
        .insert_resource(Skip(false))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Player::new());
    commands.spawn(Apple::new());
}
