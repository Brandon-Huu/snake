use crate::constants::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuCamera;

#[derive(Component)]
struct HighScoreText;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct HistoryButton;

#[derive(Component)]
struct RestartButton;

#[derive(Component)]
struct BoardSizeInput;

fn spawn_main_menu(mut commands: Commands) {
    let window = commands
        .spawn((
            Window {
                title: "Main Menu | Snake by Brandon-Huu".to_string(),
                resolution: MAIN_MENU_RESOLUTION.into(),
                resizable: false,
                ..default()
            },
            MainMenu {},
        ))
        .id();

    let camera = Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(window)),
            ..default()
        },
        ..default()
    };

    commands.spawn((camera, MainMenuCamera));

    //Title text
    commands.spawn(TextBundle::from_section(
        PROJECT_TITLE,
        TextStyle {
            font_size: MAIN_MENU_FONT_SIZE,
            ..default()
        },
    ));

    //Spawn controls
    commands.spawn(ButtonBundle::default());
}
