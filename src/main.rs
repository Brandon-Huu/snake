use bevy::prelude::*;
use snake::main_menu::MainMenuPlugin;

fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        ..default()
    });

    App::new()
        .add_plugins((default_plugins, MainMenuPlugin))
        .run()
}
