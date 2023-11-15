use bevy::prelude::*;
use crate::entities::Player;
use crate::systems::{handle_input, move_player};

mod entities;
mod components;
mod systems;

const BOARD_SIZE: usize = 40;
const PIXEL_SIZE: usize = 20;
//const SNAKE_INITIAL_SIZE: usize = 1;


#[derive(PartialEq)]
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
        .add_systems(FixedUpdate, move_player)
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Player::new());
    // Spawn first apple
}
