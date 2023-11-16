use crate::constants::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy::sprite::Anchor;

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
    let title_text = Text::from_section(
        PROJECT_TITLE,
        TextStyle {
            font_size: MAIN_MENU_FONT_SIZE,
            ..default()
        },
    );

    let text_transform = Transform::from_xyz(0.,MAIN_MENU_HEIGHT/2.,0.);

    let text_ui = Text2dBundle {
        text: title_text,
        text_anchor: Anchor::TopCenter,
        transform: text_transform,
        ..default()
    };
    commands.spawn(text_ui.clone());

    commands.spawn(text_ui);
    //Spawn controls
    commands.spawn(ButtonBundle::default());
}
