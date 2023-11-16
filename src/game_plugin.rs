use bevy::prelude::*;
use crate::constants::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ();)
    }
}
